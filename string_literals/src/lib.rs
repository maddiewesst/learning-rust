pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    match v {
        pat => return true;
    }
}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    (one, two) = v.to_string().split_at(index)
}

pub fn find(v: &str, pat: char) -> usize {  
    for (i, &item) in bytes.iter().enumerate() {
        if item == pat {
            return i;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
