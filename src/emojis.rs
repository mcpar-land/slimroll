use crate::emojis::generate::number_emojis;
use crate::env_vars::*;
use crate::EmojiError;
use futures::future::{join_all, FutureExt};
use indicatif::{ProgressBar, ProgressIterator};
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

	// async fn del_emoji(
	// 	http: &Http,
	// 	guild: &PartialGuild,
	// 	(id, emoji): &(EmojiId, Emoji),
	// 	bar: &ProgressBar,
	// ) -> Result<(), serenity::Error> {
	// 	bar.set_message(&emoji.name);
	// 	let res = guild.delete_emoji(http, id).await;
	// 	bar.inc(1);
	// 	res
	// }

	// let futures = emojis.iter().map(|key| del_emoji(http, &guild, &key, &bar));

	let mut zipped_results: Vec<Result<(EmojiId, Emoji), EmojiError>> = vec![];

	for (id, emoji) in emojis.iter() {
		bar.set_message(&emoji.name);
		zipped_results.push(match guild.delete_emoji(http, id).await {
			Ok(_) => Ok((id.clone(), emoji.clone())),
			Err(_) => Err(EmojiError::EmojiUploadFailed),
		});
		bar.inc(1);
	}

	// let future_results = join_all(futures).await;

	bar.finish();

	// let zipped_results = {
	// 	let mut results = vec![];
	// 	for (i, res) in future_results.iter().enumerate() {
	// 		results.push(match res {
	// 			Ok(_) => Ok(emojis.get(i).unwrap().clone()),
	// 			Err(_) => Err(EmojiError::EmojiUploadFailed),
	// 		});
	// 	}
	// 	results
	// };

	println!("Emojis cleared");

	Ok(zipped_results)
}

pub async fn upload_emojis(http: &Http) -> Result<(), EmojiError> {
	let guild = emoji_server(http).await?;
	println!("Generating emojis...");
	let number_emojis = number_emojis()?;

	let emoji_names: Vec<Vec<String>> = {
		let mut e: Vec<Vec<String>> = vec![];
		for i in 0..*EMOJI_COPIES {
			e.push({
				let mut v = vec![];
				for j in 0..number_emojis.len() {
					v.push(format!("slimroll_{}_c{}", j, i));
				}
				v
			})
		}
		e
	};

	let mut futures = vec![];

	for i in 0..*EMOJI_COPIES {
		for (j, emoji) in number_emojis.iter().enumerate() {
			let name = &emoji_names.get(i).unwrap().get(j).unwrap();
			let e = &emoji;
			// println!("{} :::: {:?}", name, e);
			futures.push(guild.create_emoji(http, name, e));
		}
	}

	println!("Uploading emojis...");
	let future_results = join_all(futures).await;
	println!(
		"Uploaded {} emojis",
		future_results.iter().filter(|x| x.is_ok()).count()
	);
	Ok(())
}
