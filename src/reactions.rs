use crate::emojis::{self, emoji_value::EmojiValue, EMOJIS};
use crate::env_vars::EMOJI_COPIES;
use serenity::{model::prelude::*, prelude::*};
use std::collections::HashMap;

pub enum Digit {
	Negative,
	Num(u8),
}

impl Digit {
	fn emoji_value(&self, copy: u8) -> EmojiValue {
		match self {
			Self::Negative => EmojiValue::Negative,
			Self::Num(num) => EmojiValue::Num(*num, copy),
		}
	}
}

/// Turn a number into a vector of its individual digits
pub fn digit_vec(number: i64) -> Vec<Digit> {
	let mut digits: Vec<Digit> = vec![];
	digits.extend::<Vec<Digit>>(
		number
			.to_string()
			.chars()
			.map(|c| {
				if c == '-' {
					Digit::Negative
				} else {
					Digit::Num(c.to_string().parse::<u8>().unwrap())
				}
			})
			.collect(),
	);
	digits
}

pub fn emojis_for_number(number: i64, max: usize) -> Option<Vec<EmojiValue>> {
	// turn a number into a vector of all its digits
	let digits: Vec<Digit> = digit_vec(number);

	let mut values: Vec<EmojiValue> = vec![];

	// create a running counter of all the counts of all the digits
	let mut counts: [u8; 10] = [0; 10];
	for i in 0..=9u8 {
		let count = digits
			.iter()
			.filter(|d| match d {
				Digit::Negative => false,
				Digit::Num(num) => num == &(i as u8),
			})
			.count();
		if count > max || (i == 0 && count > max - 1) {
			return None;
		}
	}
	for d in digits {
		match d {
			Digit::Negative => values.push(EmojiValue::Negative),
			Digit::Num(num) => {
				counts[num as usize] += 1;
				values.push(EmojiValue::Num(num, counts[num as usize] - 1));
			}
		}
	}

	Some(values)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_emojis_for_number() {
		assert_eq!(
			emojis_for_number(3655613, 5),
			Some(vec![
				EmojiValue::Num(3, 0),
				EmojiValue::Num(6, 0),
				EmojiValue::Num(5, 0),
				EmojiValue::Num(5, 1),
				EmojiValue::Num(6, 1),
				EmojiValue::Num(1, 0),
				EmojiValue::Num(3, 1)
			])
		);
		assert_eq!(
			emojis_for_number(-20, 5),
			Some(vec![
				EmojiValue::Negative,
				EmojiValue::Num(2, 0),
				EmojiValue::Num(0, 0)
			])
		);
		assert_eq!(
			emojis_for_number(99999, 5),
			Some(vec![
				EmojiValue::Num(9, 0),
				EmojiValue::Num(9, 1),
				EmojiValue::Num(9, 2),
				EmojiValue::Num(9, 3),
				EmojiValue::Num(9, 4),
			])
		);
		assert_eq!(emojis_for_number(999999, 5), None);
		assert_eq!(
			emojis_for_number(123456789, 1),
			Some(vec![
				EmojiValue::Num(1, 0),
				EmojiValue::Num(2, 0),
				EmojiValue::Num(3, 0),
				EmojiValue::Num(4, 0),
				EmojiValue::Num(5, 0),
				EmojiValue::Num(6, 0),
				EmojiValue::Num(7, 0),
				EmojiValue::Num(8, 0),
				EmojiValue::Num(9, 0),
			])
		);
		assert_eq!(emojis_for_number(11, 1), None);
	}
}
