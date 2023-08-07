use crate::{application::media_parser::traits::Source, extractors::MediaParserSourceWrapper};

use telers::{
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::Message,
    Bot,
};

pub async fn gifs(
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
