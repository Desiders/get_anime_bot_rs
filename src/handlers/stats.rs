use crate::{
    application::common::traits::{UnitOfWork, UnitOfWorkFactory},
    extractors::UoWFactoryWrapper,
};

use telers::{
    errors::HandlerError,
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::{MessageText, ReplyParameters},
    Bot,
};
use tracing::{event, instrument, Level, Span};

#[instrument(skip_all, fields(%message_id))]
pub async fn stats<UoWFactory>(
    bot: Bot,
    MessageText {
        id: message_id,
        from,
        chat,
        ..
    }: MessageText,
    UoWFactoryWrapper(uow_factory): UoWFactoryWrapper<UoWFactory>,
) -> HandlerResult
where
    UoWFactory: UnitOfWorkFactory,
{
    Span::current().record("user_id", from.map(|user| user.id));

    event!(Level::DEBUG, "Getting media stats");

    let mut uow = uow_factory.new_unit_of_work();

    let mut media_reader = uow.media_reader().await.map_err(HandlerError::new)?;

    let media_stats = media_reader
        .get_media_stats()
        .await
        .map_err(HandlerError::new)?;

    let genre_stats = media_reader
        .get_genre_stats()
        .await
        .map_err(HandlerError::new)?;

    let text = format!("Media statistics:\n\n{media_stats}\n\n{genre_stats}");

    event!(Level::TRACE, "Sending media stats");

    bot.send(SendMessage::new(chat.id(), text).reply_parameters(ReplyParameters::new(message_id)))
        .await?;

    Ok(EventReturn::Finish)
}
