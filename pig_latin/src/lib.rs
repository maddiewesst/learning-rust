pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let mut result = String::new();
    for word in text.split_whitespace() {
        let chars: Vec<char> = word.chars().collect();
        let mut consonants = Vec::new();
        let mut i = 0;
        while i < chars.len() && !vowels.contains(&chars[i]) {
            if i+2 < chars.len() && chars[i+1] == 'q' && chars[i+2] == 'u' {
                consonants.push(chars[i]);
                consonants.push(chars[i+1]);
                consonants.push(chars[i+2]);
                i += 3;
            } else {
                consonants.push(chars[i]);
                i += 1;
            }
        }
        if i == 0 {
            result.push_str(&word);
            result.push_str("ay");
        } else {
            result.push_str(&word[i..]);
            for c in consonants {
                result.push(c);
            }
            result.push_str("ay");
        }
        result.push(' ');
    }
    result.pop();  // remove trailing space
    result
}