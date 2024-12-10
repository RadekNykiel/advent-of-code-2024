mod advent2024;
mod utils2d;

use std::env;
use std::fs::read_to_string;

fn main() {
    let args: Vec<String> = env::args().collect();
    let [_, day, filename, ..] = &args[..] else {
        panic!("not enough args!")
    };
    println!("DAY: {}, FILENAME: {}", day, filename);
    let input: String = read_to_string(&filename).unwrap();
    match day.as_str() {
        "1" => advent2024::day01::solve(&input),
        "2" => advent2024::day02::solve(&input),
        "3" => advent2024::day03::solve(&input),
        "4" => advent2024::day04::solve(&input),
        "5" => advent2024::day05::solve(&input),
        "6" => advent2024::day06::solve(&input),
        "7" => advent2024::day07::solve(&input),
        "8" => advent2024::day08::solve(&input),
        "9" => advent2024::day09::solve(&input),
        "10" => advent2024::day10::solve(&input),
        _ => panic!("No such task implemented"),
    }
}
