pub fn talking(text: &str) -> &str {
    let trimmed = text.trim();
    if trimmed.is_empty() {
        "Just say something!"
    } else if trimmed.to_uppercase() == trimmed && text.ends_with('?') {
        "Quiet, I am thinking!"
    } else if trimmed.to_uppercase() == trimmed {
        "There is no need to yell, calm down!"
    } else if text.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }

}