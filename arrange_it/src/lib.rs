pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<&str> = phrase.split(' ').collect();
    words.sort_by_key(|word| word.chars().find(|c| c.is_digit(10)).unwrap());
    words
        .iter()
        .map(|word| word.chars().filter(|c| !c.is_digit(10)).collect::<String>())
        .collect::<Vec<String>>()
        .join(" ")
}

fn main() {
    println!("{:?}", arrange_phrase("is2 Thi1s T4est 3a"));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_arrange_phrase() {
        assert_eq!(arrange_phrase("is2 Thi1s T4est 3a"), "This is a Test");
        assert_eq!(arrange_phrase("4of Fo1r pe6ople g3ood th5e the2"), "For the good of the people");
        assert_eq!(arrange_phrase("6 2 4 3 5 1"), "1 2 3 4 5 6");
        assert_eq!(arrange_phrase(""), "");
    }
}