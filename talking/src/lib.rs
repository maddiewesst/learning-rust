pub fn talking(text: &str) -> &str {
    
    let trimmed = text.trim();
    let mut caps = false;

    for c in text.chars().filter(|c| c.is_alphabetic()) {
        if !c.is_uppercase() {
            caps = false;
            break;
        } else {
        caps = true;
        }
    }

    if trimmed.is_empty() {
        "Just say something!"
    } else if caps && text.ends_with('?') {
        "Quiet, I am thinking!"
    } else if caps {
        "There is no need to yell, calm down!"
    } else if text.ends_with('?') {
        "Sure."
    } else {
        "Interesting"
    }

}