mod application;
mod config;
mod domain;
mod extractors;
mod filters;
mod handlers;
mod infrastructure;
mod middlewares;

use config::{read_config_from_env, Database};
use infrastructure::{
    database::SqlxUnitOfWork,
    media_parser::{NekosBest, NekosFun, WaifuPics},
};
use middlewares::{
    Acl as ACLMiddleware, Database as DatabaseMiddleware,
    MediaParserSources as MediaParserSourcesMiddleware,
};
use sqlx::{PgPool, Postgres};
use telers::{
    event::ToServiceProvider,
    filters::{Command, Text},
    Bot, Dispatcher, Router,
};
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
        .register(handlers::user::update_age_restriction_callback::<SqlxUnitOfWork<Postgres>>)
        .filter(Text::many([
            "user enable_show_nsfw",
            "user disable_show_nsfw",
        ]));
    user_router
        .message
        .register(handlers::media::genre::<SqlxUnitOfWork<Postgres>>)
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
}
