use std::char;

use napi::{Error, Status};
use napi_derive::napi;

use crate::{constants::*, word_part::*};

#[napi]
pub struct Word {
	parts: Vec<WordPart>,
	bound_left: bool,
	bound_right: bool,
}

#[napi]
pub struct WordMatch {
	start: usize,
	end: usize,
}

#[napi]
impl WordMatch {
	#[napi(getter, enumerable = true, js_name = "start")]
	pub fn get_start(&self) -> u32 {
		self.start.try_into().unwrap()
	}

	#[napi(getter, enumerable = true, js_name = "end")]
	pub fn get_end(&self) -> u32 {
		self.end.try_into().unwrap()
	}
}

#[napi]
impl Word {
	#[napi(constructor)]
	pub fn new(word: String) -> Result<Self, Error> {
		let mut parts: Vec<WordPart> = Vec::new();
		let mut chars = word.chars().peekable();

		while let Some(c) = chars.next() {
			// If '*' is found, it is a SingleWildcard
			// If '**' is found, it is an AnyWildcard
			// If '[' is found, it is the beginning of a Group, read until ']' and
			// deduplicated If '\' is found, it is an escape character, read the next
			// character Otherwise, it is a Single

			let part = match c {
				ASTERISK => {
					if chars.peek() == Some(&ASTERISK) {
						chars.next();
						WordPart::AnyWildcard
					} else {
						WordPart::SingleWildcard
					}
				}
				GROUP_START => {
					let mut group: Vec<char> = Vec::new();
					loop {
						match chars.next() {
							// If ']' is found, it is the end of the Group
							Some(GROUP_END) => {
								break;
							}
							// If '\' is found, it is an escape character, read the next character
							Some(ESCAPE) => {
								if let Some(c) = chars.next() {
									group.push(c);
								}
							}
							// If a character is found, add it to the Group
							Some(c) => {
								if !group.contains(&c) {
									group.push(c);
								}
							}
							// If the end of the word is reached, return an error
							None => {
								return Err(Error::new(Status::GenericFailure, "Unterminated character group"));
							}
						}
					}

					WordPart::Group(group)
				}
				ESCAPE => {
					if let Some(c) = chars.next() {
						WordPart::Single(c)
					} else {
						return Err(Error::new(
							Status::GenericFailure,
							"Escape character cannot be at the end of the word.",
						));
					}
				}
				_ => WordPart::Single(c),
			};

			parts.push(part);
		}

		if parts.is_empty() {
			return Err(Error::new(Status::GenericFailure, "The word cannot be empty."));
		}

		let bound_left = parts.first().unwrap() == &WordPart::SingleWildcard;
		if bound_left {
			parts.remove(0);
		}

		let bound_right = parts.last().unwrap() == &WordPart::SingleWildcard;
		if bound_right {
			parts.pop();
		}

		Ok(Word { parts, bound_left, bound_right })
	}

	#[napi]
	pub fn matches(&self, sentence: String) -> Option<WordMatch> {
		let mut chars = sentence.char_indices().peekable();

		while let Some((start, c)) = chars.next() {
			if self.parts[0].matches(c) {
				let mut part_index = 1;
				let mut word_index = 1;
				while let Some((end, c)) = chars.next() {
					if self.parts[word_index].matches(c) {
						word_index += 1;
						if word_index == self.parts.len() {
							return Some(WordMatch { start, end });
						}
					} else if self.parts[part_index].matches(c) {
						part_index += 1;
					} else {
						break;
					}
				}
			}
		}

		None
	}

	#[napi]
	pub fn to_string(&self) -> Result<String, Error> {
		let mut word = String::new();

		if self.bound_left {
			word.push('*');
		}

		for part in &self.parts {
			word.push_str(&part.to_string());
		}

		if self.bound_right {
			word.push('*');
		}

		Ok(word)
	}
}
