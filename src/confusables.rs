include!(concat!(env!("OUT_DIR"), "/codegen.rs"));

pub trait Confusable {
	fn contains_confusable(&self) -> bool;
	fn replace_confusable(&self) -> String;
}

impl Confusable for char {
	fn contains_confusable(&self) -> bool {
		KEYWORDS.get(&self).is_some()
	}

	fn replace_confusable(&self) -> String {
		match KEYWORDS.get(&self) {
			Some(value) => value.to_string(),
			None => self.to_string(),
		}
	}
}

impl Confusable for String {
	fn contains_confusable(&self) -> bool {
		self.chars().any(|c| c.contains_confusable())
	}

	fn replace_confusable(&self) -> String {
		self.chars().map(|c| c.replace_confusable()).collect()
	}
}
