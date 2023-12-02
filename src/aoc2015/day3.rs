use std::{borrow::BorrowMut, collections::HashMap};

use crate::utils::*;

fn move_location(location: &mut Vec<i32>, direction: char) {
    match direction {
        '^' => location[1] += 1,
        'v' => location[1] -= 1,
        '>' => location[0] += 1,
        '<' => location[0] -= 1,
        _ => todo!(),
    }
}

fn track_movement(directions: &String) -> HashMap<Vec<i32>, i32> {
    let mut history: HashMap<Vec<i32>, i32> = HashMap::new();
    let mut location = vec![0, 0];

    *history.entry(location.clone()).or_insert(0) += 1;

    for c in directions.chars() {
        move_location(&mut location, c);
        *history.entry(location.clone()).or_insert(0) += 1;
    }
    history
}

pub fn day3(input_file: &str) -> [i32; 2] {
    let lines = parse_file(input_file);
    let history = track_movement(&lines[0]);
    let visit_count = history.len();
    [visit_count as i32, 0]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day3_part1_example() {
        let lines = &parse_file("inputs/2015/day3-example.txt");
        let history: Vec<_> = lines.iter().map(track_movement).collect();

        assert_eq!(*history[0].get(&vec![0, 0]).unwrap(), 1);
        assert_eq!(*history[0].get(&vec![1, 0]).unwrap(), 1);

        assert_eq!(*history[1].get(&vec![0, 0]).unwrap(), 2);
        assert_eq!(*history[1].get(&vec![1, 0]).unwrap(), 1);
        assert_eq!(*history[1].get(&vec![0, 1]).unwrap(), 1);
        assert_eq!(*history[1].get(&vec![1, 1]).unwrap(), 1);

        assert_eq!(*history[2].get(&vec![0, 0]).unwrap(), 6);
        assert_eq!(*history[2].get(&vec![0, 1]).unwrap(), 5);
    }
    #[test]
    fn test_day3_part1() {
        assert_eq!(day3("inputs/2015/day3.txt")[0], 2081);
    }
}
