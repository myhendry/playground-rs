use std::{collections::HashMap };

#[derive(Debug)]
pub struct QueryString<'buf> {
	data: HashMap<&'buf str, Value<'buf>>,
}

#[derive(Debug)]
pub enum Value<'buf> {
	Single(&'buf str),
	Multiple(Vec<&'buf str>),
}

impl<'buf> QueryString<'buf> {
	pub fn get(&self, key: &str) -> Option<&Value> {
		self.data.get(key)
	}
}

/*
	FromStr cannot have lifetime parameters.
	It can only parse types that don't have a lifetime parameters
	Otherwise FromStr would have been able to convert string slices &str to other types
	From Trait allows lifetime parameters
	It can parse types that have lifetime parameters like structs and references

	In this case, we use the From Trait rather than from the Try_From Trait because
	this will not fail. The Try_From Trait is for conversion methods that can fail
*/
// a=1&b=2&c&d=&e===&d=7&d=abc
impl<'buf> From<&'buf str> for QueryString<'buf>{
	fn from(s: &'buf str) -> Self { 
		let mut data = HashMap::new();
		
		// split will return An iterator over substrings of this string slice, separated by characters matched by a pattern.
		for sub_str in s.split('&') {
			let mut key = sub_str;
			let mut val = ""; // for c, that does not have a value
			// Returns the byte index of the first character of this string slice that matches the pattern.
			if let Some(i) = sub_str.find('=') {
				key = &sub_str[..i];
				val = &sub_str[1+1..];
			}
			/* 
			Case 1:	if key exist and Value is of single variant, 
			we have to replace the value in the hashmap with the multiple variant 
			and we need to create a vector with the previous value from the single 
			and the new value

			Case 2: if the key already exist and it is already of the multiple type, 
			we just get the existing vector and push the new value to it
			simplest case

			Case 3: if the key does not exists, we need to insert a new value in the hashmap
			and it will be of the single type containing our Value
			*/

			// or_insert: handle case 3
			data.entry(key).and_modify(|existing | match existing {
				Value::Single(prev_val) => {
					// OPTION 1
					// let mut vec = Vec::new();
					// vec.push(val);
					// vec.push(prev_val);
					// ! OPTION 2 - more condense
					*existing = Value::Multiple(vec![prev_val, val]);
				},
				Value::Multiple(vec) => {vec.push(val)},
			} ).or_insert(Value::Single(val));


		}

		QueryString { data }
	}
}