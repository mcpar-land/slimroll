use crate::emojis::*;
use crate::env_vars::*;
use crate::reactions::*;
use caith::{RollResult, Roller};
use log::{error, info};
use serenity::{async_trait, model::prelude::*, prelude::*};

pub async fn roll(
	ctx: &Context,
	msg: &Message,
	roll: RollResult,
) -> serenity::Result<()> {
	if let Some(values) =
		emojis_for_number(roll.get_total() as i64, *EMOJI_COPIES)
	{
		let emojis = EMOJIS.read().await;
		for val in values {
			if let Some(emoji) = emojis.get(&val) {
				if let Err(why) = msg
					.react(
						&ctx.http,
						EmojiIdentifier {
							id: emoji.id,
							name: emoji.name.clone(),
						},
					)
					.await
				{
					println!("React for {:?} ({:?}) failed: {}", val, emoji, why);
				};
			}
		}
		if let Err(why) = msg
			.react(&ctx.http, ReactionType::Unicode("✅".to_string()))
			.await
		{
			println!("React failed {}", why);
		}
	} else {
		roll_details(ctx, msg, roll).await?;
	}

	Ok(())
}

pub async fn roll_details(
	ctx: &Context,
	msg: &Message,
	roll: RollResult,
) -> serenity::Result<()> {
	msg
		.react(ctx.http.clone(), ReactionType::Unicode("⤵".to_string()))
		.await?;
	msg
		.reply(ctx.http.clone(), format!("{}", roll.to_string()))
		.await?;
	Ok(())
}
