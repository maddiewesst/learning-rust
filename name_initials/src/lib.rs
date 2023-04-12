pub fn initials(names: Vec<&str>) -> Vec<String> {
    let mut result: Vec<String> = Vec::new();

    for name in names.iter() {
        // let mut n: Vec<&str> = name.split(" ").collect();
        let parts  = name.split(' ').collect::<Vec<&str>>();
        let mut initials = String::new();
        initials.push(parts[0].chars().nth(0).unwrap());
        initials.push('.');
        initials.push(' ');
        initials.push(parts[1].chars().nth(0).unwrap());
        initials.push('.');
        result.push(initials.to_string());

    }
    return result;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];

        let result = initials(names);
        assert_eq!(result, ["H. P.", "S. E.", "J. L.", "B. O."]);
    }
}
