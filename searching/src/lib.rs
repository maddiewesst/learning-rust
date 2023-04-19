


pub fn search(array: &[i32], key: i32) -> Option<usize> {
   
    for (i, val) in array.iter().enumerate() {
        if *val == key {
            return Some(i);
        } 
    }

    return None;

}


