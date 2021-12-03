use std::convert::{TryFrom};

use std::net::TcpListener;
use std::io::{Read, Write};
use crate::http::request::ParseError;
use crate::http::{ Request, Response, StatusCode};


// ! L59 Traits can provide for default implementations
// in event caller does not implement
pub trait Handler {
	fn handle_request(&mut self, request: &Request) -> Response;
	
	fn handle_bad_request(&mut self, e: &ParseError) -> Response {
		println!("Failed to parse request: {}", e);
		Response::new(StatusCode::BadRequest, None)
	}
}

pub struct Server {
	addr: String,
}


impl Server {
	pub fn new(addr: String) -> Self {
		Self {
			addr
		}
	}

	pub fn run(&self, mut handler: impl Handler) {
		// Unwrap will panic if no result found and will log error to the screen
		// If unwrap ok, it will return the result
	 	// If double binding (connect to address already in use), this line will panic
		// -> io::Result<TcpListener>
		let listener = TcpListener::bind(&self.addr).unwrap();

		// loop through LISTENER
		// if ok, get a STREAM	
		// create a BUFFER
		// STREAM will read a mutable BUFFER
		// BUFFER (&[u8] aka utf8 byte slice) will be converted into REQUEST (try_from) (&str)
		loop {
			// -> io::Result<(TcpStream, SocketAddr)>
			match listener.accept() {
				Ok((mut stream, _addr)) => {
					let mut buffer = [0; 1024];
					// use std::io::{Read} trait needs to be pulled in before stream can read from buffer
					// -> Result<usize>
 					match stream.read(&mut buffer) {
						Ok(_) => {
							println!("Received a request: {}", String::from_utf8_lossy(&buffer));

							// L38 15:00
							// convert byte slice to request
							// rust std library got many modules to assist in type conversion
							// From Trait must not fail. If conversion can fail, then use TryFrom	
							// If conversion fails using TryFrom, it can be managed in a controlled way			
								
							// Need to 
							// Need to convert to byte slice as in this case try from implementation is generic. It cannot figure out what type as its using generic otherwise it can automatically convert
							// First Way: Request::try_from(&buffer as &[u8]); 
							// Second Way: Request::try_from(&buffer[..]);
							/*
							Error if use Request::try_from(&buffer);
							the trait bound `Request<'_>: From<&[u8; 1024]>` is not satisfied
							required because of the requirements on the impl of `Into<Request<'_>>` for `&[u8; 1024]`rustcE0277
							request.rs(14, 12): required because of the requirements on the impl of `TryFrom<&[u8; 1024]>` for `Request<'_>`
							mod.rs(477, 5): required by `try_from`
							*/
							// Need to pull in the trait to use it
							// use std::convert::TryFrom;
							/*
							Second way to convert byte slice buffer into request since implementing TryFrom trait gives the try_into for free
							type annotations needed
							cannot satisfy `_: TryFrom<&[u8]>`
							required because of the requirements on the impl of `TryInto<_>` for `&[u8]`rustcE0283
							server.rs(67, 9): this method call resolves to `Result<T, <Self as TryInto<T>>::Error>`
							*/	
							// &buffer[..].try_into();			
							// let res: &Result<Request, _> = &buffer[..].try_into();			
							// this will create a relationship between buffer and request
							// request will have a relationship with the buffer
							// lifetime tool to guarantee memory safety
							// communicate to the compiler some references are related to the same memory and are expected to share the same lifetimes
							// Specifying a lifetime does not allow us to choose how long a value will live
							let response = match Request::try_from(&buffer[..]) {
								Ok(request) => {
									// Here is write! to the stream
									// For response to implement the write macro to write to the stream, 
									// it needs to implement the Display Trait
									// write!(TARGET, PLACEHOLDER,CONTENT);  
									// write!(stream, "{}", response);
									 
									// Both below methods return an IoResult which needs to be handled
									// It would be nicer if they can be handled together and no need to duplicate
									// response.send(&mut stream);

									// ! L55 Using Custom Traits
									// Response::new(StatusCode::Ok, None)
									handler.handle_request(&request)
								},
								Err(e) => { 
									println!("Failed to parse the request: {}", e);

									/*
										unused `Result` that must be used
										`#[warn(unused_must_use)]` on by default
										this `Result` may be an `Err` variant, which should be handled		
									*/
									// Response::new(StatusCode::BadRequest, None).send(&mut stream);
									
									// ! L55 Using Custom Traits
									// Response::new(StatusCode::BadRequest, None)
									handler.handle_bad_request(&e)


								}
							};
							
							if let Err(e) = response.send(&mut stream) {
								println!("Failed to send response: {}", e);
							}
						},
						Err(e) => println!("Failed to read from connection: {:?}", e),
					}
				},
				Err(e) => println!("Failed to establish a connection: {}", e),
				}
		}


	}
}