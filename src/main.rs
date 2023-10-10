mod application;
mod config;
mod domain;
mod extractors;
mod filters;
mod handlers;
mod infrastructure;
mod middlewares;

use application::common::traits::UnitOfWork;
use config::{read_config_from_env, Database};
use infrastructure::{
    database::{SqlxUnitOfWork, SqlxUnitOfWorkFactory},
    media_parser::{
        worker::start_worker_manager_and_save, NekosBest, NekosFun, WaifuPics, WorkerManager,
    },
};
use middlewares::{
    Database as DatabaseMiddleware, MediaParserSources as MediaParserSourcesMiddleware,
    ACL as ACLMiddleware,
};
use sqlx::{PgPool, Postgres};
use std::sync::Arc;
use telers::{
    event::ToServiceProvider,
    filters::{Command, Text},
    Bot, Dispatcher, Router,
};
use tokio::sync::Mutex;
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

fn get_database_url_by_config(config: &Database) -> String {
    format!(
        "postgres://{user}:{password}@{host}:{port}/{db}",
        user = config.user,
        password = config.password,
        host = config.host,
        port = config.port,
        db = config.db,
    )
}

async fn start_media_parser_workers<UoW>(
    uow: Arc<Mutex<UoW>>,
    nekos_best: NekosBest,
    nekos_fun: NekosFun,
    waifu_pics: WaifuPics,
) where
    UoW: UnitOfWork + Send + 'static,
{
    tokio::join!(
        async {
            event!(Level::INFO, "Worker manager for `nekos_best` started");

            match tokio::spawn(start_worker_manager_and_save(
                WorkerManager::default(),
                nekos_fun,
                uow.clone(),
            ))
            .await
            {
                Ok(Ok(_)) => {
                    event!(Level::INFO, "Worker manager for `nekos_fun` stopped");
                }
                Ok(Err(err)) => {
                    event!(Level::ERROR, error = ?err, "Worker manager for `nekos_fun` stopped with error");
                }
                Err(err) => {
                    event!(Level::ERROR, error = ?err, "Worker manager for `nekos_fun` panicked");
                }
            }
        },
        async {
            event!(Level::INFO, "Worker manager for `nekos_best` started");

            match tokio::spawn(start_worker_manager_and_save(
                WorkerManager::default(),
                nekos_best,
                uow.clone(),
            ))
            .await
            {
                Ok(Ok(_)) => {
                    event!(Level::INFO, "Worker manager for `nekos_best` stopped");
                }
                Ok(Err(err)) => {
                    event!(Level::ERROR, error = ?err, "Worker manager for `nekos_best` stopped with error");
                }
                Err(err) => {
                    event!(Level::ERROR, error = ?err, "Worker manager for `nekos_best` panicked");
                }
            }
        },
        async {
            event!(Level::INFO, "Worker manager for `waifu_pics` started");

            match tokio::spawn(start_worker_manager_and_save(
                WorkerManager::default(),
                waifu_pics,
                uow.clone(),
            ))
            .await
            {
                Ok(Ok(_)) => {
                    event!(Level::INFO, "Worker manager for `waifu_pics` stopped");
                }
                Ok(Err(err)) => {
                    event!(Level::ERROR, error = ?err, "Worker manager for `waifu_pics` stopped with error");
                }
                Err(err) => {
                    event!(Level::ERROR, error = ?err, "Worker manager for `waifu_pics` panicked");
                }
            }
        },
    );
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let config = match read_config_from_env() {
        Ok(config) => {
            tracing_subscriber::registry()
                .with(fmt::layer())
                .with(EnvFilter::from_env("LOGGING_LEVEL"))
                .init();

            event!(Level::DEBUG, "Config loaded from env");

            config
        }
        Err(err) => {
            eprintln!("Error reading config from env: {err}");
            std::process::exit(1);
        }
    };

    let url = get_database_url_by_config(&config.database);
    let pool = match PgPool::connect(&url).await {
        Ok(pool) => {
            event!(Level::DEBUG, "Database pool created");

            pool
        }
        Err(err) => {
            eprintln!("Error creating database pool: {err}");

            std::process::exit(1);
        }
    };

    let mut main_router = Router::new("main");

    let database_middleware = DatabaseMiddleware::new(pool.clone());
    let acl_middleware = ACLMiddleware::<SqlxUnitOfWorkFactory<Postgres>>::new();

    main_router
        .update
        .outer_middlewares
        .register(database_middleware.clone());
    main_router
        .update
        .outer_middlewares
        .register(acl_middleware);

    let nekos_best = NekosBest::default();
    let nekos_fun = NekosFun::default();
    let waifu_pics = WaifuPics::default();

    let media_parser_sources_middleware = MediaParserSourcesMiddleware::default()
        .source(nekos_best.clone())
        .source(nekos_fun.clone())
        .source(waifu_pics.clone());

    main_router
        .message
        .inner_middlewares
        .register(media_parser_sources_middleware);

    let mut user_router = Router::new("users");

    user_router
        .message
        .register(handlers::start::start)
        .filter(Command::many(["start", "help"]));
    user_router
        .message
        .register(handlers::source::source)
        .filter(Command::many(["source", "about"]));
    user_router
        .message
        .register(handlers::media::gifs)
        .filter(Command::one("gifs"));
    user_router
        .message
        .register(handlers::media::images)
        .filter(Command::one("images"));
    user_router
        .message
        .register(handlers::user::settings)
        .filter(Command::one("settings"));
    user_router
        .callback_query
        .register(handlers::user::settings_callback)
        .filter(Text::one("user settings"));
    user_router
        .callback_query
        .register(handlers::user::update_age_restriction)
        .filter(Text::one("user update_age_restriction"));
    user_router
        .callback_query
        .register(
            handlers::user::update_age_restriction_callback::<SqlxUnitOfWorkFactory<Postgres>>,
        )
        .filter(Text::many([
            "user enable_show_nsfw",
            "user disable_show_nsfw",
        ]));
    user_router
        .message
        .register(handlers::media::genre::<SqlxUnitOfWorkFactory<Postgres>>)
        .filter(Text::starts_with_single("/"));

    main_router.include(user_router);

    // Shutdown the connection pool
    main_router.shutdown.register(
        |database_middleware: DatabaseMiddleware<_>| async {
            database_middleware.close().await;

            Ok(())
        },
        (database_middleware,),
    );

    let bot = Bot::new(config.bot.token);

    let dispatcher = Dispatcher::builder()
        .bot(bot)
        .allowed_updates(main_router.resolve_used_update_types())
        .router(main_router)
        .build();

    let media_parser_worker = if config.media_parser_worker.start_worker {
        let uow = Arc::new(Mutex::new(SqlxUnitOfWork::new(pool)));

        Some(tokio::spawn(start_media_parser_workers(
            uow.clone(),
            nekos_best,
            nekos_fun,
            waifu_pics,
        )))
    } else {
        event!(Level::WARN, "Media parser worker disabled. To enable it set `START_MEDIA_PARSER_WORKER` to `true` in env");

        None
    };

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => {
            event!(Level::WARN, "Bot stopped");
        }
        Err(err) => {
            event!(Level::ERROR, error = ?err, "Bot stopped with error");
        }
    }

    if let Some(media_parser_worker) = media_parser_worker {
        media_parser_worker.abort();
    }
}
