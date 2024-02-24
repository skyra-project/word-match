use napi::Error;

use crate::{sentence::Sentence, word::Word};

#[napi]
pub struct WordGroup {
	words: Vec<Word>,
}

#[napi]
impl WordGroup {
	#[napi(constructor)]
	pub fn new(entries: Vec<String>) -> Result<Self, Error> {
		let mut words: Vec<Word> = Vec::with_capacity(entries.len());
		for entry in entries {
			words.push(Word::new(entry)?);
		}

		Ok(Self { words })
	}

	#[napi(getter, js_name = "words")]
	pub fn get_words(&self) -> Vec<Word> {
		self.words.clone()
	}

	#[napi]
	pub fn matches(&self, sentence: &mut Sentence) -> bool {
		let mut matched = false;
		// FIXME: This does not work correctly
		for (start, end) in sentence.indexes.iter() {
			if sentence.checked[*start] {
				continue;
			}

			for word in self.words.iter() {
				// TODO: This needs to accept spans
				if let Some(range) = word.matches(sentence.contents[*start..=*end].iter().collect::<String>().as_str())
				{
					matched = true;
					for i in range.start..range.end {
						sentence.checked[i] = true;
					}
				}
			}
		}

		matched
	}
}
