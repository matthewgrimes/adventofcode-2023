pub mod day1;
pub mod utils;
use day1::day1;
pub mod day2;
use day2::day2;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(version)]
struct Args {
    #[arg(short, long)]
    day: usize,
    #[arg(short, long)]
    input: Option<String>,
}

fn main() {
    let args = Args::parse();
    let file_path = args
        .input
        .unwrap_or_else(|| format!("../inputs/day{}.txt", args.day));
    match args.day {
        1 => println!("{:?}", day1(file_path.as_str())),
        2 => println!("{:?}", day2(file_path.as_str())),
        _ => (),
    }
}
