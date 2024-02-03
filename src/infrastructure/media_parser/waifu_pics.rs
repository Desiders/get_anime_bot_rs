use crate::{
    application::media_parser::{exceptions::MediaGetException, traits::Source},
    domain::media_parser::{
        entities::{
            genre::{vec_new_nsfw_gif, vec_new_nsfw_image, vec_new_sfw_gif, vec_new_sfw_image},
            Genre, Genres, Media,
        },
        value_objects::age_restriction,
    },
};

use age_restriction::AgeRestriction;
use async_trait::async_trait;
use lazy_static::lazy_static;
use reqwest::multipart::Form;
use serde::Deserialize;
use std::borrow::Cow;
use tracing::{event, instrument, Level};

#[derive(Debug, Clone)]
pub struct WaifuPics<Client = reqwest::Client> {
    url: Cow<'static, str>,
    exclude_urls: Vec<Cow<'static, str>>,
    client: Client,
}

impl<Client> WaifuPics<Client> {
    /// Create a new instance of [`WaifuPics`].
    /// # Arguments
    /// * `client` - The client to use for the requests.
    /// # Note
    /// By default, the url is set as `https://api.waifu.pics`.
    /// If you want to change it, use the `WaifuPics::with_url` method.
    pub fn new(client: Client) -> WaifuPics<Client> {
        Self {
            client,
            exclude_urls: Vec::new(),
            url: "https://api.waifu.pics".into(),
        }
    }

    /// Set the exclude url with a mutable reference
    pub fn exclude_url(&mut self, exclude_url: impl Into<Cow<'static, str>>) {
        self.exclude_urls.push(exclude_url.into());
    }
}

impl Default for WaifuPics {
    fn default() -> Self {
        Self::new(reqwest::Client::default())
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    files: Vec<String>,
}

#[async_trait]
impl Source for WaifuPics<reqwest::Client> {
    fn name(&self) -> &str {
        "api.waifu.pics"
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn genres(&self) -> &Genres {
        lazy_static! {
            static ref GENRES: Genres = Genres::new(
                [
                    vec_new_sfw_gif![
                        "bully", "cuddle", "cry", "hug", "kiss", "lick", "pat", "smug", "bonk",
                        "yeet", "blush", "smile", "wave", "nom", "bite", "glomp", "slap", "kill",
                        "kick", "happy", "wink", "poke", "dance", "cringe",
                    ],
                    vec_new_sfw_image!["waifu", "neko", "shinobu", "megumin", "awoo",],
                    vec_new_nsfw_gif!["blowjob"],
                    vec_new_nsfw_image!["waifu", "neko", "trap"],
                ]
                .concat()
            );
        }

        &GENRES
    }

    #[instrument(skip(self))]
    async fn get_media_list_by_genre(
        &self,
        genre: &Genre,
    ) -> Result<Vec<Media>, MediaGetException> {
        let age_restriction = match genre.age_restriction() {
            AgeRestriction::Sfw => "sfw",
            AgeRestriction::Nsfw => "nsfw",
            AgeRestriction::Unknown => {
                return Err(MediaGetException::new(
                    genre.clone(),
                    "only SFW/NSFW restrictions are valid",
                ));
            }
        };
        let url = format!(
            "{api_url}/many/{age_restriction}/{genre}",
            api_url = self.url,
            genre = genre.name()
        );

        let exclude_urls = serde_json::to_string(&self.exclude_urls)
            .map_err(|err| MediaGetException::new(genre.clone(), err.to_string()))?;
        let form = Form::new().text("exclude", exclude_urls);

        let content = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await
            .map_err(|err| MediaGetException::new(genre.clone(), err.to_string()))?
            .text()
            .await
            .map_err(|err| MediaGetException::new(genre.clone(), err.to_string()))?;

        let api_response: ApiResponse = match serde_json::from_str(&content) {
            Ok(api_response) => api_response,
            Err(err) => {
                event!(Level::ERROR, %err, "Failed to parse response");

                return Err(MediaGetException::new(genre.clone(), err.to_string()));
            }
        };

        let mut list = Vec::with_capacity(api_response.files.len());

        for file in api_response.files {
            list.push(Media::new(file, genre.clone()));
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_media_list_by_genre() {
        let waifu_pics = WaifuPics::new(reqwest::Client::new());

        let genres = waifu_pics.genres();

        for genre in genres.iter() {
            let media_list = match waifu_pics.get_media_list_by_genre(genre).await {
                Ok(media_list) => media_list,
                Err(err) => {
                    panic!("Failed to get media list with genre `{genre:?}`, error: {err}");
                }
            };

            assert!(!media_list.is_empty());
        }
    }
}
