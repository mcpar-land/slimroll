use crate::error::EmojiError;
use regex::Regex;
use std::convert::{TryFrom, TryInto};

#[derive(Hash, PartialEq, Eq, Clone, Copy, Debug)]
pub enum EmojiValue {
	Negative,
	Num(u8, u8),
}

impl Into<String> for EmojiValue {
	fn into(self) -> String {
		match self {
			Self::Negative => "slimroll_neg".to_string(),
			Self::Num(n, c) => format!("slimroll_{}_c{}", n, c),
		}
	}
}

impl TryFrom<String> for EmojiValue {
	type Error = EmojiError;

	fn try_from(value: String) -> Result<Self, Self::Error> {
		use EmojiError::*;
		let re = Regex::new(r"slimroll_(?:(\d)_c(\d+)|(neg))")?;
		let captures = re.captures(&value).ok_or(CouldNotParseEmojiName)?;
		if captures.get(3).is_some() {
			Ok(Self::Negative)
		} else {
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
			Ok(Self::Num(num, copy))
		}
	}
}

impl TryFrom<&str> for EmojiValue {
	type Error = EmojiError;

	fn try_from(value: &str) -> Result<Self, Self::Error> {
		Self::try_from(value.to_string())
	}
}

#[cfg(test)]
mod test {
	use super::*;
	use std::convert::TryFrom;

	#[test]
	fn test_from_string() {
		assert_eq!(
			EmojiValue::try_from("slimroll_6_c5").unwrap(),
			EmojiValue::Num(6, 5)
		);
		assert_eq!(
			EmojiValue::try_from("slimroll_2_c20").unwrap(),
			EmojiValue::Num(2, 20)
		);
		assert_eq!(
			EmojiValue::try_from("slimroll_neg").unwrap(),
			EmojiValue::Negative
		);

		assert_eq!(EmojiValue::try_from("slifhgskjf").is_err(), true);
		assert_eq!(EmojiValue::try_from("climroll_10_c2").is_err(), true);
		let a: String = EmojiValue::Num(3, 8).into();
		assert_eq!("slimroll_3_c8".to_string(), a);
		let b: String = EmojiValue::Negative.into();
		assert_eq!("slimroll_neg", b);
	}
}
