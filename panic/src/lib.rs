use std::fs::File;

pub fn open_file(s: &str) -> File {
    return File::open(s).expect("File not found");
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
