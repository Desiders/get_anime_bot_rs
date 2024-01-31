use crate::{
    application::{
        common::traits::{UnitOfWork, UnitOfWorkFactory},
        user::dto::UpdateUserShowNsfw,
    },
    domain::user::entities::User as UserEntity,
    extractors::UoWFactoryWrapper,
};

use anyhow::anyhow;
use telers::{
    errors::HandlerError,
    event::{telegram::HandlerResult, EventReturn},
    methods::{AnswerCallbackQuery, DeleteMessage, SendMessage},
    types::{
        CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, MaybeInaccessibleMessage,
        Message, ReplyParameters, User,
    },
    Bot,
};
use tracing::{event, instrument, Level, Span};

#[instrument(skip_all)]
pub async fn settings(bot: Bot, message: Message) -> HandlerResult {
    Span::current().record("user_id", message.from_id());

    event!(Level::DEBUG, "Sending settings");

    bot.send(
        SendMessage::new(message.chat().id(), "Settings")
            .reply_parameters(ReplyParameters::new(message.id()))
            .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                "Change age restriction (SFW / NSFW)",
            )
            .callback_data("user update_age_restriction")]])),
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all, fields(%callback_query_id, %user_id))]
pub async fn settings_callback(
    bot: Bot,
    CallbackQuery {
        id: callback_query_id,
        from: User { id: user_id, .. },
        message: maybe_inaccessible_message,
        ..
    }: CallbackQuery,
) -> HandlerResult {
    let (chat_id, message_id) =
        if let Some(MaybeInaccessibleMessage::Message(message)) = maybe_inaccessible_message {
            (message.chat().id(), message.id())
        } else {
            event!(
                Level::WARN,
                "Callback query doesn't have chat id. Message is too old",
            );

            bot.send(
                AnswerCallbackQuery::new(callback_query_id)
                    .text("Message is too old. Please, send the command again"),
            )
            .await?;

            return Ok(EventReturn::Finish);
        };

    bot.send(
        SendMessage::new(chat_id, "Settings")
            .reply_parameters(ReplyParameters::new(message_id))
            .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                "Change age restriction (SFW / NSFW)",
            )
            .callback_data("user update_age_restriction")]])),
    )
    .await?;

    bot.send(AnswerCallbackQuery::new(callback_query_id))
        .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all, fields(%callback_query_id, %user_id))]
pub async fn update_age_restriction(
    bot: Bot,
    CallbackQuery {
        id: callback_query_id,
        from: User { id: user_id, .. },
        message: maybe_inaccessible_message,
        ..
    }: CallbackQuery,
    UserEntity { show_nsfw, .. }: UserEntity,
) -> HandlerResult {
    let (chat_id, message_id) =
        if let Some(MaybeInaccessibleMessage::Message(message)) = maybe_inaccessible_message {
            (message.chat().id(), message.id())
        } else {
            event!(
                Level::WARN,
                "Callback query doesn't have chat id. Message is too old",
            );

            bot.send(
                AnswerCallbackQuery::new(callback_query_id)
                    .text("Message is too old. Please, send the command again"),
            )
            .await?;

            return Ok(EventReturn::Finish);
        };

    let show_nsfw = show_nsfw.map_or(false, |show_nsfw| show_nsfw);

    if show_nsfw {
        event!(Level::DEBUG, "Show NSFW is enabled");

        bot.send(
            SendMessage::new(chat_id, "Change age restriction")
                .reply_parameters(ReplyParameters::new(message_id))
                .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                    "Disable show NSFW",
                )
                .callback_data("user disable_show_nsfw")]])),
        )
        .await?;
    } else {
        event!(Level::DEBUG, "Show NSFW is disabled");

        bot.send(
            SendMessage::new(
                chat_id,
                "Change age restriction\n\nBy clicking on the button, you confirm that you're 18 years old",
            )
                .reply_parameters(ReplyParameters::new(message_id))
                .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                    "Enable show NSFW (18+)",
                )
                .callback_data("user enable_show_nsfw")]])),
        )
        .await?;
    }

    bot.send(AnswerCallbackQuery::new(callback_query_id))
        .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all, fields(%callback_query_id, %user_id))]
pub async fn update_age_restriction_callback<UoWFactory>(
    bot: Bot,
    CallbackQuery {
        id: callback_query_id,
        from: User { id: user_id, .. },
        data,
        message: maybe_inaccessible_message,
        ..
    }: CallbackQuery,
    UoWFactoryWrapper(uow_factory): UoWFactoryWrapper<UoWFactory>,
    UserEntity { id: db_user_id, .. }: UserEntity,
) -> HandlerResult
where
    UoWFactory: UnitOfWorkFactory,
{
    // `unwrap` is safe here, because we use `Text` filter for this handler, so we can be sure that `data` is `Some`
    let callback_data = data.as_deref().unwrap();

    let show_nsfw = match callback_data {
        "user enable_show_nsfw" => true,
        "user disable_show_nsfw" => false,
        _ => {
            return Err(HandlerError::new(anyhow!(
                "Unknown callback data. Callback data: {callback_data}",
            )));
        }
    };

    event!(Level::DEBUG, show_nsfw, "Updating show NSFW");

    let mut uow = uow_factory.new_unit_of_work();

    uow.user_repo()
        .await
        .map_err(HandlerError::new)?
        .update_show_nsfw(UpdateUserShowNsfw::new(&db_user_id, show_nsfw))
        .await
        .map_err(HandlerError::new)?;

    uow.commit().await.map_err(HandlerError::new)?;

    drop(uow);

    event!(Level::DEBUG, show_nsfw, "Show NSFW updated");

    let text = if show_nsfw {
        "You have enabled show NSFW!"
    } else {
        "You have disabled show NSFW!"
    };

    bot.send(
        AnswerCallbackQuery::new(callback_query_id)
            .text(text)
            .cache_time(5),
    )
    .await?;

    if let Some(MaybeInaccessibleMessage::Message(message)) = maybe_inaccessible_message {
        bot.send(DeleteMessage::new(message.chat().id(), message.id()))
            .await?;
    } else {
        event!(
            Level::WARN,
            "Callback query doesn't have message. Message is too old"
        );
    };

    Ok(EventReturn::Finish)
}
