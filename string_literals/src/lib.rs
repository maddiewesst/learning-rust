pub fn is_empty(v: &str) -> bool {
    v.is_empty()
}

pub fn is_ascii(v: &str) -> bool {
    v.is_ascii()
}

pub fn contains(v: &str, pat: &str) -> bool {
    v.contains(pat)
}


pub fn split_at(v: &str, index: usize) -> (&str, &str) {
    let (one, two) = v.split_at(index);
    let tup: (&str, &str)= (one, two);
    return tup;
     
 
 }
 
 pub fn find(v: &str, pat: char) -> usize {  
     for (i, item) in v.chars().enumerate() {
         if item == pat {
             return i;
         }
     }
     return v.len();
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = split_at("rust", 2);
        assert_eq!(result, ("ru", "st"));
    }
}
