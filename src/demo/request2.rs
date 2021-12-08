use std::{error::Error, fmt::{Display, Debug, Formatter, Result as FmtResult}, convert::TryFrom, str::{Utf8Error, FromStr}};

#[derive(Debug)]
pub struct Request2<'a> {
	pub path: &'a str,
	pub query_string: Option<String>,
	pub method: Method2,
}

 impl<'a> TryFrom<&'a [u8]> for Request2<'a> {
	type Error = ParseError2;

	// pass in buffer
	// GET /search?name=abc&sort=1 HTTP/1.1
    	// TryFrom Trait CAN fail hence may return error that needs to be handled unlike From Trait
	fn try_from(buf: &'a [u8]) -> Result<Self, Self::Error> {
		// below is same as above and is what is returned with a lifetime 'buf
    		// fn try_from(buf: &'buf [u8]) -> Result<Request<'buf>, Self::Error> { ...
       		// pub fn from_utf8(v: &[u8]) -> Result<&str, Utf8Error>

       		// ! Verbose Way 1
        	// match str::from_utf8(buf) {
        	//     Ok(request) => {},
        	//     Err(_) => return Err(ParseError::InvalidEncoding),
        	// }

        	// ! Verbose Way 1
        	// match str::from_utf8(buf).or(Err(ParseError::InvalidEncoding)) {
        	//   Ok(request) => {},
        	//   Err(e) => return Err(e),   
        	// }

		/*
			let request: Result<&str, Utf8Error> = std::str::from_utf8(buf);
			let request: &str = std::str::from_utf8(buf)?;

			todo how are we handling the Utf8Error here if we use ? operator
			Solution: `?` couldn't convert the error to `ParseError2`
			the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
			required because of the requirements on the impl of `FromResidual<Result<Infallible, Utf8Error>>` for `Result<Request2<'_>, ParseError2>`
		*/

		// ! Succint Way 1
 		let request = std::str::from_utf8(buf)?;

 		// GET /search?name=abc&sort=1 HTTP/1.1\r\n...HEADERS...
 		/*
			fn get_next_word(request: &str) -> Option<(&str, &str)>
			let z: Option<(&str, &str)> = get_next_word(request);
			let z: Result<(&str, &str), ParseError2> = get_next_word(request).ok_or(ParseError2::InvalidRequest);
			let z: (&str, &str) = get_next_word(request).ok_or(ParseError2::InvalidRequest)?;

			todo how to know when to use ok_or combinators or how to know when to use which combinators at any given moment?
		*/

	        // ! Verbose Way 2
	        // match get_next_word(request) {
	        //     Some((method, request)) => {},
	        //     None => return Err(ParseError::InvalidRequest),
	        // };
		// ! Succint Way 2
		let (method, request) = get_next_word(request).ok_or(ParseError2::InvalidRequest)?;
		let (mut path, request) = get_next_word(request).ok_or(ParseError2::InvalidRequest)?;
		let (protocol, _) = get_next_word(request).ok_or(ParseError2::InvalidRequest)?;

		if protocol != "HTTP/1.1" {
			return Err(ParseError2::InvalidProtocol)
		}

		// Convert Method String to Method Enum. Because use trait FromStr, we get parse() method for free
		/*
			Parses this string slice into another type - in this case Enum Type.
			Because parse is so general, it can cause problems with type inference. As such, parse is one of the few times you'll see the syntax affectionately known as the 'turbofish': ::<>. This helps the inference algorithm understand specifically which type you're trying to parse into.
			parse can parse into any type that implements the FromStr trait.
		*/
		/*
			`?` couldn't convert the error to `ParseError2`
			the question mark operation (`?`) implicitly performs a conversion on the error value using the `From` trait
			the following implementations were found:
			<ParseError2 as From<Utf8Error>>
			required because of the requirements on the impl of `FromResidual<Result<Infallible, MethodError2>>` for `Result<Request2<'_>, ParseError2>`

			Convert Method String to Method Enum. Because use trait FromStr, we get parse() method for free
		*/
		// todo why involve the ParseError2 here and not just MethodError2?
		// because in this case, this function's Error Type is ParseError2 and not MethodError2
		// hence need to convert MethodError2 to ParseError2
		// todo is it always use ? to 'unwrap' Result?	
		let method: Method2 = method.parse()?;

		let mut query_string = None;


        // !Verbose Way 3: Unnecessary to have the None arm since it returns an empty () anyway 3
        // match path.find('?') {
        //     Some(i) => {
        //         query_string = Some(&path[i+1..]);
        //         path = &path[..i];
        //     },
        //     None => {},
        // }
        
        // !Verbose Way 3: Better way compared to above but still verbose 3
        // let q = path.find('?');
        // if q.is_some() {
        //     let i = q.unwrap();
        //     query_string = Some(&path[i+1..]);
        //     path = &path[..i]; 
        // }

        // !Better Way 3: since care only about a single variant of it
        if let Some(i) = path.find('?') {
                query_string = Some(QueryString::from(&path[i + 1..]));
                path = &path[..i];
        }

        Ok(Self {
            path,
            query_string,
            method,
        })


	}
}
	


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

/*
	not all trait items implemented, missing: `from`
	missing `from` in implementationrustcE0046
	request2.rs(159, 1): implement the missing item: `fn from(_: T) -> Self { todo!() }`: `fn from(_: T) -> Self { todo!() }
`
*/
impl From<MethodError2> for ParseError2 {
	fn from(_: MethodError2) -> Self {
		Self::InvalidMethod
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

/*
	Parse a value from a string
	FromStr's [from_str] method is often used implicitly, through str's [parse] method. See [parse]'s documentation for examples.
	FromStr does not have a lifetime parameter, and so you can only parse types that do not contain a lifetime parameter themselves. 
	In other words, you can parse an i32 with FromStr, but not a &i32. You can parse a struct that contains an i32, but not one that contains an &i32.
*/

impl FromStr for Method2 {
	type Err = MethodError2;
	
	fn from_str(s: &str) -> Result<Self, Self::Err> {
		match s {
			"GET" => Ok(Self::GET),
			"DELETE" => Ok(Self::DELETE),
			"POST" => Ok(Self::POST),
			"PUT" => Ok(Self::PUT),
			"PATCH" => Ok(Self::PATCH),
			_ => 	Err(MethodError2),
		}

	}
}

// todo why use an empty struct here?
pub struct MethodError2;



