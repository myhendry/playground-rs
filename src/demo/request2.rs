use std::{error::Error, fmt::{Display, Formatter}, convert::TryFrom};

#[derive(Debug)]
pub struct Request2<'a> {
	pub path: &'a str,
	pub query_string: Option<String>,
	pub method: Method2,
}

// impl<'a> TryFrom<&'a [u8]> for Request2<'a> {
// 	type Error = ParseError2;

// 	// pass in buffer
// 	fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {

// 	}
// }

pub enum ParseError2 {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

// impl<'a> Error for Request2<'a> {
	
// }

// impl<'a> Display for Request2<'a> {
// 	fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), String> {
// 		write!()		
// 	}
// }

#[derive(Debug)]
pub enum Method2 {
	GET,
	POST,
	DELETE,
	PUT,
	PATCH
}



