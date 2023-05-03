pub fn delete_prefix<'a>(prefix: &'a str, s: &'a str) -> Option<&'a str> {
    s.strip_prefix(prefix)
}



// pub fn delete_prefix(prefix: &'static str, s: &'static str) -> Option<&'static str> {
//     s.strip_prefix(prefix)
// }