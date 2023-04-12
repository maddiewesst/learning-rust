pub fn first_subword(mut s: String) -> String {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b'_' || item.is_ascii_uppercase() && i != 0 {
            return s[..i].to_string();
        }
    }
 
  s

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
    let s1 = String::from("helloWorld");
	let s2 = String::from("snake_case");
	let s3 = String::from("CamelCase");
	let s4 = String::from("just");
        let result = first_subword(s1);
        assert_eq!(result, "hello");
    }
}