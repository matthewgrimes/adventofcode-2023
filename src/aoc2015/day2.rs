use crate::utils::*;

pub fn day2(input_path: &str) -> [i32; 2] {
    let lines = parse_file(input_path);
    let wrapping_paper_needs = lines.iter().map(|l| area_from_dimensions(l.as_str())).sum();
    [wrapping_paper_needs, 0]
}

fn area_from_dimensions(dimensions_str: &str) -> i32 {
    let mut dimensions = dimensions_str
        .split("x")
        .map(|x| x.parse().unwrap())
        .collect::<Vec<i32>>();
    dimensions.sort();
    2 * (dimensions[0] * dimensions[1]
        + dimensions[0] * dimensions[2]
        + dimensions[1] * dimensions[2])
        // add slack
       + dimensions[..2].iter().product::<i32>()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_part1_example() {
        assert_eq!(area_from_dimensions("2x3x4"), 58);
        assert_eq!(area_from_dimensions("1x1x10"), 43);
    }
    #[test]
    fn test_part1() {
        assert_eq!(day2("inputs/2015/day2.txt")[0], 1586300);
    }
}
