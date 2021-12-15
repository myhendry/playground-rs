use std::fs;
use crate::{demo::request2::{Response2, StatusCode2}, server2::Handler2};
use super::demo::{Request2, Method2};

pub struct WebsiteHandler2 {
	// todo why use String in this case?
	// todo why not use &str since public_path is  not changing
	public_path: String
}

impl WebsiteHandler2 {
	pub fn new(public_path: String) -> Self {
		Self {
			public_path,
		}
	}

	fn read_file(&self, file_path: &str) -> Option<String> {
		let path = format!("{}/{}", self.public_path, file_path);	
		// security authentication
		// fs::read_to_string(path).ok()
		match fs::canonicalize(path) {
			Ok(path) => {
				if path.starts_with(&self.public_path) {
					fs::read_to_string(path).ok()
				} else {
					println!("Directory Traversal Attack Attempted: {}", file_path);
					None
				}
			}
			Err(_) => None,
		}	
	}
}

impl Handler2 for WebsiteHandler2 {
	fn handle_request(&self, request: &Request2) -> Response2 {
		match request.method() {
			Method2::GET => match request.path() {
				// "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
				"/" => Response2::new(StatusCode2::Ok, self.read_file("index.html")),
				// "/hello" => Response::new(StatusCode::Ok, Some("<h1>Hello</h1>".to_string())),
				"/hello" => Response2::new(StatusCode2::Ok, self.read_file("about.html")),
				// _ => Response::new(StatusCode::NotFound, None),	
				path => match self.read_file(path) {
					Some(contents) => Response2::new(StatusCode2::Ok, Some(contents)), 
					None => Response2::new(StatusCode2::NotFound, None), 
				},
			},
			_ => Response2::new(StatusCode2::NotFound, None),
		}
	}
}