use std::convert::{TryFrom};

use std::net::TcpListener;
use std::io::{Read};
use crate::http::{ Request};

pub struct Server {
	addr: String,
}

impl Server {
	pub fn new(addr: String) -> Self {
		Self {
			addr
		}
	}

	pub fn run(&self) {
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
							match Request::try_from(&buffer[..]) {
								Ok(_) => {},
								Err(e) => println!("Failed to parse the request: {}", e),
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