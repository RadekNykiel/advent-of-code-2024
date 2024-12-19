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
        "11" => advent2024::day11::solve(&input),
        "12" => advent2024::day12::solve(&input),
        "13" => advent2024::day13::solve(&input),
        "14" => advent2024::day14::solve(&input),
        "15" => advent2024::day15::solve(&input),
        "16" => advent2024::day16::solve(&input),
        "17" => advent2024::day17::solve(&input),
        "18" => advent2024::day18::solve(&input),
        "19" => advent2024::day19::solve(&input),
        "20" => advent2024::day20::solve(&input),
        "21" => advent2024::day21::solve(&input),
        "22" => advent2024::day22::solve(&input),
        "23" => advent2024::day23::solve(&input),
        "24" => advent2024::day24::solve(&input),
        "25" => advent2024::day25::solve(&input),
        _ => panic!("No such task implemented"),
    }
}
