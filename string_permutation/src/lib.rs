use std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {

    let mut count1: HashMap<char, usize> = HashMap::new();
    let mut count2: HashMap<char, usize> = HashMap::new();

    for word in s1.chars().into_iter() {
        if count1.contains_key(&word) {
            *count1.entry(word).or_insert(0) += 1;
            continue;
        }
        count1.insert(word, 1);
    }

    for word in s2.chars().into_iter() {
        if count2.contains_key(&word) {
            *count2.entry(word).or_insert(0) += 1;
            continue;
        }
        count2.insert(word, 1);
    }

    return count1 == count2;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let word = "thought";
        let word1 = "thougth";
        println!(
            "Is `{}` a permutation of `{}`? = {}",
            word,
            word1,
            is_permutation(word, word1)
        );
    }
}
