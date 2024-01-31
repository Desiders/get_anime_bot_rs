use crate::{
    application::{
        common::{
            exceptions::{BeginError, CommitError, RepoError, RepoKind, RollbackError},
            traits::{UnitOfWork, UnitOfWorkFactory},
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
use std::borrow::Cow;
use time::OffsetDateTime;
use tokio::{
    sync::mpsc::{channel as tokio_mpsc_channel, Receiver},
    time as tokio_time,
};
use tracing::{event, field::display, instrument, Level, Span};
use uuid::Uuid;

#[allow(clippy::module_name_repetitions)]
#[derive(Debug)]
pub struct WorkerManager {
    channel_buffer: usize,
    backoff: ExponentialBackoff<SystemClock>,
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
    #[instrument(skip_all, fields(source_name = "nekos_best"))]
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
                                %err,
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
                                %err,
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
    #[instrument(skip_all, fields(source_name = "nekos_fun"))]
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
                                %err,
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
                                %err,
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
    #[instrument(skip_all, fields(source_name = "waifu_pics"))]
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
                                %err,
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
                                %err,
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

/// Run polling for a source and worker manager.
/// This function creates a source in the database if it doesn't exist
/// and then starts polling for media from the source and save them in the database.
/// # Arguments
/// * `worker` - Worker manager for the source.
/// * `source` - Source to parse.
/// * `uow_factory` - Unit of work factory.
#[instrument(skip_all, fields(source_id, name = %source.name(), url = %source.url()))]
pub async fn run_polling<S, UoWFactory>(
    worker: WorkerManager,
    source: S,
    uow_factory: UoWFactory,
) -> Result<(), ErrorKind>
where
    S: Source + 'static,
    WorkerManager: Worker<S>,
    UoWFactory: UnitOfWorkFactory,
{
    let mut source_id = Uuid::new_v4();

    Span::current().record("source_id", display(&source_id));

    event!(Level::DEBUG, "Creating source");

    let mut uow = uow_factory.new_unit_of_work();

    let create_source_result = uow
        .source_repo()
        .await?
        .create(CreateSource::new(&source_id, source.name(), source.url()))
        .await;

    match create_source_result {
        Ok(()) => {
            uow.commit().await?;

            event!(Level::DEBUG, "Source created");
        }
        Err(RepoKind::Exception(_)) => {
            uow.rollback().await?;

            let db_source = uow
                .source_reader()
                .await?
                .get_by_name_and_url(GetSourceByNameAndUrl::new(source.name(), source.url()))
                .await?;

            event!(Level::DEBUG, "Source already exists");

            source_id = db_source.id;
        }
        Err(RepoKind::Unexpected(err)) => {
            uow.rollback().await?;

            event!(Level::ERROR, %err, "Unexpected error while creating source");

            return Err(err.into());
        }
    };

    let mut receiver = Worker::<S>::parse(worker, source).await;

    while let Some(media) = receiver.recv().await {
        let media_id = Uuid::new_v4();

        let create_media_result = uow
            .media_repo()
            .await?
            .create(CreateMedia::new(
                &media_id,
                media.url(),
                Some(media.genre().name()),
                media.genre().media_type().as_str(),
                Some(media.genre().is_sfw()),
                &source_id,
            ))
            .await;

        match create_media_result {
            Ok(()) => uow.commit().await?,
            Err(RepoKind::Exception(_)) => uow.rollback().await?,
            Err(RepoKind::Unexpected(_)) => uow.rollback().await?,
        };
    }

    Ok(())
}

/// Run polling for all known sources.
/// # Arguments
/// * `uow_factory` - Unit of work factory.
pub async fn run_pollings<UoWFactory>(
    nekos_fun: NekosFun,
    nekos_best: NekosBest,
    waifu_pics: WaifuPics,
    uow_factory: UoWFactory,
) where
    UoWFactory: UnitOfWorkFactory + Clone + Send + 'static,
    UoWFactory::UnitOfWork: Send,
{
    tokio::join!(
        async {
            match tokio::spawn(run_polling(
                WorkerManager::default(),
                nekos_fun,
                uow_factory.clone(),
            ))
            .await
            {
                Ok(Ok(())) => {
                    event!(Level::INFO, "Worker manager for `nekos_fun` stopped");
                }
                Ok(Err(err)) => {
                    event!(Level::ERROR, %err, "Worker manager for `nekos_fun` stopped with error");
                }
                Err(err) => {
                    event!(Level::ERROR, %err, "Worker manager for `nekos_fun` panicked");
                }
            }
        },
        async {
            match tokio::spawn(run_polling(
                WorkerManager::default(),
                nekos_best,
                uow_factory.clone(),
            ))
            .await
            {
                Ok(Ok(())) => {
                    event!(Level::INFO, "Worker manager for `nekos_best` stopped");
                }
                Ok(Err(err)) => {
                    event!(Level::ERROR, %err, "Worker manager for `nekos_best` stopped with error");
                }
                Err(err) => {
                    event!(Level::ERROR, %err, "Worker manager for `nekos_best` panicked");
                }
            }
        },
        async {
            match tokio::spawn(run_polling(
                WorkerManager::default(),
                waifu_pics,
                uow_factory.clone(),
            ))
            .await
            {
                Ok(Ok(())) => {
                    event!(Level::INFO, "Worker manager for `waifu_pics` stopped");
                }
                Ok(Err(err)) => {
                    event!(Level::ERROR, %err, "Worker manager for `waifu_pics` stopped with error");
                }
                Err(err) => {
                    event!(Level::ERROR, %err, "Worker manager for `waifu_pics` panicked");
                }
            }
        },
    );
}
