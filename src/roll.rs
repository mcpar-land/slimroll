use crate::emojis::*;
use crate::env_vars::*;
use crate::reactions::*;
use caith::{RollResult, Roller};
use log::{error, info};
use serenity::{
	async_trait,
	framework::standard::{macros::command, Args, CommandResult},
	model::prelude::*,
	prelude::*,
};

#[command]
pub async fn roll(
	ctx: &Context,
	msg: &Message,
	mut args: Args,
) -> CommandResult {
	let formula = format!("0 + {}", args.message());
	match Roller::new(&formula)?.roll() {
		Ok(res) => {
			if let Some(values) =
				emojis_for_number(res.get_total() as i64, *EMOJI_COPIES)
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
			} else {
				msg.reply(ctx, res.to_string()).await?;
			}
		}
		Err(_) => {
			msg
				.react(ctx.http.clone(), ReactionType::Unicode("❌".to_string()))
				.await?;
		}
	};

	let emojis = emojis(ctx.http.as_ref()).await?;
	msg.reply(ctx, format!("```{:#?}```", emojis)).await?;

	Ok(())
}
