use crate::utils::*;
use md5;

fn add_salt(base: &String, salt: usize) -> String {
    format!("{}{}", base, salt)
}

pub fn day4(input_file: &str) -> [i32; 2] {
    let lines = parse_file(input_file);
    let base = &lines[0];

    let mut leading_zeros = 0;
    let mut salt = 0;
    while leading_zeros < 6 {
        let result = format!("{:x}", md5::compute(add_salt(base, salt)));
        leading_zeros = result[..6].chars().filter(|x| *x == '0').count();
        salt += 1;
        if salt % 1_000_000 == 0 {
            println!("{:?}", salt);
        }
    }
    [salt as i32 - 1, 0]
}
