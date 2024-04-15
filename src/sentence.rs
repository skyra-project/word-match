use std::fmt;

use napi::{Error, Result};

use crate::confusables::Confusable;

#[napi]
#[derive(PartialEq)]
pub enum Boundary {
	/// The start of a new word.
	Start,
	/// The contents of a word.
	Word,
	/// The end of a word.
	End,
	/// A single character that is both the start and the end of a word.
	Mixed,
	/// A non-word character, which can be skipped, such as punctuation, spaces,
	/// or newlines.
	NoContent,
	/// A character that has been marked
	Marked,
}

impl Boundary {
	pub(crate) fn is_start(&self) -> bool {
		matches!(self, Boundary::Start | Boundary::Mixed)
	}

	pub(crate) fn is_end(&self) -> bool {
		matches!(self, Boundary::End | Boundary::Mixed)
	}
}

#[derive(Clone)]
#[napi(object)]
pub struct ToCensoredStringOptions {
	/// The character to use to censor the marked characters, defaults to `'*'`.
	pub character: Option<String>,
	/// The original sentence before it was sanitized and lowercased.
	pub original: String,
}

/// A struct that represents a sentence. The sentence is split into spans that
/// are checked by the `Word` class. The spans are updated when a word is
/// checked, and the indexes are updated to keep track of the words that have
/// been checked.
#[napi]
pub struct Sentence {
	/// The spans of the sentence that will be checked by the `Word` class. The
	/// entries are tuples that represent the start and end of the span, which
	/// will be used to work in "sub-sentences".
	///
	/// When the sentence is created, the `spans` vector is initialized with a
	/// single span that covers the entire sentence (0, `contents.len()`). When
	/// a word is checked, the span is split into two spans, one that covers the
	/// start of the sentence and the other that covers the end.
	///
	/// If only non-word characters are between two spans, they merge into a
	/// single span. Spans are at least one character long. Surrounding non-word
	/// characters are included in the span.
	///
	/// For example, if the sentence is "Pepe ate a banana", the `spans` vector
	/// will be initialized with a single span (0, 17).
	///
	/// If the word "ate" is checked, the `spans` vector will be updated to the
	/// spans:
	/// - `(0, 3)` "Pepe"
	/// - `(9, 17)` "a banana"
	///
	/// If the word "Pepe" is checked, the `spans` vector will be updated to the
	/// spans:
	/// - `(9, 17)` "a banana"
	///
	/// The `spans` vector is also always sorted by the start of the span.
	pub(crate) spans: Vec<(usize, usize)>,
	/// A vector of `Boundary` that represent the boundaries of the words in the
	/// sentence.
	///
	/// This vector is updated when a span is marked, updating the surrounding
	/// boundaries to reflect the start and end of a word.
	///
	/// For example, if the sentence is "A Pepe", the `boundaries` vector will
	/// be initialized with the boundaries:
	/// - `Mixed` "A"
	/// - `NoContent` " "
	/// - `Start` "P"
	/// - `Word` "e"
	/// - `Word` "p"
	/// - `End` "e"
	///
	/// If the word "Pepe" is checked, the `boundaries` vector will be updated
	/// to the boundaries:
	/// - `Mixed` "A"
	/// - `NoContent` " "
	/// - `Marked` "P"
	/// - `Marked` "e"
	/// - `Marked` "p"
	/// - `Marked` "e"
	///
	/// Similarly, if the slice "Pep" is checked, the `boundaries` vector will
	/// be updated to the boundaries:
	/// - `Mixed` "A"
	/// - `NoContent` " "
	/// - `Marked` "P"
	/// - `Marked` "e"
	/// - `Marked` "p"
	/// - `Mixed` "e"
	///
	/// This becomes useful when checking for words that are prefixes, suffixes,
	/// or infixes of other words.
	pub boundaries: Vec<Boundary>,
	/// A vector of characters that represent the sanitized contents of the
	/// sentence. They are lowercased and have confusables replaced with their
	/// base characters.
	pub(crate) contents: Vec<char>,
}

#[napi]
impl Sentence {
	#[napi(constructor)]
	pub fn new(sentence: String) -> Self {
		let sentence = sentence.replace_confusables().to_lowercase();
		let mut boundaries: Vec<Boundary> = Vec::with_capacity(sentence.len());
		let contents: Vec<char> = sentence.chars().collect();

		let mut chars = sentence.chars().peekable();
		while let Some(c) = chars.next() {
			// If the character is a whitespace or control character, the boundary is no
			// content.
			if !c.is_alphanumeric() {
				boundaries.push(Boundary::NoContent);
				continue;
			};

			// Read the following characters until the end of `sentence` or until a
			// whitespace or control character is found.
			//
			// The first character is always the start of a word, so the boundary is set to
			// start. Then, we scan the following characters until a whitespace or control
			// character is found.
			let mut boundary = Boundary::Start;
			while let Some(c) = chars.peek() {
				if !c.is_alphanumeric() {
					break;
				}

				// There is a word character, push the previous character's boundary, increase
				// the word size, and set the boundary to `Word`.
				boundaries.push(boundary);
				boundary = Boundary::Word;
				chars.next();
			}

			// Process the last character from the loop.
			boundaries.push(if boundary == Boundary::Start {
				// If the boundary is the start, the size is 1, and the character is a word,
				// therefore the boundary is mixed.
				Boundary::Mixed
			} else {
				// If the size is greater than 1, the boundary is a word.
				Boundary::End
			});
		}

		Self { spans: vec![(0usize, contents.len())], boundaries, contents }
	}

	/// Gets the length of the array. This is a number one higher than the
	/// highest index in the array.
	#[napi(getter)]
	pub fn length(&self) -> u32 {
		self.contents.len() as u32
	}

	/// Returns the contents of the sentence as a string.
	///
	/// @remarks
	///
	/// This method does not return the original sentence, but ts sanitized
	/// and lowercased contents.
	#[napi(js_name = "toString")]
	pub fn js_to_string(&self) -> String {
		self.to_string()
	}

	/// Returns the contents of the sentence as a string, censoring the marked
	/// characters with the provided character.
	///
	/// @param options - The options to use when censoring the marked
	/// characters.
	///
	/// @returns The contents of the sentence with the marked characters
	/// censored.
	///
	/// @example
	///
	/// ```ts
	/// const original = "Pepe ate a banana";
	/// const sentence = new Sentence(original);
	/// const word = new Word("Pepe");
	///
	/// word.check(sentence);
	/// sentence.toCensoredString({ character: "X", original });
	/// // ⇒ "XXXX ate a banana"
	/// ```
	///
	/// @remarks
	///
	/// The original sentence must have the same length as the sentence, or an
	/// error will be thrown.
	#[napi(js_name = "toCensoredString")]
	pub fn js_to_censored_string(&self, options: ToCensoredStringOptions) -> Result<String> {
		let mut out = String::with_capacity(self.contents.len());
		let character = options.character.unwrap_or('*'.to_string());
		let original = options.original;

		if original.len() != self.contents.len() {
			return Err(Error::from_reason("The original sentence must have the same length as the sentence"));
		}

		for (index, c) in original.chars().enumerate() {
			if self.boundaries[index] == Boundary::Marked {
				out.push_str(&character);
			} else {
				out.push(c);
			}
		}

		Ok(out)
	}

	/// Updates the state of the sentence to mark the start and end of a word.
	pub(crate) fn mark(&mut self, start: usize, end: usize) {
		debug_assert!(start < end, "start must be less than end");
		debug_assert!(end <= self.contents.len(), "end must be less than or equal to the length of the sentence");

		let (start, end) = self.mark_update_spans(start, end);
		self.mark_update_boundaries(start, end);
	}

	/// Updates the `spans` vector to mark the start and end of a word. If the
	/// word is surrounded by non-word characters, the span is expanded to
	/// include the surrounding characters.
	///
	/// If the expanded span is in contact with the previous or next span, the
	/// spans are merged into a single span.
	fn mark_update_spans(&mut self, start: usize, end: usize) -> (usize, usize) {
		// 1. Find the span that contains the word.
		let span_index = self.spans.iter().position(|(s, e)| *s <= start && end <= *e).unwrap();
		let (current_start, current_end) = self.spans[span_index];

		// 2. Expand the span to include non-word characters surrounding the matched
		//    slice within
		// the current span.
		let start = self
			.boundaries
			.iter()
			.enumerate()
			.take(start)
			.rposition(|(index, boundary)| index >= current_start && *boundary == Boundary::NoContent)
			.map_or(0, |i| i + 1);
		let end = self
			.boundaries
			.iter()
			.enumerate()
			.skip(end)
			.position(|(index, boundary)| index < current_end && *boundary == Boundary::NoContent)
			.map_or(self.contents.len(), |i| end + i);

		// 3. Merge with surrounding spans if applicable:
		// - Merge with the previous span if the end of the previous span is the start
		//   of the word.
		// - Merge with the next span if the start of the next span is the end of the
		//   word.
		//
		// If the span merges with the previous and next spans, all 3 are removed from
		// the `spans` vector and a new span is added to the `spans` vector.
		match (start == current_start && start != 0usize, end == current_end && end != self.contents.len()) {
			(true, true) => self.merge_span_both(span_index),
			(true, false) => self.merge_span_previous(span_index),
			(false, true) => self.merge_span_next(span_index),
			(false, false) => self.spans[span_index] = (start, end),
		};

		(start, end)
	}

	/// Merges the current span with the previous span.
	fn merge_span_previous(&mut self, index: usize) {
		let (_, end) = self.spans.remove(index);
		let (previous_start, _) = self.spans.remove(index - 1);
		self.spans.insert(index - 1, (previous_start, end));
	}

	/// Merges the current span with the next span.
	fn merge_span_next(&mut self, index: usize) {
		let (start, _) = self.spans.remove(index);
		let (_, next_end) = self.spans.remove(index);
		self.spans.insert(index, (start, next_end));
	}

	/// Merges the current span with the previous and next spans.
	fn merge_span_both(&mut self, index: usize) {
		let (_, next_end) = self.spans.remove(index + 1);
		self.spans.remove(index);
		let (previous_start, _) = self.spans.remove(index - 1);
		self.spans.insert(index - 1, (previous_start, next_end));
	}

	/// Updates the `boundaries` vector to set all characters within `start` and
	/// `end` as `Marked`. Boundaries surrounding the marked characters are
	/// updated to `Start`, `End`, or `Mixed`, where applicable.
	fn mark_update_boundaries(&mut self, start: usize, end: usize) {
		// 1. Update the boundaries of the marked characters.
		for boundary in &mut self.boundaries[start..end] {
			if *boundary == Boundary::NoContent {
				continue;
			}

			*boundary = Boundary::Marked;
		}

		// 2. Update the boundaries surrounding the marked characters.
		// 2.1. Update the boundary before the marked characters if applicable.
		if start > 0 {
			self.boundaries[start - 1] = match self.boundaries[start - 1] {
				// 2.1.1. If the character before the marked characters is a start or mixed boundary,
				// the update will make it a single-character word, making it a `Mixed` boundary.
				Boundary::Start | Boundary::Mixed => Boundary::Mixed,
				// 2.1.2. If the character before the marked characters is a word boundary, the end
				// boundary of the word is moved to this position, making it an `End` boundary.
				Boundary::Word => Boundary::End,
				// 2.1.3. Preserve existing `End`, `NoContent`, and `Marked` boundaries.
				boundary => boundary,
			};
		}

		// 2.2. Update the boundary after the marked characters if applicable.
		if end < self.boundaries.len() {
			self.boundaries[end] = match self.boundaries[end] {
				// 2.2.1. If the character after the marked characters is an end or mixed boundary,
				// the update will make it a single-character word, making it a `Mixed` boundary.
				Boundary::End | Boundary::Mixed => Boundary::Mixed,
				// 2.2.2. If the character after the marked characters is a word boundary, the start
				// boundary of the word is moved to this position, making it a `Start` boundary.
				Boundary::Word => Boundary::Start,
				// 2.2.3. Preserve existing `Start`, `NoContent`, and `Marked` boundaries.
				boundary => boundary,
			};
		}
	}
}

impl fmt::Display for Sentence {
	fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
		write!(f, "{}", self.contents.iter().collect::<String>())
	}
}
