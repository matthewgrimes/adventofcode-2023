use crate::utils::*;

const NUMBERS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn find_text_number(string: &str) -> Option<usize> {
    for (i, &text) in NUMBERS.iter().enumerate() {
        if text.len() <= string.len() && string[0..text.len()] == *text {
            return Some(i + 1);
        }
    }
    None
}

fn get_digits_including_words(string: &str) -> Vec<usize> {
    let mut digits: Vec<usize> = vec![];
    for (i, character) in string.chars().enumerate() {
        match character.to_digit(10) {
            Some(digit) => digits.push(digit as usize),
            None => {
                if let Some(digit) = find_text_number(&string[i..]) {
                    digits.push(digit);
                }
            }
        }
    }
    digits
}

fn get_digits(string: &str) -> Vec<usize> {
    let mut digits: Vec<usize> = vec![];
    for character in string.chars() {
        if let Some(digit) = character.to_digit(10) {
            digits.push(digit as usize);
        }
    }
    digits
}

pub fn day1(input_file: &str) -> [usize; 2] {
    let lines = parse_file(input_file);
    let part1 = lines
        .iter()
        .map(|l| l.as_str())
        .map(get_digits)
        .map(|digits| digits[0] * 10 + digits.last().unwrap())
        .sum();
    let part2 = lines
        .iter()
        .map(|l| l.as_str())
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
        assert_eq!(day1("inputs/2023/day1.txt"), [54940, 54208]);
    }
}
