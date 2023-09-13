use crate::{
    application::{
        common::traits::UnitOfWork, media::dto::GetMediaByInfoUnviewedByUser,
        media_parser::traits::Source, user_media_view::dto::CreateUserMediaView,
    },
    domain::media_parser::entities::Genre,
    domain::user::entities::User as UserEntity,
    extractors::{MediaParserSourceWrapper, UnitOfWorkWrapper},
};

use std::borrow::Cow;
use telers::{
    errors::HandlerError,
    event::{telegram::HandlerResult, EventReturn},
    filters::CommandObject,
    methods::{SendDocument, SendMediaGroup, SendMessage},
    types::{InputFile, InputMediaDocument, Message},
    Bot,
};
use tracing::{event, instrument, Level};
use uuid::Uuid;

#[instrument(skip_all)]
pub async fn gifs(
    bot: Bot,
    message: Message,
    MediaParserSourceWrapper(media_parser_sources): MediaParserSourceWrapper,
) -> HandlerResult {
    let genres = media_parser_sources
        .iter()
        .map(Source::genres)
        .collect::<Vec<_>>();

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
            nsfw_genres.join(" ").to_string()
        }
    );

    bot.send(
        &SendMessage::new(message.chat.id, text).reply_to_message_id(message.message_id),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all)]
pub async fn images(
    bot: Bot,
    message: Message,
    media_parser_sources: MediaParserSourceWrapper,
) -> HandlerResult {
    let media_parser_sources = media_parser_sources.inner();

    let genres = media_parser_sources
        .iter()
        .map(Source::genres)
        .collect::<Vec<_>>();

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
            nsfw_genres.join(" ").to_string()
        }
    );

    bot.send(
        &SendMessage::new(message.chat.id, text).reply_to_message_id(message.message_id),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}

#[instrument(skip_all)]
pub async fn genre<UoW>(
    bot: Bot,
    message: Message,
    UnitOfWorkWrapper(uow): UnitOfWorkWrapper<UoW>,
    CommandObject {
        command: genre,
        args,
        ..
    }: CommandObject,
    UserEntity {
        id: db_user_id,
        show_nsfw,
        ..
    }: UserEntity,
) -> HandlerResult
where
    UoW: UnitOfWork,
{
    let genre: Genre = match genre.as_str().try_into() {
        Ok(genre) => genre,
        Err(err) => {
            event!(Level::DEBUG, error = %err, raw_genre = genre, "Failed to parse genre");

            bot.send(
                &SendMessage::new(message.chat.id, err.to_string())
                    .reply_to_message_id(message.message_id),
                None,
            )
            .await?;

            return Ok(EventReturn::Finish);
        }
    };

    let show_nsfw = if let Some(show_nsfw) = show_nsfw {
        show_nsfw
    } else {
        false
    };

    if !genre.is_sfw() && !show_nsfw {
        bot.send(
            &SendMessage::new(
                message.chat.id,
                "NSFW content is disabled. You can enable it in the settings.\n\n/settings",
            )
            .reply_to_message_id(message.message_id),
            None,
        )
        .await?;

        return Ok(EventReturn::Finish);
    }

    #[allow(clippy::cast_sign_loss)]
    let count_media = if let Some(Ok(count)) = args.get(0).map(|arg| arg.parse::<i8>()) {
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

    let mut uow = uow.lock().await;

    let media_group = uow
        .media_reader()
        .await
        .map_err(HandlerError::new)?
        .get_by_info_unviewed_by_user(GetMediaByInfoUnviewedByUser::new(
            db_user_id,
            Some(Cow::Owned(genre.name().to_owned())),
            genre.media_type().as_str(),
            Some(genre.is_sfw()),
            None,
            Some(count_media),
        ))
        .await
        .map_err(HandlerError::new)?;

    let media_group_len = media_group.len();

    if media_group_len == 0 {
        bot.send(
            &SendMessage::new(message.chat.id, "No media found for genre")
                .reply_to_message_id(message.message_id),
            None,
        )
        .await?;
    } else if media_group_len == 1 {
        // `unwrap` is safe here, because we checked that `media_group_len` is equal to 1
        let media = media_group.first().unwrap();

        bot.send(
            &SendDocument::new(message.chat.id, InputFile::url(&media.url))
                .reply_to_message_id(message.message_id),
            None,
        )
        .await?;

        uow.user_media_view_repo()
            .await
            .map_err(HandlerError::new)?
            .create(CreateUserMediaView::new(
                Uuid::new_v4(),
                db_user_id,
                media.id,
            ))
            .await
            .map_err(HandlerError::new)?;

        uow.commit().await.map_err(HandlerError::new)?;
    } else {
        let input_media_group = media_group
            .iter()
            .map(|media| InputMediaDocument::new(InputFile::url(&media.url)));

        bot.send(
            &SendMediaGroup::new(message.chat.id, input_media_group)
                .reply_to_message_id(message.message_id),
            None,
        )
        .await?;

        for media in media_group {
            uow.user_media_view_repo()
                .await
                .map_err(HandlerError::new)?
                .create(CreateUserMediaView::new(
                    Uuid::new_v4(),
                    db_user_id,
                    media.id,
                ))
                .await
                .map_err(HandlerError::new)?;
        }

        uow.commit().await.map_err(HandlerError::new)?;
    }

    Ok(EventReturn::Finish)
}
