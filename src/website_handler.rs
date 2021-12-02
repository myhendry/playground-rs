use std::fs;

use crate::http::Method;

use super::http::{Request, Response, StatusCode};

use super::server::Handler;

pub struct WebsiteHandler {
	public_path: String,
}

impl WebsiteHandler {
	pub fn new(public_path: String) -> Self {
		Self {
			public_path,
		}
	}

	fn read_file(&self, file_path: &str) -> Option<String> {
		let path = format!("{}/{}", self.public_path, file_path);

		/*
			! L60 Serving Arbitray Files
			Prevent Directory Traversal Attack by having path ../../../

		*/
		match fs::canonicalize(path) {
			Ok(path) => {
				if path.starts_with(&self.public_path) {
					fs::read_to_string(path).ok()
				} else {
					println!("Directory Traversal Attack Attempted: {}", file_path);
					None
				}
			},
			Err(_) => None,
		}
 
		/*
		! L59 Serving HTML Files
		Read the entire contents of a file into a string.

		This is a convenience function for using File::open and read_to_string with fewer imports 
		and without an intermediate variable. It pre-allocates a buffer based on the file size when available,
		so it is generally faster than reading into a string created with String::new().
		*/

		// ok() will look at the result. if its ok, it will get the ok and convert it into an Option. 
		// if error, it will discard the error and convert to None
		// fs::read_to_string(path).ok()
	}
}

impl Handler for WebsiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		//Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
		// ! L57 Routing Incoming Requests	
		match request.method() {
			Method::GET => match request.path() {
				"/" => Response::new(StatusCode::Ok, self.read_file("index.html")),
				"/about" => Response::new(StatusCode::Ok, self.read_file("about.html")),
				/*
					! L60 Serving Arbitray Files
					this implementation will lead to terrible security vulnerability
					this will lead to directory traversy vulnerability
					an attacker can reads arbitrary file from the system
					the 'path' can be anything the user sends
					a malicious user can practically read anything from our file system
				*/
				path => match self.read_file(path) {
					Some(contents) => Response::new(StatusCode::Ok, Some(contents)),
					None => Response::new(StatusCode::NotFound, None),
				},
				// _ => Response::new(StatusCode::NotFound, None),
			},
			_ => Response::new(StatusCode::NotFound, None),
		}
	}
}