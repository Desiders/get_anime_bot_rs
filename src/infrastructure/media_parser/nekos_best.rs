use crate::{
    application::media_parser::{exceptions::MediaGetException, traits::Source},
    domain::media_parser::entities::{
        genre::{vec_new_sfw_gif, vec_new_sfw_image},
        Genre, Genres, Media,
    },
};

use async_trait::async_trait;
use lazy_static::lazy_static;
use serde::Deserialize;
use std::{borrow::Cow, collections::HashMap};
use tracing::{event, instrument, Level};

#[derive(Debug, Clone)]
pub struct NekosBest<Client = reqwest::Client> {
    url: Cow<'static, str>,
    client: Client,
}

impl<Client> NekosBest<Client> {
    /// Create a new instance of [`NekosBest`].
    /// # Arguments
    /// * `client` - The client to use for the requests.
    /// # Note
    /// By default, the url is set as `https://nekos.best/api/v2`.
    /// If you want to change it, use the `NekosBest::with_url` method.
    pub fn new(client: Client) -> NekosBest<Client> {
        Self {
            client,
            url: "https://nekos.best/api/v2".into(),
        }
    }
}

impl Default for NekosBest {
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
struct ApiResult {
    url: String,
    // artist_href: Option<String>,
    // artist_name: Option<String>,
    // source_url: Option<String>,
    // anime_name: Option<String>,
}

#[derive(Debug, Deserialize)]
struct ApiResponse {
    results: Vec<ApiResult>,
}

#[async_trait]
impl Source for NekosBest<reqwest::Client> {
    fn name(&self) -> &str {
        "nekos.best.v2"
    }

    fn url(&self) -> &str {
        &self.url
    }

    fn genres(&self) -> &Genres {
        lazy_static! {
            static ref GENRES: Genres = Genres::new(
                [
                    vec_new_sfw_gif![
                        "baka", "bite", "blush", "bored", "cry", "cuddle", "dance", "facepalm",
                        "feed", "handhold", "happy", "highfive", "hug", "kick", "kiss", "laugh",
                        "nod", "nom", "nope", "pat", "poke", "pout", "punch", "shoot", "shrug",
                        "slap", "sleep", "smile", "smug", "stare", "think", "thumbsup", "tickle",
                        "wave", "wink", "yeet",
                    ],
                    vec_new_sfw_image!["husbando", "kitsune", "neko", "waifu"],
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

        let mut params = HashMap::with_capacity(1);
        params.insert("amount", 20);

        let content = self
            .client
            .get(&url)
            .query(&params)
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

        let mut list = Vec::with_capacity(api_response.results.len());

        for result in api_response.results {
            list.push(Media::new(result.url, genre.clone()));
        }

        Ok(list)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_get_media_list_by_genre() {
        let waifu_pics = NekosBest::new(reqwest::Client::new());

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
