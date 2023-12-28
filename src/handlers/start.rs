use telers::{
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};
use tracing::instrument;

#[instrument(skip_all)]
pub async fn start(bot: Bot, message: Message) -> HandlerResult {
    let text = format!(
        "Hi, {first_name}!\n\n\
        Get an anime GIF or image by genre!\n\
        /gifs\n\
        /images",
        first_name = match message.from() {
            Some(user) => &user.first_name,
            None => "anonymous",
        }
    );

    bot.send(
        SendMessage::new(message.chat().id(), text).reply_markup(InlineKeyboardMarkup::new([[
            InlineKeyboardButton::new("Settings").callback_data("user settings"),
        ]])),
    )
    .await?;

    Ok(EventReturn::Finish)
}
