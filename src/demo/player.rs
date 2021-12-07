#[derive(Debug)]
pub struct Player<'a> {
	name: &'a str,
	body: Option<String>,
	pub profile: Profile,
}

#[derive(Debug)]
pub struct Profile {
	age: u32,
	nickname: String,
}

impl Profile {
	pub fn amend(& mut self, new_age: u32, new_nickname: &str) {
		self.age = new_age;
		self.nickname =  format!("{} {}", self.nickname, new_nickname);
		
	}
}

impl<'a> Player<'a> {
	pub fn new(_name: &'a str, _body: Option<String>, _age: u32, _nickname: String) -> Self {
		Self {
			name: _name,
			body: _body,
			profile: Profile {
				age: _age,
				nickname: _nickname,
			},
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
