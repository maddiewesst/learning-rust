pub fn is_pangram(s: &str) -> bool {

    for c in 'a'..'z' {
        if !s.to_lowercase().contains(c) {
            return false
        }
    }

    true
}