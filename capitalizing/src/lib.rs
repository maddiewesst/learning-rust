
pub fn title_case(input: &str) -> String{
	let mut result = String::new();

	let parts = input.split_whitespace();
	let collection: Vec<&str> = parts.collect();

	for s in collection {
		let mut v: Vec<char> = s.chars().collect();
		v[0] = v[0].to_uppercase().nth(0).unwrap();
		for c in v.iter() {
			result.push(*c);
		}
		result.push(' ');
	}
	result
}

pub fn change_case(input: &str) -> String {
	let mut result = String::new();

	for c in input.chars() {
		if c.is_ascii_lowercase() {
			result.push(c.to_ascii_uppercase());
		} else if c.is_ascii_uppercase() {
			result.push(c.to_ascii_lowercase());
		} else {
			result.push(c);
		}

	}
	result
}