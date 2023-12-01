use crate::utils::*;
pub fn day1(input_file: String) {
    let numbers = parse_numbers_from_file(input_file, None);
    println!("{:?}", numbers);
    println!("{}", numbers.iter().sum::<i32>());
}

#[cfg(tests)]
mod tests {}
