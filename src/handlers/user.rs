use crate::{
    application::{
        common::traits::{UnitOfWork, UnitOfWorkFactory},
        user::dto::UpdateUserShowNsfw,
    },
    domain::user::entities::User as UserEntity,
    extractors::UnitOfWorkFactoryWrapper,
};

use anyhow::anyhow;
use telers::{
    errors::HandlerError,
    event::{telegram::HandlerResult, EventReturn},
    methods::{AnswerCallbackQuery, DeleteMessage, SendMessage},
    types::{CallbackQuery, InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};
use tracing::{event, instrument, Level};

#[instrument(skip_all)]
pub async fn settings(bot: Bot, message: Message) -> HandlerResult {
    bot.send(
        SendMessage::new(message.chat().id(), "Settings")
            .reply_to_message_id(message.id())
            .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                "Change age restriction (SFW / NSFW)",
            )
            .callback_data("user update_age_restriction")]])),
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all)]
pub async fn settings_callback(bot: Bot, callback_query: CallbackQuery) -> HandlerResult {
    let (chat_id, message_id) = if let Some(message) = callback_query.message {
        (message.chat().id(), message.id())
    } else {
        event!(
            Level::WARN,
            ?callback_query,
            "Callback query doesn't have chat id. Message is too old"
        );

        bot.send(
            AnswerCallbackQuery::new(callback_query.id)
                .text("Message is too old. Please, send the command again"),
        )
        .await?;

        return Ok(EventReturn::Finish);
    };

    bot.send(
        SendMessage::new(chat_id, "Settings")
            .reply_to_message_id(message_id)
            .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                "Change age restriction (SFW / NSFW)",
            )
            .callback_data("user update_age_restriction")]])),
    )
    .await?;

    bot.send(AnswerCallbackQuery::new(callback_query.id))
        .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all)]
pub async fn update_age_restriction(
    bot: Bot,
    callback_query: CallbackQuery,
    UserEntity { show_nsfw, .. }: UserEntity,
) -> HandlerResult {
    let show_nsfw = if let Some(show_nsfw) = show_nsfw {
        show_nsfw
    } else {
        false
    };

    let (chat_id, message_id) = if let Some(message) = callback_query.message {
        (message.chat().id(), message.id())
    } else {
        event!(
            Level::WARN,
            ?callback_query,
            "Callback query doesn't have chat id. Message is too old"
        );

        bot.send(
            AnswerCallbackQuery::new(callback_query.id)
                .text("Message is too old. Please, send the command again"),
        )
        .await?;

        return Ok(EventReturn::Finish);
    };

    if show_nsfw {
        bot.send(
            SendMessage::new(chat_id, "Change age restriction")
                .reply_to_message_id(message_id)
                .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                    "Disable show NSFW",
                )
                .callback_data("user disable_show_nsfw")]])),
        )
        .await?;
    } else {
        bot.send(
            SendMessage::new(
                chat_id,
                "Change age restriction\n\nBy clicking on the button, you confirm that you're 18 years old",
            )
                .reply_to_message_id(message_id)
                .reply_markup(InlineKeyboardMarkup::new([[InlineKeyboardButton::new(
                    "Enable show NSFW (18+)",
                )
                .callback_data("user enable_show_nsfw")]])),
        )
        .await?;
    }

    bot.send(AnswerCallbackQuery::new(callback_query.id))
        .await?;

    Ok(EventReturn::Finish)
}

pub async fn update_age_restriction_callback<UoWFactory>(
    bot: Bot,
    callback_query: CallbackQuery,
    UnitOfWorkFactoryWrapper(uow_factory): UnitOfWorkFactoryWrapper<UoWFactory>,
    UserEntity { id: db_user_id, .. }: UserEntity,
) -> HandlerResult
where
    UoWFactory: UnitOfWorkFactory,
{
    // `unwrap` is safe here, because we use `Text` filter for this handler, so we can be sure that `data` is `Some`
    let callback_data = callback_query.data.as_deref().unwrap();

    let show_nsfw = match callback_data {
        "user enable_show_nsfw" => true,
        "user disable_show_nsfw" => false,
        _ => {
            event!(Level::ERROR, ?callback_query, "Unknown callback data");

            return Err(HandlerError::new(anyhow!(
                "Unknown callback data. Callback query: {callback_query:?}",
            )));
        }
    };

    let mut uow = uow_factory.new_unit_of_work();

    uow.user_repo()
        .await
        .map_err(HandlerError::new)?
        .update_show_nsfw(UpdateUserShowNsfw::new(db_user_id, show_nsfw))
        .await
        .map_err(HandlerError::new)?;

    uow.commit().await.map_err(HandlerError::new)?;

    let text = if show_nsfw {
        "You have enabled show NSFW!"
    } else {
        "You have disabled show NSFW!"
    };

    bot.send(
        AnswerCallbackQuery::new(callback_query.id.as_ref())
            .text(text)
            .cache_time(5),
    )
    .await?;

    if let Some(message) = callback_query.message {
        bot.send(DeleteMessage::new(message.chat().id(), message.id()))
            .await?;
    } else {
        event!(
            Level::WARN,
            ?callback_query,
            "Callback query doesn't have message. Message is too old",
        );
    }

    Ok(EventReturn::Finish)
}
