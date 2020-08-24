use crate::emojis::*;
use d20;
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
	// match d20::roll_dice(args.message()) {
	// 	Ok(res) => {
	// 		msg.reply(ctx, res.to_string()).await?;
	// 	}
	// 	Err(_) => {
	// 		msg
	// 			.react(ctx.http.clone(), ReactionType::Unicode("❌".to_string()))
	// 			.await?;
	// 	}
	// };

	let emojis = emojis(ctx.http.as_ref()).await?;
	msg.reply(ctx, format!("```{:#?}```", emojis)).await?;

	Ok(())
}
