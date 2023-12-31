use crate::utils::*;

fn is_symbol(character: char) -> Option<char> {
    match !character.is_digit(10) & (character != '.') {
        true => Some(character),
        false => None,
    }
}

fn check_for_symbols(characters: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    // row above if there is one
    if row > 1 {
        if is_symbol(characters[row - 1][column]).is_some() {
            return true;
        }
        if column > 1 {
            if is_symbol(characters[row - 1][column - 1]).is_some() {
                return true;
            }
        }
        if column < characters[row - 1].len() - 1 {
            if is_symbol(characters[row - 1][column + 1]).is_some() {
                return true;
            }
        }
    }
    // adjacent
    if (column > 1) {
        if is_symbol(characters[row][(column as isize - 1) as usize]).is_some() {
            return true;
        }
    }
    if (column < characters[row].len() - 1) {
        if is_symbol(characters[row][column + 1]).is_some() {
            return true;
        }
    }
    // below
    if row < characters.len() - 1 {
        if is_symbol(characters[row + 1][column]).is_some() {
            return true;
        }
        if column > 1 {
            if is_symbol(characters[row + 1][column - 1]).is_some() {
                return true;
            }
        }

        if column < characters[row + 1].len() - 1 {
            if characters[row][column] == '7' {}
            if is_symbol(characters[row + 1][column + 1]).is_some() {
                return true;
            }
        }
    }
    false
}

fn get_numbers_from_row(row: &Vec<char>) -> Vec<(Vec<usize>, i32)> {
    let mut numbers = vec![];
    let mut skip = 0;
    for (i, c) in row.iter().enumerate() {
        if skip > 0 {
            skip -= 1;
            continue;
        }
        if let Some(d) = c.to_digit(10) {
            let end = row[i..]
                .iter()
                .position(|&x| !x.is_digit(10))
                .map_or(row.len(), |e| e + i);
            let number: i32 = row[i..end].iter().collect::<String>().parse().unwrap();
            numbers.push(((i..end).collect(), number));
            skip = end - i;
        }
    }
    numbers
}

pub fn day3(input_file: &str) -> [i32; 2] {
    let lines = parse_file(&input_file);
    let characters = lines
        .iter()
        .map(|line| line.chars().collect())
        .collect::<Vec<Vec<char>>>();
    let numbers: Vec<Vec<(Vec<usize>, i32)>> =
        characters.iter().map(get_numbers_from_row).collect();
    let mut sum = 0;
    for (row_index, row) in numbers.iter().enumerate() {
        for (indices, number) in row.iter() {
            if indices
                .iter()
                .map(|&col_index| check_for_symbols(&characters, row_index, col_index))
                .any(|x| x)
            {
                sum += number;
            }
        }
    }
    let mut gear_ratio_sum = 0;
    for (row_index, row) in characters.iter().enumerate() {
        for (col_index, character) in row.iter().enumerate() {
            if *character == '*' {
                // need to check for adjacent numbers now
                let mut potential_gear_numbers = vec![];
                for (number_row_index, row) in numbers.iter().enumerate() {
                    for (indices, number) in row.iter() {
                        let mut is_adjacent = false;

                        for number_col_index in indices.iter() {
                            if is_adjacent {
                                continue;
                            }
                            if (((col_index as isize) - (*number_col_index as isize)).abs() <= 1)
                                & (((row_index as isize) - (number_row_index as isize)).abs() <= 1)
                            {
                                potential_gear_numbers.push(number);
                                is_adjacent = true;
                            }
                        }
                    }
                }
                println!("{:?}", potential_gear_numbers);
                if potential_gear_numbers.len() == 2 {
                    gear_ratio_sum += potential_gear_numbers[0] * potential_gear_numbers[1];
                }
            }
        }
    }
    [sum, gear_ratio_sum]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(day3("inputs/2023/day3.txt"), [498559, 72246648]);
    }
}
