use std::net::TcpListener;

pub struct Server2 {
	// todo ok to use &str in this case?
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

		
		
	}
}