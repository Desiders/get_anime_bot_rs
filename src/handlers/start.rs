use telers::{
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::{InlineKeyboardButton, InlineKeyboardMarkup, Message},
    Bot,
};
use tracing::{event, instrument, Level, Span};

#[instrument(skip_all, fields(message_id, user_id))]
pub async fn start(bot: Bot, message: Message) -> HandlerResult {
    Span::current()
        .record("message_id", message.id())
        .record("user_id", message.from_id());

    event!(Level::DEBUG, "Sending start message");

    let text = format!(
        "Hi, {first_name}!\n\n\
        Get an anime GIF or image by genre!\n\
        /gifs\n\
        /images\n\n\
        /stats\n\n\
        You can also pass media count you want to get. For example:\n\
        /neko_img_sfw 5\n",
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
