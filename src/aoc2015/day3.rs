use std::collections::HashSet;

use itertools::Itertools;

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

fn track_movement(directions: &String) -> HashSet<Vec<i32>> {
    let mut history: HashSet<Vec<i32>> = HashSet::new();
    let mut location = vec![0, 0];

    history.insert(location.clone());

    for c in directions.chars() {
        move_location(&mut location, c);
        history.insert(location.clone());
    }
    history
}

fn get_santa_and_robo_routes(directions: &String) -> [String; 2] {
    let raw_directions = directions.split("").collect_vec();
    [
        raw_directions[1..].iter().step_by(2).join(""),
        raw_directions.iter().step_by(2).join(""),
    ]
}

pub fn day3(input_file: &str) -> [i32; 2] {
    let lines = parse_file(input_file);
    let santa_robot_routes = get_santa_and_robo_routes(&lines[0]);
    let history = track_movement(&lines[0]);
    let visit_count = history.len();

    let santa_robot_histories: Vec<HashSet<Vec<i32>>> =
        santa_robot_routes.iter().map(track_movement).collect();
    [
        visit_count as i32,
        santa_robot_histories[0]
            .union(&santa_robot_histories[1])
            .collect::<HashSet<_>>()
            .len() as i32,
    ]
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_day3_part1_example() {
        let lines = &parse_file("inputs/2015/day3-example.txt");
        let history: Vec<_> = lines.iter().map(track_movement).collect();

        assert_eq!(history[0], HashSet::from([vec![0, 0], vec![1, 0]]));

        assert_eq!(
            history[1],
            HashSet::from([vec![0, 0], vec![1, 0], vec![1, 1], vec![0, 1]])
        );
        assert_eq!(history[2], HashSet::from([vec![0, 0], vec![0, 1]]));
    }
    #[test]
    fn test_day3() {
        assert_eq!(day3("inputs/2015/day3.txt"), [2081, 2341]);
    }
    #[test]
    fn test_get_santa_and_robo_routes() {
        let routes = get_santa_and_robo_routes(&">>^<v>".to_string());
        assert_eq!(routes[0], ">^v");
        assert_eq!(routes[1], "><>");
    }
    #[test]
    fn test_day3_part2_example() {
        let lines = &parse_file("inputs/2015/day3-example.txt");
        let history: Vec<_> = lines
            .iter()
            .map(get_santa_and_robo_routes)
            .map(|routes| vec![track_movement(&routes[0]), track_movement(&routes[1])])
            .collect();
        assert_eq!(history[0][0], HashSet::from([vec![0, 0], vec![1, 0]]));
        assert_eq!(history[0][1], HashSet::from([vec![0, 0]]));
        assert_eq!(
            history[0][0]
                .union(&history[0][1])
                .collect::<HashSet<_>>()
                .len(),
            2
        );

        assert_eq!(history[1][0], HashSet::from([vec![0, 0], vec![0, 1]]));
        assert_eq!(history[1][1], HashSet::from([vec![0, 0], vec![1, 0]]));
        assert_eq!(
            history[1][0]
                .union(&history[1][1])
                .collect::<HashSet<_>>()
                .len(),
            3
        );

        assert_eq!(
            history[2][0],
            HashSet::from_iter((0..6).map(|x| vec![0, x]))
        );
        assert_eq!(
            history[2][1],
            HashSet::from_iter((0..6).map(|x| vec![0, -x]))
        );
        assert_eq!(
            history[2][0]
                .union(&history[2][1])
                .collect::<HashSet<_>>()
                .len(),
            11
        );
    }
}
