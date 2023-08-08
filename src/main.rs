mod application;
mod config;
mod domain;
mod extractors;
mod filters;
mod handlers;
mod infrastructure;
mod middlewares;

use config::read_config_from_env;
use infrastructure::{
    database::SqlxUnitOfWork,
    media_parser::{NekosBest, NekosFun, WaifuPics},
};
use middlewares::{
    Acl as ACLMiddleware, Database as DatabaseMiddleware,
    MediaParserSources as MediaParserSourcesMiddleware,
};
use sqlx::{PgPool, Postgres};
use telers::{event::ToServiceProvider, filters::Command, Bot, Dispatcher, Router};
use tracing::{debug_span, error_span, instrument, warn_span};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

#[instrument]
#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
async fn main() {
    let config = match read_config_from_env() {
        Ok(config) => {
            tracing_subscriber::registry()
                .with(fmt::layer().with_thread_names(true))
                .with(EnvFilter::from_env("LOGGING_LEVEL"))
                .init();

            debug_span!("Config read from env");
            config
        }
        Err(err) => {
            eprintln!("Error reading config from env: {err}");
            std::process::exit(1);
        }
    };

    let url = format!(
        "postgres://{user}:{password}@{host}:{port}/{db}",
        user = config.database.user,
        password = config.database.password,
        host = config.database.host,
        port = config.database.port,
        db = config.database.db,
    );
    let pool = match PgPool::connect(&url).await {
        Ok(pool) => {
            debug_span!("Database pool created");
            pool
        }
        Err(err) => {
            eprintln!("Error creating database pool: {err}");
            std::process::exit(1);
        }
    };

    let mut main_router = Router::new("main");

    let database_middleware = DatabaseMiddleware::new(pool);
    let acl_middleware = ACLMiddleware::<SqlxUnitOfWork<Postgres>>::new();

    main_router
        .telegram_observers_mut()
        .iter_mut()
        .for_each(|observer| {
            observer
                .outer_middlewares
                .register(database_middleware.clone());
            observer.outer_middlewares.register(acl_middleware);
        });

    let nekos_best = NekosBest::default();
    let nekos_fun = NekosFun::default();
    let waifu_pics = WaifuPics::default();

    let media_parser_sources_middleware = MediaParserSourcesMiddleware::default()
        .source(nekos_best)
        .source(nekos_fun)
        .source(waifu_pics);

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
        .filter(Command::builder().ignore_case(true).command("gifs").build());
    user_router
        .message
        .register(handlers::media::images)
        .filter(
            Command::builder()
                .ignore_case(true)
                .command("images")
                .build(),
        );

    main_router.include(user_router);

    let bot = Bot::new(config.bot.token);

    let dispatcher = Dispatcher::builder().bot(bot).router(main_router).build();

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => {
            warn_span!("Bot stopped");
        }
        Err(err) => {
            error_span!("Bot stopped with error", error = %err);
        }
    }
}
