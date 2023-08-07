use telers::{
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::Message,
    Bot,
};

pub async fn start(bot: Bot, message: Message) -> HandlerResult {
    let text = format!(
        "Hi, {first_name}!\n\n\
        Get an anime GIF or image by genre!\n\
        /gifs\n\
        /images",
        first_name = match message.from {
            Some(user) => user.first_name,
            None => "anonymous".to_string(),
        }
    );

    bot.send(&SendMessage::new(message.chat.id, text), None)
        .await?;

    Ok(EventReturn::Finish)
}
