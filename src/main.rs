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
    database::SqlxUnitOfWorkFactory,
    media_parser::{worker, NekosBest, WaifuPics},
};
use middlewares::{
    Database as DatabaseMiddleware, MediaParserSources as MediaParserSourcesMiddleware,
    ACL as ACLMiddleware,
};
use sqlx::{PgPool, Pool, Postgres};
use telers::{
    errors::HandlerError,
    event::ToServiceProvider,
    filters::{Command, Text},
    methods::SetMyCommands,
    types::{BotCommand, BotCommandScopeAllPrivateChats},
    Bot, Dispatcher, Router,
};
use tracing::{event, Level};
use tracing_subscriber::{fmt, layer::SubscriberExt as _, util::SubscriberInitExt as _, EnvFilter};

async fn set_my_commands(bot: Bot) -> Result<(), HandlerError> {
    let help_command = BotCommand::new("help", "Show help message");
    let source_command = BotCommand::new("source", "Show source of the bot");
    let gifs_command = BotCommand::new("gifs", "Get random gifs");
    let images_command = BotCommand::new("images", "Get random images");

    let private_chats = [help_command, source_command, gifs_command, images_command];

    bot.send(SetMyCommands::new(private_chats.clone()).scope(BotCommandScopeAllPrivateChats {}))
        .await?;

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
#[allow(clippy::too_many_lines)]
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

    let pool = match PgPool::connect(&config.database.get_postgres_url()).await {
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

    main_router
        .update
        .outer_middlewares
        .register(DatabaseMiddleware::new(SqlxUnitOfWorkFactory::new(
            pool.clone(),
        )));
    main_router
        .update
        .outer_middlewares
        .register(ACLMiddleware::<SqlxUnitOfWorkFactory<Postgres>>::new());

    let nekos_best = NekosBest::default();
    let waifu_pics = WaifuPics::default();

    main_router.message.inner_middlewares.register(
        MediaParserSourcesMiddleware::default()
            .source(nekos_best.clone())
            .source(waifu_pics.clone()),
    );

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
        .register(handlers::stats::stats::<SqlxUnitOfWorkFactory<Postgres>>)
        .filter(Command::many(["stats", "statistics"]));
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

    if config.media_parser_worker.start_worker {
        main_router.startup.register(
            |nekos_best, waifu_pics, pool| async {
                tokio::spawn(worker::run_pollings(
                    nekos_best,
                    waifu_pics,
                    SqlxUnitOfWorkFactory::new(pool),
                ));

                Ok(())
            },
            (nekos_best, waifu_pics, pool.clone()),
        );
    } else {
        event!(Level::WARN, "Media parser worker disabled. To enable it set `START_MEDIA_PARSER_WORKER` to `true` in env");
    }

    // Shutdown the connection pool
    main_router.shutdown.register(
        |pool| async move {
            Pool::close(&pool).await;
            Ok(())
        },
        (pool,),
    );

    let bot = Bot::new(config.bot.token);

    main_router
        .startup
        .register(set_my_commands, (bot.clone(),));

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
        Ok(()) => {
            event!(Level::WARN, "Bot stopped");
        }
        Err(err) => {
            event!(Level::ERROR, %err, "Bot stopped with error");
        }
    }
}
