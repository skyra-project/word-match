include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub trait Confusable {
	fn contains_confusables(&self) -> bool;
	fn replace_confusables(&self) -> String;
}

impl Confusable for char {
	fn contains_confusables(&self) -> bool {
		KEYWORDS.get(self).is_some()
	}

	fn replace_confusables(&self) -> String {
		match KEYWORDS.get(self) {
			Some(value) => value.to_string(),
			None => self.to_string(),
		}
	}
}

impl Confusable for String {
	fn contains_confusables(&self) -> bool {
		self.chars().any(|c| c.contains_confusables())
	}

	fn replace_confusables(&self) -> String {
		self.chars().map(|c| c.replace_confusables()).collect()
	}
}
