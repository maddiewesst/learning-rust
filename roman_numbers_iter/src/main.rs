use roman_numbers_iter::RomanNumber;

fn main() {
	let mut number = RomanNumber::from(1);

	println!("{:?}", number);
	println!("{:?}", number.next());
}