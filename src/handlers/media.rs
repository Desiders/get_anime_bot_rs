use crate::{
    application::{
        common::{
            exceptions::RepoKind,
            traits::{UnitOfWork, UnitOfWorkFactory},
        },
        media::dto::GetMediaByInfoUnviewedByUser,
        media_parser::traits::Source,
        user_media_view::dto::CreateUserMediaView,
    },
    domain::media_parser::entities::Genre,
    domain::user::entities::User as UserEntity,
    extractors::{MediaParserSourceWrapper, UoWFactoryWrapper},
};

use telers::{
    errors::HandlerError,
    event::{telegram::HandlerResult, EventReturn},
    filters::CommandObject,
    methods::{SendDocument, SendMessage},
    types::{InputFile, Message, MessageText, ReplyParameters},
    Bot,
};
use tracing::{event, field, instrument, Level, Span};
use uuid::Uuid;

#[instrument(skip_all, fields(message_id, user_id))]
pub async fn gifs(
    bot: Bot,
    message: Message,
    MediaParserSourceWrapper(media_parser_sources): MediaParserSourceWrapper,
) -> HandlerResult {
    Span::current()
        .record("message_id", message.id())
        .record("user_id", message.from_id());

    event!(Level::DEBUG, "Getting genres");

    let genres = media_parser_sources
        .iter()
        .map(Source::genres)
        .collect::<Box<[_]>>();

    let mut sfw_genres = genres
        .iter()
        .flat_map(|genres| genres.sfw_gifs())
        .map(|genre| format!("/{genre}"))
        .collect::<Vec<_>>();
    sfw_genres.sort();
    sfw_genres.dedup();

    let mut nsfw_genres = genres
        .iter()
        .flat_map(|genres| genres.nsfw_gifs())
        .map(|genre| format!("/{genre}"))
        .collect::<Vec<_>>();
    nsfw_genres.sort();
    nsfw_genres.dedup();

    let text = format!(
        "{sfw_genres}\n\nNot safe for work:\n{nsfw_genres}",
        sfw_genres = if sfw_genres.is_empty() {
            "No SFW GIFs available".to_string()
        } else {
            sfw_genres.join(" ").to_string()
        },
        nsfw_genres = if nsfw_genres.is_empty() {
            "No NSFW GIFs available".to_string()
        } else {
            let mut text = nsfw_genres.join(" ").to_string();
            text.push_str(
                "\n\n* We don't guarantee that SFW media is really SFW, so don't check it on the bus and if you're younger than 18 y.o. ^_^",
            );
            text
        }
    );

    event!(Level::TRACE, ?sfw_genres, ?nsfw_genres, "Sending genres");

    bot.send(
        SendMessage::new(message.chat().id(), text)
            .reply_parameters(ReplyParameters::new(message.id())),
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all, fields(message_id, user_id))]
pub async fn images(
    bot: Bot,
    message: Message,
    MediaParserSourceWrapper(media_parser_sources): MediaParserSourceWrapper,
) -> HandlerResult {
    Span::current()
        .record("message_id", message.id())
        .record("user_id", message.from_id());

    event!(Level::DEBUG, "Getting genres");

    let genres = media_parser_sources
        .iter()
        .map(Source::genres)
        .collect::<Box<[_]>>();

    let mut sfw_genres = genres
        .iter()
        .flat_map(|genres| genres.sfw_images())
        .map(|genre| format!("/{genre}"))
        .collect::<Vec<_>>();
    sfw_genres.sort();
    sfw_genres.dedup();

    let mut nsfw_genres = genres
        .iter()
        .flat_map(|genres| genres.nsfw_images())
        .map(|genre| format!("/{genre}"))
        .collect::<Vec<_>>();
    nsfw_genres.sort();
    nsfw_genres.dedup();

    let text = format!(
        "{sfw_genres}\n\nNot safe for work:\n{nsfw_genres}",
        sfw_genres = if sfw_genres.is_empty() {
            "No SFW images available".to_string()
        } else {
            sfw_genres.join(" ").to_string()
        },
        nsfw_genres = if nsfw_genres.is_empty() {
            "No NSFW images available".to_string()
        } else {
            let mut text = nsfw_genres.join(" ").to_string();
            text.push_str(
                "\n\n* We don't guarantee that SFW media is really SFW, so don't check it on the bus and if you're younger than 18 y.o. ^_^",
            );
            text
        }
    );

    event!(Level::TRACE, ?sfw_genres, ?nsfw_genres, "Sending genres");

    bot.send(
        SendMessage::new(message.chat().id(), text)
            .reply_parameters(ReplyParameters::new(message.id())),
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all, fields(%message_id, user_id, genre))]
pub async fn genre<UoWFactory>(
    bot: Bot,
    MessageText {
        id: message_id,
        text,
        from,
        chat,
        ..
    }: MessageText,
    UoWFactoryWrapper(uow_factory): UoWFactoryWrapper<UoWFactory>,
    UserEntity {
        id: db_user_id,
        show_nsfw,
        ..
    }: UserEntity,
) -> HandlerResult
where
    UoWFactory: UnitOfWorkFactory,
{
    Span::current().record("user_id", from.map(|user| user.id));

    let Some(CommandObject {
        args,
        command: genre,
        ..
    }) = CommandObject::extract(&text)
    else {
        event!(Level::DEBUG, text, "Not a command");

        return Ok(EventReturn::Finish);
    };

    event!(Level::DEBUG, "Parsing genre");

    let genre: Genre = match genre.as_ref().try_into() {
        Ok(genre) => genre,
        Err(err) => {
            event!(Level::DEBUG, %err, genre, "Failed to parse genre");

            bot.send(
                SendMessage::new(chat.id(), err.to_string())
                    .reply_parameters(ReplyParameters::new(message_id)),
            )
            .await?;

            return Ok(EventReturn::Finish);
        }
    };

    let show_nsfw = show_nsfw.map_or(false, |show_nsfw| show_nsfw);

    if !show_nsfw && genre.is_nsfw() {
        event!(Level::DEBUG, "NSFW content is disabled");

        bot.send(
            SendMessage::new(
                chat.id(),
                "NSFW content is disabled. You can enable it in the settings.\n\n/settings",
            )
            .reply_parameters(ReplyParameters::new(message_id)),
        )
        .await?;

        return Ok(EventReturn::Finish);
    }

    #[allow(clippy::cast_sign_loss)]
    let count_media = if let Some(Ok(count)) = args.first().map(|arg| arg.parse::<i8>()) {
        if count > 10 {
            10
        } else if count <= 0 {
            1
        } else {
            count
        }
    } else {
        1
    } as u64;

    event!(Level::DEBUG, count = count_media, "Getting media");

    let mut uow = uow_factory.new_unit_of_work();

    let media_group = uow
        .media_reader()
        .await
        .map_err(HandlerError::new)?
        .get_by_info_unviewed_by_user(GetMediaByInfoUnviewedByUser::new(
            &db_user_id,
            Some(genre.name()),
            genre.media_type().as_str(),
            Some(genre.is_sfw()),
            None,
            Some(count_media),
        ))
        .await
        .map_err(HandlerError::new)?;

    let media_group_len = media_group.len();

    if media_group_len == 0 {
        event!(Level::DEBUG, "No media found for genre");

        bot.send(
            SendMessage::new(chat.id(), "No media found for genre")
                .reply_parameters(ReplyParameters::new(message_id)),
        )
        .await?;
    } else if media_group_len == 1 {
        // `unwrap` is safe here, because we checked that `media_group_len` is equal to 1
        let media = media_group
            .first()
            .expect("Media group is empty, but it shouldn't be");

        Span::current().record("media_id", field::display(media.id));

        event!(Level::DEBUG, ?media, "Sending media");

        bot.send(
            SendDocument::new(chat.id(), InputFile::url(&media.url))
                .reply_parameters(ReplyParameters::new(message_id)),
        )
        .await?;

        let res = uow
            .user_media_view_repo()
            .await
            .map_err(HandlerError::new)?
            .create(CreateUserMediaView::new(
                &Uuid::new_v4(),
                &db_user_id,
                &media.id,
            ))
            .await;

        match res {
            Ok(()) => {
                uow.commit().await.map_err(HandlerError::new)?;

                event!(Level::DEBUG, "User media view created");
            }

            Err(RepoKind::Unexpected(err)) => {
                uow.rollback().await.map_err(HandlerError::new)?;

                event!(Level::ERROR, %err, "Failed to create user media view");

                return Err(HandlerError::new(err));
            }
            Err(RepoKind::Exception(_)) => {
                uow.rollback().await.map_err(HandlerError::new)?;

                event!(Level::WARN, "User media view already exists");
            }
        }
    } else {
        event!(
            Level::DEBUG,
            count = media_group_len,
            ?media_group,
            "Sending media group",
        );

        for media in media_group {
            event!(Level::DEBUG, ?media, "Sending media");

            bot.send(
                SendDocument::new(chat.id(), InputFile::url(&media.url))
                    .reply_parameters(ReplyParameters::new(message_id)),
            )
            .await?;

            let res = uow
                .user_media_view_repo()
                .await
                .map_err(HandlerError::new)?
                .create(CreateUserMediaView::new(
                    &Uuid::new_v4(),
                    &db_user_id,
                    &media.id,
                ))
                .await;

            match res {
                Ok(()) => {
                    uow.commit().await.map_err(HandlerError::new)?;

                    event!(Level::DEBUG, "User media view created");
                }

                Err(RepoKind::Unexpected(err)) => {
                    uow.rollback().await.map_err(HandlerError::new)?;

                    event!(Level::ERROR, %err, "Failed to create user media view");

                    return Err(HandlerError::new(err));
                }
                Err(RepoKind::Exception(_)) => {
                    uow.rollback().await.map_err(HandlerError::new)?;

                    event!(Level::WARN, "User media view already exists");
                }
            }
        }
    }

    Ok(EventReturn::Finish)
}
