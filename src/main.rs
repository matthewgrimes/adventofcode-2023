pub mod day1;
pub mod utils;
use day1::day1;

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
        1 => day1(file_path),
        _ => (),
    }
}
