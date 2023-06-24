mod application;
mod config;
mod domain;
mod handlers;
mod infrastructure;

use config::read_config_from_env;
use handlers::{source as source_handler, start as start_handler};
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

    let bot = Bot::new(config.bot.token);

    let mut router = Router::new("main");
    router
        .message
        .register(start_handler)
        .filter(Command::many(["start", "help"]));
    router
        .message
        .register(source_handler)
        .filter(Command::many(["source", "about"]));

    let dispatcher = Dispatcher::builder().bot(bot).main_router(router).build();

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
