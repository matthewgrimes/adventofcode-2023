use crate::utils::*;

fn is_symbol(character: char) -> bool {
    !character.is_digit(10) & (character != '.')
}

fn check_for_symbols(characters: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    // row above if there is one
    if row > 1 {
        if is_symbol(characters[row - 1][column]) {
            return true;
        }
        if column > 1 {
            if is_symbol(characters[row - 1][column - 1]) {
                return true;
            }
        }
        if column < characters[row - 1].len() - 1 {
            if is_symbol(characters[row - 1][column + 1]) {
                return true;
            }
        }
    }
    // adjacent
    if (column > 1) {
        if is_symbol(characters[row][(column as isize - 1) as usize]) {
            return true;
        }
    }
    if (column < characters[row].len() - 1) {
        if is_symbol(characters[row][column + 1]) {
            return true;
        }
    }
    // below
    if row < characters.len() - 1 {
        if is_symbol(characters[row + 1][column]) {
            return true;
        }
        if column > 1 {
            if is_symbol(characters[row + 1][column - 1]) {
                return true;
            }
        }

        if column < characters[row + 1].len() - 1 {
            if characters[row][column] == '7' {
                println!("hi");
            }
            if is_symbol(characters[row + 1][column + 1]) {
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
    println!("{:?}", characters[0][2]);
    println!("{:?}", characters[1].len());
    println!("{:?}", characters[1][3]);
    println!("{:?}", check_for_symbols(&characters, 0, 2));
    let mut sum = 0;
    for (row_index, row) in numbers.iter().enumerate() {
        for (indices, number) in row.iter() {
            if indices
                .iter()
                .map(|&col_index| check_for_symbols(&characters, row_index, col_index))
                .any(|x| x)
            {
                println!("Valid: {}", number);
                sum += number;
            }
        }
    }
    [sum, 0]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_example() {}
}
