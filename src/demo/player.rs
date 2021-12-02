pub struct Player<'a> {
	name: &'a str,
	body: Option<String>,
}

impl<'a> Player<'a> {
	pub fn new(_name: &'a str, _body: Option<String>) -> Self {
		Self {
			name: _name,
			body: _body,
		}
	}

	pub fn shout(&'a self) -> &'a str {
		let z = match &self.body {
			Some(b) => {b},
			None => "",
		};
		z
	}
}

pub struct Teacher<'a> {
	name: &'a str,
	age: u32,
}

impl<'a> Teacher<'a> {
	pub fn new(name: &'a str, age: u32) -> Self {
		Self {
			name,
			age
		}
		
	}

	pub fn introduce(&self) -> &'a str {
		self.name
	}

	pub fn hey(&self) -> String {
		format!("Hey! {}", self.name)
	}


}
