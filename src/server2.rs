use std::{net::TcpListener, io::Read, convert::TryFrom};

use crate::demo::Request2;

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
	pub fn run(self) {
		println!("{:?}", self.addr);

		// todo why use &self in this case? why can't use &self at run(&self) and use self here
		let listener = TcpListener::bind(&self.addr).unwrap();

		loop {
			/*
				stream TcpStream { addr: 127.0.0.1:3000, peer: 127.0.0.1:51517, fd: 4 } 
				addr 127.0.0.1:51517
			*/
 			// *1 Accept Listener -> Receive TcpStream & SocketAddr
			match listener.accept() {

				Ok((mut stream, addr)) => {
					println!("stream {:?} addr {:?}",stream, addr);
					
					let mut buffer = [0; 1024];
					
					// *2 TcpStream read mutable buffer
					match stream.read(& mut buffer) {
						Ok(_) => {
							// todo difference between utf8 vs utf16
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
							match Request2::try_from(&buffer[..]) {
								Ok(request) => {},
								Err(e) => println!("{:?}", e),
							}
							
						},
						Err(e) => println!("{:?}", e)
					}
				},
				Err(e) => println!("{:?}", e),
			};
		}
		
	}
}