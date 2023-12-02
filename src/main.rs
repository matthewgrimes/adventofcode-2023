pub mod aoc2015;
pub mod aoc2023;
pub mod utils;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    day: usize,
    #[arg(short, long)]
    input: Option<String>,
    #[arg(short, long, default_value_t = 2023)]
    year: usize,
}

fn main() {
    let args = Args::parse();
    let file_path = args
        .input
        .unwrap_or_else(|| format!("../inputs/{}/day{}.txt", args.year, args.day));
    match args.year {
        2023 => match args.day {
            1 => println!("{:?}", aoc2023::day1::day1(file_path.as_str())),
            2 => println!("{:?}", aoc2023::day2::day2(file_path.as_str())),
            _ => (),
        },
        2015 => match args.day {
            1 => println!("{:?}", aoc2015::day1::day1(file_path.as_str())),
            _ => (),
        },
        _ => (),
    }
}
