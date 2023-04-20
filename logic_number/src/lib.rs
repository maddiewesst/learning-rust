pub fn number_logic(num: u32) -> bool {
    let num_str = num.to_string();
    let count = num_str.len();
    let mut sum = 0;

    for c in num_str.chars() {
        let digit = c.to_digit(10).unwrap();
        sum += digit.pow(count as u32);
    }

    sum == num

}