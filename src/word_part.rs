use std::fmt::{Display, Formatter, Result};

use crate::constants::*;

#[derive(PartialEq)]
pub enum WordPart {
	Single(char),     // Single character
	Group(Vec<char>), // Group of characters (e.g. "[abc]" -> [a, b, c])
	SingleWildcard,   // Single wildcard character, "*"
	AnyWildcard,      // Any wildcard character, "**"
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
	pub fn matches(&self, character: char) -> bool {
		match self {
			WordPart::Single(c) => *c == character,
			WordPart::Group(chars) => chars.contains(&character),
			WordPart::SingleWildcard => true,
			WordPart::AnyWildcard => true,
		}
	}
}
