pub fn capitalize_first(input: &str) -> String {
   if input.is_empty() {
    return input
   }
	format!("{}{}", input.chars().next().unwrap().to_uppercase(), 
	input.chars().skip(1).collect::<String>())
    
}


pub fn title_case(input: &str) -> String {
	// let mut result = String::new();
	let mut newvec: Vec<String> = Vec::new();

	let parts = input.split_whitespace();
	let collection: Vec<&str> = parts.collect();

	for s in collection {
		if !s.is_empty() {
        let mut v: Vec<char> = s.chars().collect();
		v[0] = v[0].to_uppercase().nth(0).unwrap();
		let newstring: String = v.into_iter().collect();
		newvec.push(newstring);
    }
	
	}

	let join = newvec.join(" ");
	join
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