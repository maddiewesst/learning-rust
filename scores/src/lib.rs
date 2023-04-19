

pub fn score(s: &str) -> u64 {
    let mut result: u64 = 0;

    for c in s.to_uppercase().chars() {
        match c {
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => result += 1,
            'D' | 'G' => result += 2,	
            'B' | 'C' | 'M' | 'P' => result += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => result += 4,
            'K' => result += 5,
            'J' | 'X' => result += 8,
            'Q' | 'Z' => result += 10,
            _ => continue,
        };
    }
    result
    

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(score("a"), 1);
        assert_eq!(score("ThiS is A Test"), 14);
    }
}