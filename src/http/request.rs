use std::convert::TryFrom;
use std::str::{self, Utf8Error};
use std::fmt::{Debug, Display, Formatter, Result as FmtResult};
use std::error::Error;
use super::method::{Method, MethodError};

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
		// convert string slice to Method enum
		// when use the FromStr Trait, it provides the parse method for free
		let method: Method = method.parse()?;

		// + 1 means plus one byte not one character
		// we know at this index, there is a question mark which is one byte so it is fine and wouldnt throw an error
		let mut query_string = None;

		// * OPTION 1
		// match  path.find('?') {
		// 	Some(i) => {
		// 		query_string = Some(&path[i+1..]);
		// 		path = &path[..i]
		// 	},
		// 	None => {}
		// }

		// * OPTION 2
		// let q = path.find("?");
		// if q.is_some() {
		// 	// unwrap will panic which is not elegant
		// 	// but because we have wrapped it with is_some(), it is fine
		// 	// cause we have done the check
		// 	let i = q.unwrap();
		// 	query_string = Some(&path[i+1..]);
		// 	path = &path[..i]

		// } 

		// OPTION 3 (BEST SOLUTION)
		// we use if Let because we only care about the Some arm and not care about the None arm
		// Rust has the if Let to make it easier for coders
		if let Some(i) = path.find('?') {
			query_string = Some(&path[i+1..]);
			path = &path[..i]	
		}

		// Drawbacks of passing as Strings rather than string slice
		// though returning values as string slices might mean need to implement lifetimes
		// More efficient
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

// ! used inside str::from_utf8(buf)?;
impl From<Utf8Error> for ParseError {
	fn from(_: Utf8Error) -> Self {
		Self::InvalidEncoding
	}
}

// ! used inside let method: Method = method.parse()?;
impl From<MethodError> for ParseError {
	fn from(_: MethodError) -> Self {
		Self::InvalidMethod	
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

