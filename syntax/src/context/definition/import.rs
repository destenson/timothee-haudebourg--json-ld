use locspan_derive::StrippedPartialEq;
use std::hash::Hash;

#[derive(Clone, Copy, PartialEq, StrippedPartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct Import;

impl Import {
	pub fn into_str(self) -> &'static str {
		"@import"
	}
}

impl Hash for Import {
	fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
		self.into_str().hash(state)
	}
}