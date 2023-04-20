
pub fn rotate(input: &str, key: i8) -> String {
    let mut output = String::new();
    let key = key % 26;
    for c in input.chars() {
        match c {
            'a'..='z' => {
                let new_char = ((c as i8 - 'a' as i8 + key + 26) % 26 + 'a' as i8) as u8 as char;
                output.push(new_char);
            }
            'A'..='Z' => {
                let new_char = ((c as i8 - 'A' as i8 + key + 26) % 26 + 'A' as i8) as u8 as char;
                output.push(new_char);
            }
            _ => output.push(c),
        }
    }
    output
}