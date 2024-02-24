use crate::constants::*;

#[derive(PartialEq)]
pub enum WordPart {
	Single(char),     // Single character
	Group(Vec<char>), // Group of characters (e.g. "[abc]" -> [a, b, c])
	SingleWildcard,   // Single wildcard character, "*"
	AnyWildcard,      // Any wildcard character, "**"
}

impl ToString for WordPart {
	fn to_string(&self) -> String {
		match self {
			WordPart::Single(character) => match character {
				&ASTERISK => format!("{ESCAPE}{ASTERISK}"),
				&GROUP_START => format!("{ESCAPE}{ASTERISK}"),
				_ => character.to_string(),
			},
			WordPart::Group(characters) => {
				format!("{GROUP_START}{0}{GROUP_END}", characters.iter().collect::<String>())
			}
			WordPart::SingleWildcard => ASTERISK.to_string(),
			WordPart::AnyWildcard => format!("{ASTERISK}{ASTERISK}"),
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
