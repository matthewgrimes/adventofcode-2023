use crate::utils::*;

const NUMBERS: &'static [&'static str; 10] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_text_number(string: &str) -> Option<usize> {
    for (i, &text) in NUMBERS.iter().enumerate() {
        if text.len() <= string.len() {
            if string[0..text.len()] == *text {
                return Some(i);
            }
        }
    }
    None
}

fn get_digits(string: &String) -> Vec<usize> {
    let mut digits: Vec<usize> = vec![];
    for (i, character) in string.chars().enumerate() {
        match character.to_digit(10) {
            Some(digit) => digits.push(digit as usize),
            None => match find_text_number(&string[i..]) {
                Some(digit) => digits.push(digit),
                None => (),
            },
        }
    }
    digits
}

pub fn day1(input_file: String) {
    let lines = parse_file(input_file);
    let digits: usize = lines
        .iter()
        .map(get_digits)
        .map(|digits| digits[0] * 10 + digits.last().unwrap())
        .sum();
    println!("{:?}", digits);
}

#[cfg(tests)]
mod tests {}
