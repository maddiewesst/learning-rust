
#[derive(Debug, Clone, Eq, PartialEq)]
pub struct CipherError {
    pub validation: bool,
    pub expected: String
}
impl CipherError {
    pub fn new(validation: bool, expected: String) -> CipherError {
        CipherError {
            validation: validation,
            expected: expected,
        }

    }
}
pub fn cipher(original: &str, ciphered: &str) -> Option<Result<bool, CipherError>> {

    if original == "" || ciphered == "" {
        return None;
    }

    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let reversed_alphabet = "zyxwvutsrqponmlkjihgfedcba";
    let mut cipher = String::new();
    
    for c in original.chars() {
        match alphabet.find(c.to_ascii_lowercase()) {
            Some(i) => {
                let cipher_char = reversed_alphabet.chars().nth(i).unwrap();
                if c.is_ascii_uppercase() {
                    cipher.push(cipher_char.to_ascii_uppercase());
                } else {
                    cipher.push(cipher_char)
                }
            }
            None => cipher.push(c),
        }
    }

    if cipher == ciphered {
        Some(Ok(true))
    } else {
        Some(Err(CipherError::new(false, cipher)))
    }
    

}

