pub struct Player<'a> {
	name: &'a str,
}

impl<'a> Player<'a> {
	pub fn new(_name: &'a str) -> Self {
		Self {
			name: _name,
		}
	}

	pub fn shout(&'a self) -> &'a str {
		self.name
	}
}