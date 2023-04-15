use std::collections::HashMap;

pub fn bigger(h: HashMap<&str, i32>) -> i32 {

    let mut bv: i32 = 0;
    for (_, value) in h.iter() {
        if value > &bv {
            bv = *value
        }
    }
    bv
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut hash = HashMap::new();
        hash.insert("Daniel", 122);
        hash.insert("Ashley", 333);
        hash.insert("Katie", 334);
        hash.insert("Robert", 14);

        println!("The biggest of the elements in the HashMap is {}", bigger(hash));
    }
}
