use crate::{application::media_parser::traits::Source, extractors::MediaParserSourceWrapper};

use telers::{
    event::{telegram::HandlerResult, EventReturn},
    methods::SendMessage,
    types::Message,
    Bot,
};

pub async fn sfw_genres(
    bot: Bot,
    message: Message,
    media_parser_sources: MediaParserSourceWrapper,
) -> HandlerResult {
    let media_parser_sources = media_parser_sources.inner();

    let genres = media_parser_sources
        .iter()
        .map(Source::genres)
        .collect::<Vec<_>>();

    let mut sfw_gif_genres = genres
        .iter()
        .flat_map(|genres| genres.sfw_gifs())
        .map(|genre| format!("/{genre}"))
        .collect::<Vec<_>>();
    sfw_gif_genres.sort();
    sfw_gif_genres.dedup();

    let mut sfw_image_genres = genres
        .iter()
        .flat_map(|genres| genres.sfw_images())
        .map(|genre| format!("/{genre}"))
        .collect::<Vec<_>>();
    sfw_image_genres.sort();
    sfw_image_genres.dedup();

    let sfw_gif_genres_string = sfw_gif_genres.join(" ").to_string();
    let sfw_image_genres_string = sfw_image_genres.join(" ").to_string();

    let text = format!(
        "SFW GIF genres:\n{sfw_gif_genres_string}\n\nSFW image genres:\n{sfw_image_genres_string}"
    );

    bot.send(
        &SendMessage::new(message.chat.id, text).reply_to_message_id(message.message_id),
        None,
    )
    .await?;

    Ok(EventReturn::Finish)
}
