use crate::{
    application::media_parser::traits::MediaSource,
    domain::media::{
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

#[derive(Debug, Clone)]
pub struct WaifuPics<Client> {
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

    /// Set the url of the api.
    /// By default, it's set as `https://api.waifu.pics`.
    pub fn with_url(self, url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            client: self.client,
            exclude_urls: self.exclude_urls,
            url: url.into(),
        }
    }

    /// Set the exclude url
    pub fn with_exclude_url(self, exclude_url: impl Into<Cow<'static, str>>) -> Self {
        Self {
            client: self.client,
            exclude_urls: self
                .exclude_urls
                .into_iter()
                .chain(Some(exclude_url.into()))
                .collect(),
            url: self.url,
        }
    }

    /// Set the exclude urls
    pub fn with_exclude_urls<T, I>(self, exclude_urls: I) -> Self
    where
        T: Into<Cow<'static, str>>,
        I: IntoIterator<Item = T>,
    {
        Self {
            client: self.client,
            exclude_urls: self
                .exclude_urls
                .into_iter()
                .chain(exclude_urls.into_iter().map(Into::into))
                .collect(),
            url: self.url,
        }
    }

    /// Set the exclude url with a mutable reference
    pub fn exclude_url(&mut self, exclude_url: impl Into<Cow<'static, str>>) {
        self.exclude_urls.push(exclude_url.into());
    }

    /// Set the exclude urls with a mutable reference
    pub fn exclude_urls<T, I>(&mut self, exclude_urls: I)
    where
        T: Into<Cow<'static, str>>,
        I: IntoIterator<Item = T>,
    {
        self.exclude_urls = exclude_urls.into_iter().map(Into::into).collect()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error(transparent)]
    Serde(#[from] serde_json::Error),
    #[error(transparent)]
    Reqwest(#[from] reqwest::Error),
    #[error("Incorrect genre: {msg}")]
    IncorrectGenre { msg: Cow<'static, str> },
}

impl ErrorKind {
    pub fn incorrect_genre(msg: impl Into<Cow<'static, str>>) -> Self {
        Self::IncorrectGenre { msg: msg.into() }
    }
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    files: Vec<String>,
}

#[async_trait]
impl MediaSource for WaifuPics<reqwest::Client> {
    type GetMediaError = ErrorKind;

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

    async fn get_media_list_by_genre(
        &self,
        genre: &Genre,
    ) -> Result<Vec<Media>, Self::GetMediaError> {
        let age_restriction = match genre.age_restriction() {
            AgeRestriction::Sfw => "sfw",
            AgeRestriction::Nsfw => "nsfw",
            AgeRestriction::Unknown => {
                return Err(ErrorKind::incorrect_genre(
                    "Only SFW/NSFW restrictions are valid",
                ))
            }
        };
        let url = format!(
            "{api_url}/many/{age_restriction}/{genre}",
            api_url = self.url,
            genre = genre.name()
        );

        let exclude_urls = serde_json::to_string(&self.exclude_urls)?;
        let form = Form::new().text("exclude", exclude_urls);

        let content = self
            .client
            .post(&url)
            .multipart(form)
            .send()
            .await?
            .text()
            .await?;
        println!("content: {content}");

        let api_response: ApiResponse = serde_json::from_str(&content)?;

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
