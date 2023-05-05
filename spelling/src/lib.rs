// Importing the `successors` iterator from the `iter` module of the `std` crate.
use std::iter::successors;

// Defining arrays of string literals to represent numbers from 0 to 19, multiples of ten from 0 to 90 and orders of magnitude (i.e. thousands, millions).
const ONES: [&str; 20] = [
    "zero",
    "one",
    "two",
    "three",
    "four",
    "five",
    "six",
    "seven",
    "eight",
    "nine",
    "ten",
    "eleven",
    "twelve",
    "thirteen",
    "fourteen",
    "fifteen",
    "sixteen",
    "seventeen",
    "eighteen",
    "nineteen",
];
const TENS: [&str; 10] = [
    "zero", "ten", "twenty", "thirty", "forty", "fifty", "sixty",
    "seventy", "eighty", "ninety",
];
const ORDERS: [&str; 3] = [
    "zero",
    "thousand",
    "million",
];

// Function that takes an integer as input and returns its English language equivalent.
pub fn spell(num: u64) -> String {
    match num {
        // For numbers between 0 and 19 (inclusive), return the corresponding string from the `ONES` array.
        0..=19 => ONES[num as usize].to_string(),
        // For numbers between 20 and 99 (inclusive), return the corresponding string by combining the strings from the `TENS` array with the strings from the `ONES` array.
        20..=99 => {
            let upper = (num / 10) as usize;
            match num % 10 {
                // If the number is a multiple of 10, return the corresponding string from the `TENS` array.
                0 => TENS[upper].to_string(),
                // If the number is not a multiple of 10, return a hyphenated string combining the corresponding string from the `TENS` array with the corresponding string from the `ONES` array.
                lower => format!("{}-{}", TENS[upper], spell(lower)),
            }                                                                               
        }
        // For numbers between 100 and 999 (inclusive), return the corresponding string by calling the `format_num` function with `num`, `100`, and `"hundred"` as arguments.
        100..=999 => format_num(num, 100, "hundred"),
         // For numbers greater than or equal to 1000, find the appropriate order of magnitude (thousand or million) and call the `format_num` function with `num`, the appropriate order of magnitude, and the corresponding string from the `ORDERS` array as arguments.
         _=> {
            let (div, order) =
                successors(Some(1u64), |v| v.checked_mul(1000))
                    .zip(ORDERS.iter())
                    .find(|&(e,_)| e > num / 1000)
                    .unwrap();

            format_num(num, div, order)
        }
    }
}

// Function that takes three arguments, `num`, `div`, and `order`, and returns the English language
//equivalent of `num` by combining the corresponding string from the `ONES` or `TENS` array with the
//corresponding string from the `ORDERS` array if necessary.

fn format_num(num: u64, div: u64, order: &str) -> String {
    match (num / div, num % div) {
        (upper, 0) => format!("{} {}", spell(upper), order),
        (upper, lower) => {
            format!("{} {} {}", spell(upper), order, spell(lower))
        }
    }
}