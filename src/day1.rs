use crate::utils::*;

fn get_digits(string: &String) -> Vec<u32> {
    let mut digits: Vec<u32> = vec![];
    for character in string.chars() {
        match character.to_digit(10) {
            Some(digit) => digits.push(digit),
            None => (),
        }
    }
    digits
}

pub fn day1(input_file: String) {
    let lines = parse_file(input_file);
    let digits: u32 = lines
        .iter()
        .map(get_digits)
        .map(|digits| digits[0] * 10 + digits.last().unwrap())
        .sum();
    println!("{:?}", digits);
}

#[cfg(tests)]
mod tests {}
