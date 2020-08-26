use crate::error::EmojiError;
use regex::Regex;
use std::convert::{TryFrom, TryInto};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub struct EmojiValue(pub u8, pub u8);

impl Into<String> for EmojiValue {
	fn into(self) -> String {
		format!("slimroll_{}_c{}", self.0, self.1)
	}
}

impl TryFrom<String> for EmojiValue {
	type Error = EmojiError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		use EmojiError::*;
		let re = Regex::new(r"slimroll_(\d+)_c(\d+)")?;
		let captures = re.captures(&value).ok_or(CouldNotParseEmojiName)?;
		let num = captures
			.get(1)
			.ok_or(CouldNotParseEmojiName)?
			.as_str()
			.parse::<u8>()
			.or(Err(CouldNotParseEmojiName))?;
		let copy = captures
			.get(2)
			.ok_or(CouldNotParseEmojiName)?
			.as_str()
			.parse::<u8>()
			.or(Err(CouldNotParseEmojiName))?;

		Ok(Self(num, copy))
	}
}
