use std::fmt::{Display, Formatter, Result};

use crate::constants::*;

#[derive(PartialEq, Clone)]
pub enum WordPart {
	/// Represents a single character.
	Single(char),
	/// Represents a group of characters (e.g. "[abc]" -> [a, b, c])  as a
	/// vector of lowercase deduplicated characters.
	///
	/// ### Example
	///
	/// - `"[abc]"` matches `"a"`, `"b"`, or `"c"`, as well as combinations of
	///   them such as `"ab"` or `"bc"`, but not letters that aren't on the list
	///   such as `"t"` or `"z"`.
	Group(Vec<char>),
	/// Represents a single wildcard character, "*", which matches a single
	/// character.
	///
	/// ### Remarks
	///
	/// If the wildcard is at the beginning of the word, it will disable the
	/// left-side boundary check. Likewise, if the wildcard is at the end of the
	/// word, it will disable the right-side boundary check.
	///
	/// ### Example
	///
	/// - `"a*t"` matches `"abt"` or `"acct"` but not `"acbt"`.
	/// - `"*t"` matches `"t"` or `"ct"` but not `"ctv"`.
	/// - `"a*"` matches `"a"` or `"ac"` but not `"cab"`.
	/// - `"*a*"` matches any word containing `"a"`.
	SingleWildcard,
	/// Represents any wildcard character, "**", which matches any character.
	///
	/// ### Example
	///
	/// - `"a**t"` matches `"abt"`, `"acct"`, or `"acbt"`.
	AnyWildcard,
}

impl Display for WordPart {
	fn fmt(&self, f: &mut Formatter<'_>) -> Result {
		match self {
			WordPart::Single(character) => match *character {
				ASTERISK => write!(f, "{ESCAPE}{ASTERISK}"),
				GROUP_START => write!(f, "{ESCAPE}{ASTERISK}"),
				_ => write!(f, "{character}"),
			},
			WordPart::Group(characters) => {
				write!(f, "{GROUP_START}{0}{GROUP_END}", characters.iter().collect::<String>())
			}
			WordPart::SingleWildcard => write!(f, "{ASTERISK}"),
			WordPart::AnyWildcard => write!(f, "{ASTERISK}{ASTERISK}"),
		}
	}
}

impl WordPart {
	pub fn matches(&self, character: char, previous_character: Option<char>) -> bool {
		match self {
			WordPart::Single(c) => *c == character,
			WordPart::Group(chars) => chars.contains(&character),
			WordPart::SingleWildcard => match previous_character {
				Some(c) => c == character,
				None => true,
			},
			WordPart::AnyWildcard => true,
		}
	}
}
