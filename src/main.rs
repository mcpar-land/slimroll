use std::{collections::HashSet, env, sync::Arc};

use log::{error, info};
use serenity::{
	async_trait,
	client::bridge::gateway::ShardManager,
	framework::{standard::macros::group, StandardFramework},
	http::Http,
	model::{event::ResumedEvent, gateway::Ready},
	prelude::*,
};

use crate::roll::*;
use env_vars::*;

mod emojis;
mod env_vars;
mod error;
mod reactions;
mod roll;

use crate::emojis::*;

pub use crate::error::EmojiError;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
	type Value = Arc<Mutex<ShardManager>>;
}

#[group]
#[commands(roll)]
struct General;

struct Handler;
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
	env_logger::init();
	&DISCORD_TOKEN;
	&EMOJI_SERVER;
	&EMOJI_COPIES;

	let mut client = Client::new(&*DISCORD_TOKEN)
		.event_handler(Handler)
		.framework(
			StandardFramework::new()
				.configure(|c| c.prefix("~"))
				.group(&GENERAL_GROUP),
		)
		.await
		.expect("Error creating client");

	let http = &client.cache_and_http.as_ref().http;

	if let Err(why) = setup_emojis(http).await {
		panic!("Error setting up emojis: {}", why);
	}

	// crate::emojis::clear_emojis(http)
	// 	.await
	// 	.expect("Error clearing emojis");

	// crate::emojis::upload_emojis(http)
	// 	.await
	// 	.expect("Error uploading emojis");

	if let Err(why) = client.start().await {
		error!("Client error: {:?}", why);
	};
}
