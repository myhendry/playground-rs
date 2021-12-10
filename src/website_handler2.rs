use std::fs;
use super::demo::{Request2};

pub struct WebsiteHandler2 {
	// todo why use String in this case?
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
		unimplemented!()	
	}
}

impl Handler for WebsiteHandler {
	fn handle_request(&self, request: &Request) -> Response {
			
		match request.method() {
					
		}
	}
}