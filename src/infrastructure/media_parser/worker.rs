use crate::{
    application::{
        common::{
            exceptions::{BeginError, CommitError, RepoError, RepoKind, RollbackError},
            traits::UnitOfWork,
        },
        media::dto::CreateMedia,
        media_parser::traits::{Source, Worker},
        source::{
            dto::{CreateSource, GetSourceByNameAndUrl},
            exceptions::{SourceNameAndUrlAlreadyExists, SourceNameAndUrlNotExist},
        },
    },
    domain::media_parser::entities::Media,
    infrastructure::media_parser::{NekosBest, NekosFun, WaifuPics},
};

use async_trait::async_trait;
use backoff::{backoff::Backoff as _, exponential::ExponentialBackoff, SystemClock};
use std::{borrow::Cow, sync::Arc};
use time::OffsetDateTime;
use tokio::{
    sync::{
        mpsc::{channel as tokio_mpsc_channel, Receiver},
        Mutex,
    },
    time as tokio_time,
};
use tracing::{event, instrument, Level};
use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct WorkerManager {
    channel_buffer: usize,
    backoff: ExponentialBackoff<SystemClock>,
}

impl WorkerManager {
    /// Create a new [`WorkerManager`]
    /// # Notes
    /// * The default channel buffer is 100
    /// * The default backoff is [`ExponentialBackoff::default`]
    pub fn new() -> Self {
        Self::default()
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

impl Default for WorkerManager {
    fn default() -> Self {
        Self {
            channel_buffer: 100,
            backoff: ExponentialBackoff::default(),
        }
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
                    let now = OffsetDateTime::now_utc();

                    let media_list = match source.get_media_list_by_genre(genre).await {
                        Ok(media_list) => media_list,
                        Err(err) => {
                            event!(
                                Level::ERROR,
                                error = %err,
                                "Error getting media list",
                            );

                            if let Some(duration) = self.backoff.next_backoff() {
                                event!(Level::WARN, "Sleep and try again at {duration:2?}",);

                                tokio_time::sleep(duration).await;
                            }

                            continue;
                        }
                    };

                    if failed {
                        event!(Level::INFO, "Connection established successfully",);

                        failed = false;

                        self.backoff.reset();
                    }

                    let media_list_len = media_list.len();

                    let elapsed = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    event!(
                        Level::DEBUG,
                        "Media list with {media_list_len} media parsed in {elapsed} seconds",
                    );

                    for media in media_list {
                        if let Err(err) = sender.send(media).await {
                            event!(Level::ERROR,
                                error = %err,
                                "Error sending media to channel",
                            );
                        }
                    }

                    let elapsed_with_send = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    if elapsed_with_send < 1.0 {
                        tokio_time::sleep(tokio_time::Duration::from_secs_f32(
                            1.0 - elapsed_with_send,
                        ))
                        .await;
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
                    let now = OffsetDateTime::now_utc();

                    let media_list = match source.get_media_list_by_genre(genre).await {
                        Ok(media_list) => media_list,
                        Err(err) => {
                            event!(Level::ERROR,
                                error = %err,
                                "Error getting media list",
                            );

                            if let Some(duration) = self.backoff.next_backoff() {
                                event!(Level::WARN, "Sleep and try again at {duration:2?}",);

                                tokio_time::sleep(duration).await;
                            }

                            continue;
                        }
                    };

                    if failed {
                        event!(Level::INFO, "Connection established successfully",);

                        failed = false;

                        self.backoff.reset();
                    }

                    let media_list_len = media_list.len();

                    let elapsed = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    event!(
                        Level::DEBUG,
                        "Media list with {media_list_len} media parsed in {elapsed} seconds",
                    );

                    for media in media_list {
                        if let Err(err) = sender.send(media).await {
                            event!(Level::ERROR,
                                error = %err,
                                "Error sending media to channel",
                            );
                        }
                    }

                    let elapsed_with_send = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    if elapsed_with_send < 1.0 {
                        tokio_time::sleep(tokio_time::Duration::from_secs_f32(
                            1.0 - elapsed_with_send,
                        ))
                        .await;
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
                    let now = OffsetDateTime::now_utc();

                    let media_list = match source.get_media_list_by_genre(genre).await {
                        Ok(media_list) => media_list,
                        Err(err) => {
                            event!(Level::ERROR,
                                error = %err,
                                "Error getting media list",
                            );

                            if let Some(backoff) = self.backoff.next_backoff() {
                                event!(Level::WARN, "Sleep and try again at {backoff:2?}");

                                tokio_time::sleep(backoff).await;
                            }

                            continue;
                        }
                    };

                    if failed {
                        event!(Level::INFO, "Connection established successfully",);

                        failed = false;

                        self.backoff.reset();
                    }

                    let media_list_len = media_list.len();

                    let elapsed = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    event!(
                        Level::DEBUG,
                        "Media list with {media_list_len} media parsed in {elapsed} seconds",
                    );

                    for media in media_list {
                        let media_url = Cow::Owned(media.url().to_owned());

                        if let Err(err) = sender.send(media).await {
                            event!(Level::ERROR,
                                error = %err,
                                "Error sending media to channel",
                            );
                        } else {
                            source.exclude_url(media_url);
                        }
                    }

                    let elapsed_with_send = (OffsetDateTime::now_utc() - now).as_seconds_f32();

                    if elapsed_with_send < 2.5 {
                        tokio_time::sleep(tokio_time::Duration::from_secs_f32(
                            2.5 - elapsed_with_send,
                        ))
                        .await;
                    }
                }
            }
        });

        receiver
    }
}

#[derive(Debug, thiserror::Error)]
pub enum ErrorKind {
    #[error(transparent)]
    Begin(#[from] BeginError),
    #[error(transparent)]
    Commit(#[from] CommitError),
    #[error(transparent)]
    Rollback(#[from] RollbackError),
    #[error(transparent)]
    SourceNameAndUrlAlreadyExists(#[from] RepoKind<SourceNameAndUrlAlreadyExists>),
    #[error(transparent)]
    SourceNameAndUrlNotExist(#[from] RepoKind<SourceNameAndUrlNotExist>),
    #[error(transparent)]
    Unexpected(#[from] RepoError),
}

pub async fn start_worker_manager_and_save<S, UoW>(
    worker: WorkerManager,
    source: S,
    uow: Arc<Mutex<UoW>>,
) -> Result<(), ErrorKind>
where
    S: Source,
    WorkerManager: Worker<S>,
    UoW: UnitOfWork,
{
    let mut source_id = Uuid::new_v4();

    let mut uow_guard = uow.lock().await;

    let create_source_result = uow_guard
        .source_repo()
        .await?
        .create(CreateSource::new(
            source_id,
            source.name().to_owned(),
            source.url().to_owned(),
        ))
        .await;

    drop(uow_guard);

    uow_guard = uow.lock().await;

    match create_source_result {
        Ok(_) => {
            uow_guard.commit().await?;

            drop(uow_guard);

            event!(
                Level::DEBUG,
                source_name = source.name(),
                source_url = source.url(),
                "Source created",
            );
        }
        Err(RepoKind::Exception(_)) => {
            uow_guard.rollback().await?;

            drop(uow_guard);

            uow_guard = uow.lock().await;

            let db_source = uow_guard
                .source_reader()
                .await?
                .get_by_name_and_url(GetSourceByNameAndUrl::new(
                    Cow::Owned(source.name().to_owned()),
                    Cow::Owned(source.url().to_owned()),
                ))
                .await?;

            drop(uow_guard);

            event!(
                Level::DEBUG,
                source_name = source.name(),
                source_url = source.url(),
                "Source already exists",
            );

            source_id = db_source.id;
        }
        Err(RepoKind::Unexpected(err)) => {
            uow_guard.rollback().await?;

            drop(uow_guard);

            event!(Level::ERROR, error = ?err, source_name = source.name(), source_url = source.url(), "Unexpected error while creating source");

            return Err(err.into());
        }
    };

    let mut receiver = Worker::<S>::parse(worker, source).await;

    while let Some(media) = receiver.recv().await {
        uow_guard = uow.lock().await;

        let create_media_result = uow_guard
            .media_repo()
            .await?
            .create(CreateMedia::new(
                Uuid::new_v4(),
                media.url().to_owned(),
                Some(media.genre().name().to_owned().into()),
                media.genre().media_type().as_str(),
                Some(media.genre().is_sfw()),
                source_id,
            ))
            .await;

        drop(uow_guard);

        uow_guard = uow.lock().await;

        match create_media_result {
            Ok(_) => uow_guard.commit().await?,
            Err(RepoKind::Exception(_)) => uow_guard.rollback().await?,
            Err(RepoKind::Unexpected(_)) => uow_guard.rollback().await?,
        };

        drop(uow_guard);
    }

    Ok(())
}
