use crate::{
    application::media_parser::traits::{Source, Worker},
    domain::media_parser::entities::Media,
    infrastructure::media_parser::{NekosBest, NekosFun, WaifuPics},
};

use async_trait::async_trait;
use backoff::{backoff::Backoff as _, exponential::ExponentialBackoff, SystemClock};
use std::borrow::Cow;
use time::OffsetDateTime;
use tokio::{
    sync::mpsc::{channel as tokio_mpsc_channel, Receiver},
    time as tokio_time,
};
use tracing::{debug_span, error_span, info_span, instrument, warn_span};

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct WorkerManager {
    name: Cow<'static, str>,
    channel_buffer: usize,
    backoff: ExponentialBackoff<SystemClock>,
}

impl WorkerManager {
    /// Create a new [`WorkerManager`]
    /// # Arguments
    /// * `name` - The name of the worker
    /// # Notes
    /// * The default channel buffer is 100
    /// * The default backoff is [`ExponentialBackoff::default`]
    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self {
            name: name.into(),
            channel_buffer: 100,
            backoff: ExponentialBackoff::default(),
        }
    }

    /// Set the channel buffer size for the worker
    pub fn with_channel_buffer(self, channel_buffer: usize) -> Self {
        Self {
            channel_buffer,
            ..self
        }
    }

    /// Set the backoff for the worker
    pub fn with_backoff(self, backoff: ExponentialBackoff<SystemClock>) -> Self {
        Self { backoff, ..self }
    }
}

#[async_trait]
impl Worker<NekosBest<reqwest::Client>> for WorkerManager {
    #[instrument]
    async fn parse(mut self, source: NekosBest<reqwest::Client>) -> Receiver<Media> {
        let (sender, receiver) = tokio_mpsc_channel(self.channel_buffer);

        tokio::spawn(async move {
            let genres = source.genres();

            let mut failed = false;

            loop {
                for genre in genres.iter() {
                    let media_list = match source.get_media_list_by_genre(genre).await {
                        Ok(media_list) => media_list,
                        Err(err) => {
                            error_span!(
                                "Error getting media list",
                                source_name = %self.name,
                                err = %err,
                            );

                            if let Some(backoff) = self.backoff.next_backoff() {
                                warn_span!(
                                    "Sleep and try again...",
                                    source_name = %self.name,
                                    duration = backoff.as_secs_f32(),
                                );

                                tokio_time::sleep(backoff).await;
                            }

                            continue;
                        }
                    };

                    if failed {
                        info_span!(
                            "Connection established successfully",
                            source_name = %self.name
                        );

                        failed = false;

                        self.backoff.reset();
                    }

                    let media_list_len = media_list.len();

                    let now = OffsetDateTime::now_utc();

                    for media in media_list {
                        if let Err(err) = sender.send(media).await {
                            error_span!(
                                "Error sending media to channel",
                                source_name = %self.name,
                                err = %err,
                            );
                        }
                    }

                    let elapsed = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    debug_span!(
                        "Media list parsed",
                        source_name = %self.name,
                        media_list_len = media_list_len,
                        elapsed = elapsed,
                    );

                    if elapsed < 1.5 {
                        tokio_time::sleep(tokio_time::Duration::from_secs_f32(1.5 - elapsed)).await;
                    }
                }
            }
        });

        receiver
    }
}

#[async_trait]
impl Worker<NekosFun<reqwest::Client>> for WorkerManager {
    #[instrument]
    async fn parse(mut self, source: NekosFun<reqwest::Client>) -> Receiver<Media> {
        let (sender, receiver) = tokio_mpsc_channel(self.channel_buffer);

        tokio::spawn(async move {
            let genres = source.genres();

            let mut failed = false;

            loop {
                for genre in genres.iter() {
                    let media_list = match source.get_media_list_by_genre(genre).await {
                        Ok(media_list) => media_list,
                        Err(err) => {
                            error_span!(
                                "Error getting media list",
                                source_name = %self.name,
                                err = %err,
                            );

                            if let Some(backoff) = self.backoff.next_backoff() {
                                warn_span!(
                                    "Sleep and try again...",
                                    source_name = %self.name,
                                    duration = backoff.as_secs_f32(),
                                );

                                tokio_time::sleep(backoff).await;
                            }

                            continue;
                        }
                    };

                    if failed {
                        info_span!(
                            "Connection established successfully",
                            source_name = %self.name,
                        );

                        failed = false;

                        self.backoff.reset();
                    }

                    let media_list_len = media_list.len();

                    let now = OffsetDateTime::now_utc();

                    for media in media_list {
                        if let Err(err) = sender.send(media).await {
                            error_span!(
                                "Error sending media to channel",
                                source_name = %self.name,
                                err = %err,
                            );
                        }
                    }

                    let elapsed = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    debug_span!(
                        "Media list parsed",
                        source_name = %self.name,
                        media_list_len = media_list_len,
                        elapsed = elapsed,
                    );

                    if elapsed < 1.0 {
                        tokio_time::sleep(tokio_time::Duration::from_secs_f32(1.0 - elapsed)).await;
                    }
                }
            }
        });

        receiver
    }
}

#[async_trait]
impl Worker<WaifuPics<reqwest::Client>> for WorkerManager {
    #[instrument]
    async fn parse(mut self, mut source: WaifuPics<reqwest::Client>) -> Receiver<Media> {
        let (sender, receiver) = tokio_mpsc_channel(self.channel_buffer);

        tokio::spawn(async move {
            let genres = source.genres().clone();

            let mut failed = false;

            loop {
                for genre in genres.iter() {
                    let media_list = match source.get_media_list_by_genre(genre).await {
                        Ok(media_list) => media_list,
                        Err(err) => {
                            error_span!(
                                "Error getting media list",
                                source_name = %self.name,
                                err = %err,
                            );

                            if let Some(backoff) = self.backoff.next_backoff() {
                                warn_span!(
                                    "Sleep and try again...",
                                    source_name = %self.name,
                                    duration = backoff.as_secs_f32(),
                                );

                                tokio_time::sleep(backoff).await;
                            }

                            continue;
                        }
                    };

                    if failed {
                        info_span!(
                            "Connection established successfully",
                            source_name = %self.name,
                        );

                        failed = false;

                        self.backoff.reset();
                    }

                    let media_list_len = media_list.len();

                    let now = OffsetDateTime::now_utc();

                    for media in media_list {
                        let media_url = media.url().clone();

                        if let Err(err) = sender.send(media).await {
                            error_span!(
                                "Error sending media to channel",
                                source_name = %self.name,
                                err = %err,
                            );
                        } else {
                            source.exclude_url(media_url);
                        }
                    }

                    let elapsed = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    debug_span!(
                        "Media list parsed",
                        source_name = %self.name,
                        media_list_len = media_list_len,
                        elapsed = elapsed,
                    );

                    if elapsed < 2.5 {
                        tokio_time::sleep(tokio_time::Duration::from_secs_f32(2.5 - elapsed)).await;
                    }
                }
            }
        });

        receiver
    }
}
