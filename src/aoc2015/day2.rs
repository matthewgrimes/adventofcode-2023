use crate::utils::*;
use itertools::Itertools;

pub fn day2(input_path: &str) -> [i32; 2] {
    let lines = parse_file(input_path);
    let wrapping_paper_needs = lines.iter().map(|l| area_from_dimensions(l.as_str())).sum();
    let ribbon_needs = lines.iter().map(|l| ribbon_length(l.as_str())).sum();
    [wrapping_paper_needs, ribbon_needs]
}

fn split_dimensions(dimensions_str: &str) -> Vec<i32> {
    dimensions_str
        .split("x")
        .map(|x| x.parse().unwrap())
        .collect()
}

fn area_from_dimensions(dimensions_str: &str) -> i32 {
    let mut dimensions = split_dimensions(dimensions_str);
    dimensions.sort();
    2 * (dimensions[0] * dimensions[1]
        + dimensions[0] * dimensions[2]
        + dimensions[1] * dimensions[2])
        // add slack
       + dimensions[..2].iter().product::<i32>()
}

fn ribbon_length(dimenstions_str: &str) -> i32 {
    let dimensions = split_dimensions(dimenstions_str);
    let volume: i32 = dimensions.iter().product();
    let min_face_perimeter = dimensions
        .iter()
        .combinations(2)
        .map(|v| 2 * v[0] + 2 * v[1])
        .min()
        .unwrap();
    volume + min_face_perimeter
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
    #[test]
    fn test_part2_example() {
        assert_eq!(ribbon_length("2x3x4"), 34);
        assert_eq!(ribbon_length("1x1x10"), 14);
    }
    #[test]
    fn test_part2() {
        assert_eq!(day2("inputs/2015/day2.txt")[1], 3737498);
    }
}
