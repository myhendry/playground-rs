use std::fmt::{Display, Formatter, Result as FmtResult, self};

/*
cannot move out of `*self` which is behind a shared reference
move occurs because `*self` has type `StatusCode`, which does not implement the `Copy` trait

as self exists on the Heap, we need to implement the Copy Trait

When we implement Copy Trait, it will force us to implement the Clone Trait

Its good practise to implement Debug so easier to log later
*/

#[derive(Clone, Copy, Debug)]
pub enum StatusCode {
	Ok = 200,
	BadRequest = 400,
	NotFound = 404,
}

impl StatusCode {
	pub fn reason_phrase(&self) -> &str {
		match self {
			Self::Ok => "Ok",
			Self::BadRequest => "Bad Request",
			Self::NotFound => "Not Found",
		}
	}
}

// implement Display Trait will easily allow to print out
// the status code number it corresponds to
impl Display for StatusCode {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", *self as u16)
	}
}

