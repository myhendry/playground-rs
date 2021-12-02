use std::{fmt::{Display, Formatter, Result as FmtResult}, net::{TcpStream, TcpListener}};
use std::io::{Write, Read, Result as IoResult};
use super::StatusCode;

#[derive(Debug)]
pub struct Response {
	status_code: StatusCode,
	// if the body content is small, it is fine to use String
	// but if the body content is huge, it is better to use string slices 
	// which is more efficient than using String because if use String,
	// it will be copied to the Heap
	body: Option<String>,
}

impl Response {
	pub fn new(status_code: StatusCode, body: Option<String>) -> Self {
		Self {
			status_code,
			body
		}
	}

	// implementing the Write Trait Object to make it more flexible - Generic
	// it can accept any parameters that implement the Write Trait

	// Static vs Dynamic Dispatch
	// dynamic dispatch uses 'dyn' which stands for dynamic eg dyn stream: &mut dyn Write
	// There will be run time cost when using dyn because it will search for the appropriate method to use during runtime
	// static dispatch uses 'impl' eg stream: &mut impl Write
	// it will resolve during compile time hence there is no run time cost associated with it contrast to dynamic dispatch
	// compilation will take longer and the file size will be larger

	pub fn send(&self, stream: &mut impl Write) -> IoResult<()> {
	
		// todo why need to use &self.body here and not just self?
		let body = match &self.body {
			Some(b) => b,
			None => "",
		};

		// Here, we are writing directly to the stream
	    	write!(stream, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)	
	}
}

// Here, we are writing to the formatter first
// then from the formatter, we use the write macro to write to the stream
// using send method above, we write directly to the stream
// impl Display for Response {
// 	fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
// 		let body = match &self.body {
// 			Some(b) => b,
// 			None => "",
// 		};

// 	    	write!(f, "HTTP/1.1 {} {}\r\n\r\n{}", self.status_code, self.status_code.reason_phrase(), body)
// 	}
// }