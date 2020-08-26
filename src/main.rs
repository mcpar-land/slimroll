use std::{collections::HashSet, env, sync::Arc};

use clap::clap_app;
use log::{error, info};
use serenity::{
	async_trait,
	client::bridge::gateway::ShardManager,
	http::Http,
	model::{event::ResumedEvent, gateway::Ready},
	prelude::*,
};

use crate::handler::Handler;
use crate::roll::*;
use env_vars::*;

mod emojis;
mod env_vars;
mod error;
mod handler;
mod reactions;
mod roll;

use crate::emojis::*;

pub use crate::error::EmojiError;

struct ShardManagerContainer;

impl TypeMapKey for ShardManagerContainer {
	type Value = Arc<Mutex<ShardManager>>;
}

#[tokio::main]
async fn main() {
	let matches = clap_app!(slimroll =>
		(@arg REFRESH: -r --refresh "Refresh emojis on launch")
	)
	.get_matches();

	env_logger::init();
	&DISCORD_TOKEN;
	&EMOJI_SERVER;
	&EMOJI_COPIES;

	let mut client = Client::new(&*DISCORD_TOKEN)
		.event_handler(Handler)
		.await
		.expect("Error creating client");

	let http = &client.cache_and_http.as_ref().http;

	if matches.is_present("REFRESH") {
		println!("Forcing regeneration of emojis...");
		crate::emojis::clear_emojis(http)
			.await
			.expect("Error clearing emojis");
		crate::emojis::upload_emojis(http)
			.await
			.expect("Error uploading emojis");
	}

	if let Err(why) = setup_emojis(http).await {
		panic!("Error setting up emojis: {}", why);
	}

	if let Err(why) = client.start().await {
		error!("Client error: {:?}", why);
	};
}
