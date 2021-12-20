use std::{net::TcpListener, io::Read, convert::TryFrom};

use crate::demo::{Request2, request2::{ParseError2, Response2, StatusCode2}};

pub trait Handler2 {
	fn handle_request(&self, request: &Request2) -> Response2;

	// Using Default but can be overrided
	fn handle_bad_request(&mut self, e: &ParseError2) -> Response2 {
		println!("Failed to parse request: {}", e);
		Response2::new(StatusCode2::BadRequest, None)
	}
}

pub struct Server2 {
	// todo ok to use &str in this case?
	// todo why use String?
	pub addr: String,
}

impl Server2 {
	pub fn new(addr: String) -> Self {
		Self {
			addr
		}
	}

	// todo why not &self in this case?
	// it does not matter in this case as we are not using struct Server after the run function. Hence, its okay for run to take ownership of struct Server
	pub fn run(self, mut handler: impl Handler2) {
		println!("{:?}", self.addr);

		// todo why use &self in this case? why can't use &self at run(&self) and use self here
		let listener = TcpListener::bind(&self.addr).unwrap();

		loop {
			/*
				! Possible Way but not the best way to handle enum variable in rust
				but not appropriate if there are many arms
			*/
			// let res = listener.accept();

			// if res.is_err() {
			// 	continue;
			// }

			// let (stream, addr) = res.unwrap();
			/*
				stream TcpStream { addr: 127.0.0.1:3000, peer: 127.0.0.1:51517, fd: 4 } 
				addr 127.0.0.1:51517
			*/
 			// *1 Accept Listener -> Receive TcpStream & SocketAddr
			match listener.accept() {

				Ok((mut stream, _)) => {
					// TcpStream { addr: 127.0.0.1:8080, peer: 127.0.0.1:57270, fd: 4 } addr 127.0.0.1:57270
					// println!("stream {:?} addr {:?}",stream, addr);
					
					let mut buffer = [0; 1024];
					
					// *2 TcpStream read mutable buffer
					// Read is a Trait - to use the functionality of read, need to pull in the Read Trait
					match stream.read(& mut buffer) {
						Ok(_) => {
							// todo difference between utf8 vs utf16
							// GET /style.css HTTP/1.1
							println!("Received a request {}", String::from_utf8_lossy(&buffer));

							/*
								*implement TryFrom Trait
								default seems to have the from and into methods

								if TryFrom Trait not implemented, the trait bound `Request2<'_>: From<&[u8]>` is not satisfied
								required because of the requirements on the impl of `Into<Request2<'_>>` for `&[u8]`
								required because of the requirements on the impl of `TryFrom<&[u8]>` for `Request2<'_>`
								
								&buffer[..]  	// &[u8]
								&buffer		// &[u8; 1024]

								`ParseError2` doesn't implement `Debug`
								the trait `Debug` is not implemented for `ParseError2`
								add `#[derive(Debug)]` to `ParseError2` or manually `impl Debug for ParseError2`

							*/
							// *3 Convert Buffer into Request
							// either return a Request if successful or a ParseError if unsuccessful
							let response = match Request2::try_from(&buffer[..]) {
								Ok(request) => {
									// dbg!(_request);
									// ! Not using Traits
									// Response2::new(StatusCode2::Ok, Some("hi".to_string()))
									
									// ! Using Traits
									handler.handle_request(&request)
								},
								// try_from Error
								Err(e) => {
									// ! Not using Trait
									// println!("Failed to parse request: {}", e);
									//Response2::new(StatusCode2::BadRequest, None);
									
									// ! Using Traits
									handler.handle_bad_request(&e)
								}
								
							};

							/*
								A specialized [Result] type for I/O operations.
								This type is broadly used across [std::io] for any operation which may produce an error.
								This typedef is generally used to avoid writing out [io::Error] directly and is otherwise a direct mapping to [Result].
	
							*/
							if let Err(e) = response.send(&mut stream) {
								println!("Failed to send response: {}", e);
							}
							
						},
						// readstream Error
						Err(e) => println!("{:?}", e)
					}
				},
				Err(e) => println!("{:?}", e),
			};
		}
		
	}
}