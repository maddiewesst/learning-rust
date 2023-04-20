pub fn num_to_ordinal(x: u32) -> String {
    let last = x % 10;

    match last {
        1 if x != 11 => return x.to_string() + "st",
        2 if x != 12 => return x.to_string() + "nd",
        3 if x != 13 => return x.to_string() + "rd",
        _ => return x.to_string() + "th",
    }
}