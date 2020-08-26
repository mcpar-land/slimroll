use crate::env_vars::*;
use crate::error::EmojiError;
use crate::roll::{roll, roll_details};
use caith::{RollResult, Roller};
use lazy_static::lazy_static;
use regex::Regex;
use serenity::{
	async_trait, client::bridge::gateway::ShardManager, http::Http,
	model::prelude::*, prelude::*,
};

lazy_static! {
	static ref PREFIX_REGEX: Regex =
		Regex::new(&format!("{}(.*)", *PREFIX)).unwrap();
}

async fn process_message(
	msg: &Message,
) -> Result<Option<(bool, RollResult)>, EmojiError> {
	match PREFIX_REGEX.captures(&msg.content) {
		Some(capture) => {
			let mut formula = capture
				.get(1)
				.ok_or(EmojiError::BadCommand)?
				.as_str()
				.to_string();
			let is_detail = formula.get(..1) == Some("?");
			if is_detail {
				formula = formula.get(1..).unwrap().to_string();
			}

			let roller = Roller::new(&formula)?;
			let res = roller.roll()?;
			Ok(Some((is_detail, res)))
		}
		None => Ok(None),
	}
}

pub struct Handler;

#[async_trait]
impl EventHandler for Handler {
	async fn message(&self, ctx: Context, msg: Message) {
		match process_message(&msg).await {
			Ok(res_opt) => {
				if let Some((detail, roll_res)) = res_opt {
					if detail {
						roll_details(&ctx, &msg, roll_res).await;
					} else {
						roll(&ctx, &msg, roll_res).await;
					}
				}
			}
			Err(why) => match why {
				EmojiError::RollError(err) => match err {
					caith::RollError::ParamError(errmsg) => {
						msg
							.react(ctx.http.clone(), ReactionType::Unicode("👎".to_string()))
							.await
							.ok();
					}
					_ => {
						msg
							.react(ctx.http.clone(), ReactionType::Unicode("❗".to_string()))
							.await
							.ok();
					}
				},
				_ => {
					msg
						.react(ctx.http.clone(), ReactionType::Unicode("❗".to_string()))
						.await
						.ok();
				}
			},
		};
	}
}
