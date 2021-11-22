use std::str::{self, FromStr};

#[derive(Debug)]
pub enum Method {
	GET,
	PUT,
	PATCH,
	DELETE,
}

// Need to implement this Trait FromStr to use the parse method
impl FromStr for Method {	
	type Err = MethodError;

	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"GET" => Ok(Self::GET),
			"PUT" => Ok(Self::PUT),
			"PATCH" => Ok(Self::PATCH),
			"DELETE" => Ok(Self::DELETE),
			_ => Err(MethodError),
		}
	}
}

// todo Why use empty struct for MethodError?
pub struct MethodError;