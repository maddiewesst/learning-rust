pub fn num_to_ordinal(x: u32) -> String {
    let last = x % 10;

    match last {
        1 => return x.to_string() + "st",
        2 => return x.to_string() + "nd",
        3 => return x.to_string() + "rd",
        _ => return x.to_string() + "th",
    }
}