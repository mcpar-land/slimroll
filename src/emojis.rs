use crate::emojis::{emoji_value::EmojiValue, generate::number_emojis};
use crate::env_vars::*;
use crate::EmojiError;
use futures::future::{join_all, FutureExt};
use indicatif::{ProgressBar, ProgressStyle};
use lazy_static::lazy_static;
use serenity::{async_trait, http::Http, model::prelude::*, prelude::*};
use std::collections::HashMap;
use std::{convert::TryFrom, env};

pub mod emoji_value;
pub mod generate;

lazy_static! {
	pub static ref EMOJIS: RwLock<HashMap<EmojiValue, Emoji>> =
		RwLock::new(HashMap::new());
}

pub async fn get(val: &EmojiValue) -> Option<Emoji> {
	EMOJIS.read().await.get(val).cloned()
}

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

pub async fn clear_emojis(
	http: &Http,
) -> Result<Vec<Result<(EmojiId, Emoji), EmojiError>>, EmojiError> {
	println!("Clearing emojis...");
	let guild = emoji_server(http).await?;
	let emojis: Vec<(EmojiId, Emoji)> = guild
		.emojis
		.iter()
		.map(|(k, v)| (k.clone(), v.clone()))
		.collect();

	let bar = ProgressBar::new(emojis.len() as u64);
	bar.tick();

	let mut zipped_results: Vec<Result<(EmojiId, Emoji), EmojiError>> = vec![];

	for (id, emoji) in emojis.iter() {
		bar.set_message(&emoji.name);
		zipped_results.push(match guild.delete_emoji(http, id).await {
			Ok(_) => Ok((id.clone(), emoji.clone())),
			Err(_) => Err(EmojiError::EmojiDeletionFailed),
		});
		bar.inc(1);
	}

	bar.finish();

	println!("Emojis cleared");

	Ok(zipped_results)
}

pub async fn upload_emojis(
	http: &Http,
) -> Result<Vec<Result<Emoji, EmojiError>>, EmojiError> {
	let guild = emoji_server(http).await?;
	println!("Generating emojis...");
	let number_emojis = number_emojis()?;
	println!("Emojis generated!");
	println!("Uploading emojis...");
	let bar = ProgressBar::new(*EMOJI_COPIES as u64 * 10);
	// bar.set_style(ProgressStyle::default_spinner());
	bar.tick();

	let mut zipped_results: Vec<Result<Emoji, EmojiError>> = vec![];

	for i in 0..*EMOJI_COPIES {
		for j in 0..10 {
			// upload the negative in place of the final zero.
			let (val, data_index) = if i == *EMOJI_COPIES - 1 && j == 0 {
				(EmojiValue::Negative, 10)
			} else {
				(EmojiValue::Num(j as u8, i as u8), j)
			};
			let name: String = val.into();
			bar.set_message(&name);
			zipped_results.push(
				guild
					.create_emoji(http, &name, &number_emojis[data_index])
					.await
					.or(Err(EmojiError::EmojiUploadFailed)),
			);
			bar.inc(1);
		}
	}

	bar.finish();

	Ok(zipped_results)
}

fn emoji_map(
	emojis: HashMap<EmojiId, Emoji>,
) -> Result<HashMap<EmojiValue, Emoji>, EmojiError> {
	let mut map: HashMap<EmojiValue, Emoji> = HashMap::new();

	for (_, emoji) in emojis.iter() {
		let val = EmojiValue::try_from(emoji.name.to_string())?;
		map.insert(val, emoji.clone());
	}

	Ok(map)
}

pub async fn setup_emojis(http: &Http) -> Result<(), EmojiError> {
	let guild_emojis = emojis(http).await?;

	let all_emojis = emoji_map(guild_emojis);

	let mut valid = true;
	if let Ok(emojis) = &all_emojis {
		for i in 0..*EMOJI_COPIES {
			for j in 0..=9u8 {
				// don't check the 0 in the last copy, reserved for negative
				let is_neg_slot = i == *EMOJI_COPIES - 1 && j == 0;
				if is_neg_slot || emojis.get(&EmojiValue::Num(j, i as u8)).is_some() {
				} else {
					valid = false;
					break;
				}
			}
			if !valid {
				break;
			}
		}
		if valid {
			valid = emojis.get(&EmojiValue::Negative).is_some();
		}
	} else {
		valid = false;
	}

	let mut global_emojis = EMOJIS.write().await;
	if valid {
		let e = all_emojis.unwrap();
		println!(
			"Emoji server emojis are valid! There are {} emojis.",
			e.len()
		);
		*global_emojis = e.clone();
	} else {
		println!("Emoji server emojis are not valid! Recreating...");
		clear_emojis(http).await?;
		upload_emojis(http).await?;
		let guild_emojis = emojis(http).await?;
		*global_emojis = emoji_map(guild_emojis)?;
		println!("Recreation complete!");
	}

	Ok(())
}
