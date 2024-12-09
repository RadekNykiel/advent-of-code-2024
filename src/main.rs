mod advent2024;
mod utils2d;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let [_, day, filename, ..] = &args[..] else {
        panic!("not enough args!")
    };
    println!("DAY: {}, FILENAME: {}", day, filename);
    match day.as_str() {
        "1" => advent2024::day01::solve(filename),
        "2" => advent2024::day02::solve(filename),
        "3" => advent2024::day03::solve(filename),
        "4" => advent2024::day04::solve(filename),
        "5" => advent2024::day05::solve(filename),
        "6" => advent2024::day06::solve(filename),
        "7" => advent2024::day07::solve(filename),
        "8" => advent2024::day08::solve(filename),
        "9" => advent2024::day09::solve(filename),
        _ => panic!("No such task implemented"),
    }
}
