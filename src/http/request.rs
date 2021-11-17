use std::convert::TryFrom;
use std::str::{self, FromStr, Utf8Error};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

#[derive(Debug)]
pub struct Request<'buf> {
	path: &'buf str,
	pub query_string: Option<&'buf str>,
	pub method: Method,
}

impl<'buf> Request<'buf> {

}

impl<'buf> TryFrom<&'buf [u8]> for Request<'buf> {
	type Error = ParseError;

	fn try_from(buf: &[u8]) -> Result<Self, Self::Error> {
		// Result<&str, Utf8Error>
		// Putting ?, returns &str
		// convert utf8 &[u8] to &str
		// GET /fsdfadf HTTP/1.1

		// The ? operator can only be used on Result, NOT Option
		let request = str::from_utf8(buf)?;
		/*
		Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err).
		*/
		let (method, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		let (mut path, request) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;
		let (protocol, _) = get_next_word(request).ok_or(ParseError::InvalidRequest)?;

		if protocol != "HTTP/1.1" {
			return Err(ParseError::InvalidProtocol);
		}

		// todo why need to parse method?
		let method = method.parse()?;

		Ok(Self {
			path: "fasdf",
			method: Method::GET,
			query_string: Some("fasdfadf"),
		})
	}
}

// GET /fsdfadf HTTP/1.1
fn get_next_word(request: &str) -> Option<(&str, &str)> {
	for (i, c) in request.chars().enumerate() {
		if c == ' ' || c == '\r' {
			return Some((&request[..i], &request[i+1..]));
		} 
	}
	None
}


pub enum ParseError {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

impl ParseError {
	pub fn message(&self) -> &str {
		match self {
			Self::InvalidRequest => "Invalid Request",
			Self::InvalidEncoding => "Invalid Encoding",
			Self::InvalidMethod => "Invalid Method",
			Self::InvalidProtocol => "Invalid Protocol",
		}
	}
}

// !
impl From<Utf8Error> for ParseError {
	fn from(_: Utf8Error) -> Self {
		Self::InvalidEncoding
	}
}

impl Display for ParseError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

impl Debug for ParseError {
	fn fmt(&self, f: &mut Formatter) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

// todo why need to implement From<MethodError>, From<Utf8Error>, Display and Debug for ParseError?

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