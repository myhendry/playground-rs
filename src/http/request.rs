use std::convert::TryFrom;
use std::str::{self, Utf8Error};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::error::Error;
use super::method::Method;

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
		// str::from_utf8 convert byte slice to string slice
		// 2 possible ways but longer hence use the ? operator
		/*	
			Alternative 1
			match str::from_utf8(buf) {
				Ok(request) => {},
				Err(_) => return Err(ParseError::InvalidEncoding)
			}	
		
			Alternative 2	
			pub fn or<F>(self, res: Result<T, F>) -> Result<T, F>
			match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
				Ok(request) => {},
				Err(e) => return Err(e),
			}
		*/
		let request = str::from_utf8(buf)?;
		/*
		Transforms the Option<T> into a Result<T, E>, mapping Some(v) to Ok(v) and None to Err(err).
		*/
		// get_next_word(request) returns an Option and can be converted to a Result using ok_or
		// pub fn ok_or<E>(self, err: E) -> Result<T, E>
		/*
			Alternative: Using match	
			match get_next_word(request) {
				Some((method, request)) => {},
				None => return Err(ParseError::InvalidRequest),
			}
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
	// can also use for loop but is more verbose
	/*
		let mut iter = request.chars();
		loop {
			let item = iter.next();
			match item {
				Some(c) => {},
				None => break,
			}
		}
	*/
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

impl Error for ParseError { } // By implementing this Error Trait, it will force us to implement the Debug and Display Trait which is a good practise

// implement Error Trait will force us to implement the Display and the Debug Trait
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

