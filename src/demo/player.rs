#[derive(Debug)]
pub struct Player<'a> {
	name: &'a str,
	body: Option<String>,
	age: u32,
}

impl<'a> Player<'a> {
	pub fn new(_name: &'a str, _body: Option<String>, _age: u32) -> Self {
		Self {
			name: _name,
			body: _body,
			age: _age,
		}
	}

	pub fn shout(&'a self) -> &'a str {
		let z = match &self.body {
			Some(b) => {b},
			None => "",
		};
		z
	}

	pub fn amend(&'a mut self) {
		self.age = 100;
		
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
