mod application;
mod config;
mod filters;
mod handlers;
mod infrastructure;
mod middlewares;

use config::read_config_from_env;
use handlers::{source as source_handler, start as start_handler};
use middlewares::{ACLMiddleware, DatabaseMiddleware};
use sqlx::{PgPool, Postgres};
use telers::{event::ToServiceProvider, filters::Command, Bot, Dispatcher, Router};

#[tokio::main(flavor = "current_thread")]
async fn main() {
    match pretty_env_logger::try_init_custom_env("LOGGING_LEVEL") {
        Ok(_) => log::debug!("Logger initialized with max level: {}", log::max_level()),
        Err(err) => {
            eprintln!("Error initializing logger: {err}");
            std::process::exit(1);
        }
    }

    let config = match read_config_from_env() {
        Ok(config) => {
            log::debug!("Config read from env");
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
            log::debug!("Database pool created");
            pool
        }
        Err(err) => {
            eprintln!("Error creating database pool: {err}");
            std::process::exit(1);
        }
    };

    let database_middleware = DatabaseMiddleware::new(pool);
    let acl_middleware = ACLMiddleware::<Postgres>::new();

    let bot = Bot::new(config.bot.token);

    let mut main_router = Router::new("main");
    main_router
        .telegram_observers_mut()
        .iter_mut()
        .for_each(|observer| {
            observer
                .outer_middlewares
                .register(database_middleware.clone());
            observer.outer_middlewares.register(acl_middleware);
        });

    let mut user_router = Router::new("users");

    user_router
        .message
        .register(start_handler)
        .filter(Command::many(["start", "help"]));
    user_router
        .message
        .register(source_handler)
        .filter(Command::many(["source", "about"]));

    main_router.include(user_router);

    let dispatcher = Dispatcher::builder().bot(bot).router(main_router).build();

    match dispatcher
        .to_service_provider_default()
        .unwrap()
        .run_polling()
        .await
    {
        Ok(_) => log::warn!("Bot stopped"),
        Err(err) => log::error!("Bot stopped with error: {err}"),
    }
}
