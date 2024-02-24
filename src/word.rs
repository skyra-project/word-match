use std::char;

use napi::{Error, Status};

use crate::{confusables::Confusable, constants::*, word_part::*};

#[napi]
pub struct WordMatch {
	pub(crate) start: usize,
	pub(crate) end: usize,
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
#[derive(Clone)]
pub struct Word {
	parts: Vec<WordPart>,
	pub bound_left: bool,
	pub bound_right: bool,
}

#[napi]
impl Word {
	#[napi(constructor)]
	pub fn new(word: String) -> Result<Self, Error> {
		let word = word.replace_confusables().to_lowercase();
		let mut chars = word.chars().peekable();

		let mut parts: Vec<WordPart> = Vec::new();
		while let Some(c) = chars.next() {
			let part = match c {
				ASTERISK => {
					if chars.peek() == Some(&ASTERISK) {
						// Found '**':
						chars.next();
						WordPart::AnyWildcard
					} else {
						// Found '*':
						WordPart::SingleWildcard
					}
				}
				GROUP_START => {
					// Found '['
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
					// Found '\'
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

	pub fn matches(&self, sentence: &str) -> Option<WordMatch> {
		let mut chars = sentence.char_indices().peekable();

		while let Some((start, c)) = chars.next() {
			if self.parts[0].matches(c) {
				let mut part_index = 1;
				let mut word_index = 1;
				for (end, c) in chars.by_ref() {
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

	#[napi(js_name = "matches")]
	pub fn js_matches(&self, sentence: String) -> Option<WordMatch> {
		self.matches(sentence.as_str())
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
