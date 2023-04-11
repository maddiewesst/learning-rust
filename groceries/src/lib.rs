pub fn insert(vec: &mut Vec<String>, val: String) {
    vec.push(val);
}

pub fn at_index(vec: &Vec<String>, index: usize) -> String {
    let vec = &vec[index];
    vec.to_string()
}