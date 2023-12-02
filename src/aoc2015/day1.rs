use crate::utils::*;
pub fn day1(input_file: &str) -> [usize; 2] {
    let lines = parse_file(input_file).pop().unwrap();
    let mut tracker: i32 = 0;
    for c in lines.chars() {
        match c {
            '(' => tracker += 1,
            ')' => tracker -= 1,
            _ => todo!(),
        }
    }
    [tracker as usize, find_basement(&lines)]
}

fn find_basement(directions: &String) -> usize {
    let mut tracker: i32 = 0;
    for (i, c) in directions.chars().enumerate() {
        match c {
            '(' => tracker += 1,
            ')' => tracker -= 1,
            _ => todo!(),
        };
        if tracker < 0 {
            return i + 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(day1("inputs/2015/day1.txt")[0], 280);
    }
    #[test]
    fn test_part2_examples() {
        assert_eq!(find_basement(&")".to_string()), 1);
        assert_eq!(find_basement(&"()())".to_string()), 5);
    }
}
