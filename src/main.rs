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

mod emojis;
mod error;
mod roll;

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
	kankyo::load(false).expect("Failed to load .env file");
	env_logger::init();
	let token = env::var("DISCORD_TOKEN").expect("Expected DISCORD_TOKEN");
	env::var("EMOJI_SERVER").expect("Expected EMOJI_SERVER");

	let mut client = Client::new(&token)
		.event_handler(Handler)
		.framework(
			StandardFramework::new()
				.configure(|c| c.prefix("~"))
				.group(&GENERAL_GROUP),
		)
		.await
		.expect("Error creating client");

	if let Err(why) = client.start().await {
		error!("Client error: {:?}", why);
	};
}
