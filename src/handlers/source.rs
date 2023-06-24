use telers::{
    enums::ParseMode,
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::Message,
    utils::text_decorations::{TextDecoration as _, HTML_DECORATION},
    Bot,
};

pub async fn source(bot: Bot, message: Message) -> HandlerResult {
    let text = format!(
        "The bot has open source code!\n\n{source_link}",
        source_link = HTML_DECORATION.link(
            "Source code",
            "https://github.com/Desiders/get_anime_bot_rs",
        ),
    );

    bot.send(
        &SendMessage::new(message.chat.id, text).parse_mode(ParseMode::HTML),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}
