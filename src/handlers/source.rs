use telers::{
    enums::ParseMode,
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::Message,
    utils::text::html_text_link,
    Bot,
};
use tracing::{event, instrument, Level, Span};

#[instrument(skip_all, fields(message_id, user_id))]
pub async fn source(bot: Bot, message: Message) -> HandlerResult {
    Span::current()
        .record("message_id", message.id())
        .record("user_id", message.from_id());

    event!(Level::DEBUG, "Sending source message");

    let text = format!(
        "The bot has open source code!\n\n{source_link}",
        source_link = html_text_link(
            "Source code",
            "https://github.com/Desiders/get_anime_bot_rs",
        ),
    );

    bot.send(SendMessage::new(message.chat().id(), text).parse_mode(ParseMode::HTML))
        .await?;

    Ok(EventReturn::Finish)
}
