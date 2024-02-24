extern crate napi_build;

use std::{
	env,
	fs::File,
	io::{BufWriter, Write},
	path::Path,
};

fn main() {
	napi_build::setup();

	let path = Path::new(&env::var("OUT_DIR").unwrap()).join("codegen.rs");
	let mut file = BufWriter::new(File::create(path).unwrap());

	let mut map = phf_codegen::Map::<char>::new();

	let data = include_str!("./data/confusables.txt")
		.lines()
		.filter(|l| !l.is_empty())
		.map(|l| l.split_whitespace().collect::<Vec<_>>())
		.map(|l| (l[0], l[1]))
		.map(|(target, characters)| {
			let characters = characters.chars().collect::<Vec<_>>();
			(characters, target)
		});

	for (source, target) in data {
		let target = &format!("r\"{target}\"");
		for source in source {
			let _ = map.entry(source, target);
		}
	}

	write!(
		&mut file,
		"#[allow(clippy::unicode_not_nfc, clippy::unreadable_literal)]
    static KEYWORDS: phf::Map<char, &'static str> = {};",
		map.build()
	)
	.unwrap();
}
