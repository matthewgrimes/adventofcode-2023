use crate::utils::*;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_text_number(string: &str) -> Option<usize> {
    for (i, &text) in NUMBERS.iter().enumerate() {
        if text.len() <= string.len() {
            if string[0..text.len()] == *text {
                return Some(i + 1);
            }
        }
    }
    None
}

fn get_digits_including_words(string: &String) -> Vec<usize> {
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

fn get_digits(string: &String) -> Vec<usize> {
    let mut digits: Vec<usize> = vec![];
    for character in string.chars() {
        match character.to_digit(10) {
            Some(digit) => digits.push(digit as usize),
            None => (),
        }
    }
    digits
}

pub fn day1(input_file: String) -> [usize; 2] {
    let lines = parse_file(input_file);
    let part1 = lines
        .iter()
        .map(get_digits)
        .map(|digits| digits[0] * 10 + digits.last().unwrap())
        .sum();
    let part2 = lines
        .iter()
        .map(get_digits_including_words)
        .map(|digits| digits[0] * 10 + digits.last().unwrap())
        .sum();
    [part1, part2]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day1() {
        assert_eq!(day1("inputs/day1.txt".to_string()), [54940, 54208]);
    }
}
