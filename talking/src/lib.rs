pub fn talking(text: &str) -> &str {
    if text.is_empty() {
        "Just say something!"
    } else if text.to_uppercase() == text && text.ends_with('?') {
        "Quiet, I am thinking!"
    } else if text.to_uppercase() == text {
        "There is no need to yell, calm down!"
    } else if text.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }

}