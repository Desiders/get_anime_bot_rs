use crate::{
    application::media_parser::{exceptions::MediaGetException, traits::Source},
    domain::media_parser::entities::{
        genre::{vec_new_nsfw_gif, vec_new_nsfw_image, vec_new_sfw_gif, vec_new_sfw_image},
        Genre, Genres, Media,
    },
};

use async_trait::async_trait;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::borrow::Cow;
use tracing::{event, instrument, Level};

#[derive(Debug, Clone)]
pub struct NekosFun<Client = reqwest::Client> {
    url: Cow<'static, str>,
    client: Client,
}

impl<Client> NekosFun<Client> {
    /// Create a new instance of [`NekosFun`].
    /// # Arguments
    /// * `client` - The client to use for the requests.
    /// # Note
    /// By default, the url is set as `http://api.nekos.fun:8080/api`.
    /// If you want to change it, use the `NekosFun::with_url` method.
    pub fn new(client: Client) -> NekosFun<Client> {
        Self {
            client,
            url: "http://api.nekos.fun:8080/api".into(),
        }
    }
}

impl Default for NekosFun {
    fn default() -> Self {
        Self::new(reqwest::Client::default())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    image: String,
}

#[async_trait]
impl Source for NekosFun<reqwest::Client> {
    fn name(&self) -> &str {
        "api.nekos.fun"
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn genres(&self) -> &Genres {
        lazy_static! {
            static ref GENRES: Genres = Genres::new(
                [
                    vec_new_sfw_gif![
                        "kiss", "lick", "hug", "baka", "cry", "poke", "smug", "slap", "tickle",
                        "pat", "laugh", "feed", "cuddle",
                    ],
                    vec_new_sfw_image!["animalears", "foxgirl", "neko"],
                    vec_new_nsfw_gif!["boobs", "cum", "lesbian", "anal"],
                    vec_new_nsfw_image!["hentai", "lewd", "holo"],
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
        let url = format!(
            "{api_url}/{genre}",
            api_url = self.url,
            genre = genre.name()
        );

        let content = self
            .client
            .get(&url)
            .send()
            .await
            .map_err(|err| {
                event!(Level::ERROR, %err, "Failed to send request");

                MediaGetException::new(genre.clone(), err.to_string())
            })?
            .text()
            .await
            .map_err(|err| {
                event!(Level::ERROR, %err, "Failed to get response");

                MediaGetException::new(genre.clone(), err.to_string())
            })?;

        let api_response: ApiResponse = match serde_json::from_str(&content) {
            Ok(api_response) => api_response,
            Err(err) => {
                event!(Level::ERROR, content, %err, "Failed to parse response");

                return Err(MediaGetException::new(genre.clone(), err.to_string()));
            }
        };

        let media = Media::new(api_response.image, genre.clone());

        Ok(vec![media])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_media_list_by_genre() {
        let nekos_fun = NekosFun::new(reqwest::Client::new());

        let genres = nekos_fun.genres();

        for genre in genres.iter() {
            let media_list = match nekos_fun.get_media_list_by_genre(genre).await {
                Ok(media_list) => media_list,
                Err(err) => {
                    panic!("Failed to get media list with genre `{genre:?}`, error: {err}");
                }
            };

            assert!(!media_list.is_empty());
        }
    }
}
