use crate::EmojiError;
use serenity::{async_trait, http::Http, model::prelude::*, prelude::*};
use std::collections::HashMap;
use std::env;

mod generate;

pub fn emoji_server_id() -> Result<u64, EmojiError> {
	env::var("EMOJI_SERVER")
		.or(Err(EmojiError::NoEmojiServer))?
		.parse::<u64>()
		.or(Err(EmojiError::NoEmojiServer))
}

pub async fn emoji_server(http: &Http) -> Result<PartialGuild, EmojiError> {
	let emoji_server_id = emoji_server_id()?;

	let emoji_server = http
		.get_guild(emoji_server_id)
		.await
		.or(Err(EmojiError::NoEmojiServer))?;

	Ok(emoji_server)
}

pub async fn emojis(
	http: &Http,
) -> Result<HashMap<EmojiId, Emoji>, EmojiError> {
	let server = emoji_server(http).await?;

	Ok(server.emojis)
}
