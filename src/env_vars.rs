use lazy_static::lazy_static;

use std::env;

fn load_env(name: &str) -> String {
	kankyo::load(false).expect("Failed to load .env file");
	env::var(name).expect(&format!("Expected {}", name))
}

lazy_static! {
	pub static ref DISCORD_TOKEN: String = load_env("DISCORD_TOKEN");
	pub static ref EMOJI_SERVER: String = load_env("EMOJI_SERVER");
	pub static ref PREFIX: String = load_env("PREFIX");
	pub static ref EMOJI_COPIES: usize = {
		let copies = load_env("EMOJI_COPIES")
			.parse::<usize>()
			.expect("EMOJI_COPIES must be a number");
		if copies <= 0 {
			panic!("EMOJI_COPIES must be 1 or greater");
		}
		copies
	};
}
