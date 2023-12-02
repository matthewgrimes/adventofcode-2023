use crate::utils::parse_file;

#[derive(Debug, PartialEq, Eq, Default)]
struct DrawResult {
    red: usize,
    blue: usize,
    green: usize,
}

impl PartialOrd for DrawResult {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let comparisons = (
            self.red <= other.red,
            self.red == other.red,
            self.blue <= other.blue,
            self.blue == other.blue,
            self.green <= other.green,
            self.green == other.green,
        );
        match comparisons {
            (_, true, _, true, _, true) => Some(std::cmp::Ordering::Equal),
            (true, _, true, _, true, _) => Some(std::cmp::Ordering::Less),
            _ => None,
        }
    }
}

impl DrawResult {
    fn reduced_result(draw_results: &[DrawResult]) -> Self {
        let mut reduced_result = DrawResult::default();
        for draw_result in draw_results.iter() {
            reduced_result = reduced_result.join_other(draw_result);
        }
        reduced_result
    }
    fn from_lines(lines: &[String]) -> Vec<Vec<Self>> {
        let mut results = vec![];
        for line in lines.iter() {
            let line_parts: Vec<&str> = line.split(": ").collect();
            let mut game_results = vec![];
            for result in line_parts[1].split("; ") {
                game_results.push(DrawResult::from_text(result));
            }
            results.push(game_results);
        }
        results
    }
    fn pow(&self) -> usize {
        self.red * self.blue * self.green
    }
    fn join_other(&self, other: &Self) -> Self {
        DrawResult {
            red: self.red.max(other.red),
            blue: self.blue.max(other.blue),
            green: self.green.max(other.green),
        }
    }
    fn from_text(text: &str) -> Self {
        let (mut red, mut blue, mut green): (usize, usize, usize) = (0, 0, 0);
        let parts = text.split(", ");
        for part in parts {
            let amount_color: Vec<_> = part.split(' ').collect();
            let amount: usize = amount_color[0].parse().unwrap();
            let color = amount_color[1];
            match color {
                "red" => red += amount,
                "blue" => blue += amount,
                "green" => green += amount,
                _ => (),
            }
        }
        Self { red, blue, green }
    }
}

pub fn day2(input_file: &str) -> [usize; 2] {
    let lines = parse_file(input_file);
    let constraint = DrawResult {
        red: 12,
        green: 13,
        blue: 14,
    };
    let results = DrawResult::from_lines(&lines);
    let mut total = 0;
    for (game, game_result) in results.iter().enumerate() {
        let impossible_games = game_result
            .iter()
            .filter(|&r| !(r <= &constraint))
            .collect::<Vec<_>>();
        match impossible_games.is_empty() {
            true => total += game + 1,
            false => {
                println!("{:?}", constraint);
                println!("{}: {:?}", game + 1, impossible_games);
                println!("===========");
            }
        }
    }
    let powers = results
        .iter()
        .map(|r| DrawResult::reduced_result(r).pow())
        .sum();
    [total, powers]
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;
    #[test]
    fn test_day2_part1_sample() {
        assert_eq!(day2("inputs/day2-part1-sample.txt")[0], 8);
    }

    #[test]
    fn test_day2_part1_answer() {
        assert_eq!(day2("inputs/day2.txt")[0], 2169);
    }
    #[test]
    fn test_day2_part2_sample() {
        let lines = parse_file("inputs/day2-part1-sample.txt");
        let results = DrawResult::from_lines(&lines);
        let powers: Vec<usize> = results
            .iter()
            .map(|r| DrawResult::reduced_result(r).pow())
            .collect();
        assert_eq!(powers, vec![48, 12, 1560, 630, 36]);
    }
    #[test]
    fn test_day2_part2_answer() {
        let lines = parse_file("inputs/day2.txt");
        let results = DrawResult::from_lines(&lines);
        let powers: usize = results
            .iter()
            .map(|r| DrawResult::reduced_result(r).pow())
            .sum();
        assert_eq!(powers, 60948);
    }
    #[test]
    fn test_result_from_test() {
        let input = "3 blue, 1 red, 0 green";
        let expected = DrawResult {
            blue: 3,
            red: 1,
            green: 0,
        };
        assert_eq!(expected, DrawResult::from_text(input));
    }
    #[test]
    fn test_result_cmp() {
        assert!(
            !(DrawResult {
                red: 2,
                blue: 7,
                green: 15
            } <= DrawResult {
                red: 12,
                blue: 14,
                green: 13
            })
        );

        assert!(
            DrawResult {
                red: 6,
                blue: 5,
                green: 13
            } <= DrawResult {
                red: 12,
                green: 13,
                blue: 14
            }
        );
    }
    proptest! {
    #[test]
        fn test_result_cmp_random(r1 in 0usize..usize::MAX,r2 in 0usize..usize::MAX,b1 in 0usize..usize::MAX,b2 in 0usize..usize::MAX,g1 in 0usize..usize::MAX,g2 in 0usize..usize::MAX) {
            let result1 = DrawResult {red: r1, blue: b1, green: g1};
            let result2 = DrawResult {red: r2, blue: b2, green: g2};
            let join1 = result1.join_other(&result2);
            let join2 = result2.join_other(&result1);

            assert_eq!(join1, join2);
            assert!(result1<=join1);
            assert!(result2<=join1);
        }
        }
}
