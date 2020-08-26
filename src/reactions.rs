use crate::emojis::{self, emoji_value::EmojiValue, EMOJIS};
use crate::env_vars::EMOJI_COPIES;
use serenity::{model::prelude::*, prelude::*};
use std::collections::HashMap;

/// Turn a number into a vector of its individual digits
pub fn digit_vec(number: u64) -> Vec<u8> {
	number
		.to_string()
		.chars()
		.map(|c| c.to_string().parse::<u8>().unwrap())
		.collect()
}

/// Turn a number into an array containing the occurences of each digit
pub fn digit_counts(number: u64) -> [u8; 10] {
	let digits: Vec<u8> = digit_vec(number);

	let mut counts: [u8; 10] = [0; 10];
	for i in 0..=9usize {
		let count = digits.iter().filter(|cha| cha == &&(i as u8)).count();
		counts[i] = count as u8;
	}
	counts
}

pub fn emojis_for_number(number: u64, max: usize) -> Option<Vec<EmojiValue>> {
	// turn a number into a vector of all its digits
	let digits: Vec<u8> = digit_vec(number);

	let mut values: Vec<EmojiValue> = vec![];

	// create a running counter of all the counts of all the digits
	let mut counts: [u8; 10] = [0; 10];
	for i in 0..=9u8 {
		let count = digits.iter().filter(|cha| cha == &&(i as u8)).count();
		if count > max {
			return None;
		}
	}
	for d in digits {
		counts[d as usize] += 1;
		values.push(EmojiValue(d, counts[d as usize] - 1));
	}

	Some(values)
}

#[cfg(test)]
mod test {
	use super::*;

	#[test]
	fn test_digit_counts() {
		assert_eq!(digit_counts(11223456), [0, 2, 2, 1, 1, 1, 1, 0, 0, 0]);
		assert_eq!(digit_counts(11002299338844775566), [2; 10]);
		assert_eq!(digit_counts(000000001), [0, 1, 0, 0, 0, 0, 0, 0, 0, 0]);
	}

	#[test]
	fn test_emojis_for_number() {
		assert_eq!(
			emojis_for_number(3655613, 5),
			Some(vec![
				EmojiValue(3, 0),
				EmojiValue(6, 0),
				EmojiValue(5, 0),
				EmojiValue(5, 1),
				EmojiValue(6, 1),
				EmojiValue(1, 0),
				EmojiValue(3, 1)
			])
		);
		assert_eq!(
			emojis_for_number(99999, 5),
			Some(vec![
				EmojiValue(9, 0),
				EmojiValue(9, 1),
				EmojiValue(9, 2),
				EmojiValue(9, 3),
				EmojiValue(9, 4),
			])
		);
		assert_eq!(emojis_for_number(999999, 5), None);
		assert_eq!(
			emojis_for_number(1234567890, 1),
			Some(vec![
				EmojiValue(1, 0),
				EmojiValue(2, 0),
				EmojiValue(3, 0),
				EmojiValue(4, 0),
				EmojiValue(5, 0),
				EmojiValue(6, 0),
				EmojiValue(7, 0),
				EmojiValue(8, 0),
				EmojiValue(9, 0),
				EmojiValue(0, 0),
			])
		);
		assert_eq!(emojis_for_number(11, 1), None);
	}
}
