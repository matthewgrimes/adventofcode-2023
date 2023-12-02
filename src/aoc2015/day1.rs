use crate::utils::*;
pub fn day1(input_file: &str) -> [usize; 2] {
    let lines = parse_file(input_file).pop().unwrap();
    let mut l_count = 0_usize;
    let mut r_count = 0_usize;
    for c in lines.chars() {
        match c {
            '(' => l_count += 1,
            ')' => r_count += 1,
            _ => todo!(),
        }
    }
    [l_count - r_count, 0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1() {
        assert_eq!(day1("inputs/2015/day1.txt")[0], 280);
    }
}
