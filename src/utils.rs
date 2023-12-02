use std::fs;
pub fn parse_numbers_from_file(file_path: String, sep: Option<&str>) -> Vec<i32> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
        .split(sep.unwrap_or(","))
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}

pub fn parse_file(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents
        .split('\n')
        .map(|s| s.trim())
        .filter(|s| !s.is_empty())
        .map(|s| s.parse().unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_comma_separated() {
        assert_eq!(
            parse_numbers_from_file(
                "tests/fixtures/numbers_comma_separated.txt".to_string(),
                None
            ),
            (1..10).collect::<Vec::<i32>>()
        );
    }
    #[test]
    fn test_newline_separated() {
        assert_eq!(
            parse_numbers_from_file(
                "tests/fixtures/numbers_newline_separated.txt".to_string(),
                Some("\n")
            ),
            (1..10).collect::<Vec::<i32>>()
        );
    }
}
