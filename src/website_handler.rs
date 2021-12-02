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
}

impl Handler for WebsiteHandler {
	fn handle_request(&mut self, request: &Request) -> Response {
		//Response::new(StatusCode::Ok, Some("<h1>TEST</h1>".to_string()))
		// ! L57 Routing Incoming Requests	
		match request.method() {
			Method::GET => match request.path() {
				"/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
				"/about" => Response::new(StatusCode::Ok, Some("<h1>About</h1>".to_string())),
				_ => Response::new(StatusCode::NotFound, None),
			},
			_ => Response::new(StatusCode::NotFound, None),
		}
	}
}