use std::{error::Error, fmt::{Display, Debug, Formatter, Result as FmtResult}, convert::TryFrom, str::Utf8Error};

#[derive(Debug)]
pub struct Request2<'a> {
	pub path: &'a str,
	pub query_string: Option<String>,
	pub method: Method2,
}

// impl<'a> TryFrom<&'a [u8]> for Request2<'a> {
// 	type Error = ParseError2;

// 	// pass in buffer
// 	// GET /search?name=abc&sort=1 HTTP/1.1
//     	// TryFrom Trait CAN fail hence may return error that needs to be handled unlike From Trait
// 	fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
// 		// below is same as above and is what is returned with a lifetime 'buf
//     		// fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> { ...
//        		// pub fn from_utf8(v: &[u8]) -> Result<&str, Utf8Error>

//        		// ! Verbose Way 1
//         	// match str::from_utf8(buf) {
//         	//     Ok(request) => {},
//         	//     Err(_) => return Err(ParseError::InvalidEncoding),
//         	// }

//         	// ! Verbose Way 1
//         	// match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
//         	//   Ok(request) => {},
//         	//   Err(e) => return Err(e),   
//         	// }

// 		/*
// 			let request: Result<&str, Utf8Error> = std::str::from_utf8(buf);
// 			let request: &str = std::str::from_utf8(buf)?;

// 			todo how are we handling the Utf8Error here if we use ? operator
// 			Solution: `?` couldn't convert the error to `ParseError2`
// 			the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
// 			required because of the requirements on the impl of `FromResidual<Result<Infallible, Utf8Error>>` for `Result<Request2<'_>, ParseError2>`
// 		*/

// 		let request = std::str::from_utf8(buf)?;

// 		// GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
// 		/*
// 			fn get_next_word(request: &str) -> Option<(&str, &str)>
// 			let z: Option<(&str, &str)> = get_next_word(request);
// 			let z: Result<(&str, &str), ParseError2> = get_next_word(request).ok_or(ParseError2::InvalidRequest);
// 			let z: (&str, &str) = get_next_word(request).ok_or(ParseError2::InvalidRequest)?;

// 			todo how to know when to use ok_or combinators or how to know when to use which combinators at any given moment?
// 		*/
// 		let (method, request) = get_next_word(request).ok_or(ParseError2::InvalidRequest)?;
// 		let (mut path, request) = get_next_word(request).ok_or(ParseError2::InvalidRequest)?;
// 		let (protocol, _) = get_next_word(request).ok_or(ParseError2::InvalidRequest)?;

// 		if protocol != "HTTP/1.1" {
// 			return Err(ParseError2::InvalidProtocol)
// 		}

		


// 	}
// }

pub fn get_next_word(request: &str) -> Option<(&str, &str)> {
	// ! Approach 1: Using chars()
 	// let mut iter = request.chars();
	// loop {
	//     let item = iter.next();
	//     match item {
	//         Some(c) => {},
	//         None => break
	//     }
	// }
	for (i, c) in request.chars().enumerate() {
		if c == ' ' || c == '\r' {
			return Some((&request[..i], &request[i+1..]))
		}
	}
	None
}

// #[derive(Debug)]
pub enum ParseError2 {
	InvalidRequest,
	InvalidEncoding,
	InvalidProtocol,
	InvalidMethod,
}

impl ParseError2 {
	fn message(&self) -> &str {
		match self {
			Self::InvalidRequest => "Invalid Request",
			Self::InvalidEncoding => "Invalid Encoding",
			Self::InvalidMethod => "Invalid Method",
			Self::InvalidProtocol => "Invalid Protocol",
		}	
	}
}

// By implementing this Error Trait, it will force us to implement the Debug and Display Trait which is a good practise
impl<'a> Error for ParseError2 {}

impl<'a> Display for ParseError2 {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		// write!(&mut w, "formatted {}", "arguments")?;
		write!(f, "{}", self.message())		
	}
}

/*
	implement the missing item: `fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }`: `fn fmt(&self, _: &mut Formatter<'_>) -> Result<(), std::fmt::Error> { todo!() }
*/
impl<'a> Debug for ParseError2 {
	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
		write!(f, "{}", self.message())
	}
}

/*
	not all trait items implemented, missing: `from`
	missing `from` in implementationrustcE0046
	request2.rs(64, 1): implement the missing item: `fn from(_: T) -> Self { todo!() }`: `fn from(_: T) -> Self { todo!() }
*/
impl From<Utf8Error> for ParseError2 {
	fn from(_: Utf8Error) -> Self { 
		Self::InvalidEncoding
	 }
}

#[derive(Debug)]
pub enum Method2 {
	GET,
	POST,
	DELETE,
	PUT,
	PATCH
}



