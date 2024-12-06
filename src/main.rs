mod d01;
mod d02;
mod d03;
mod d04;
mod d05;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let [_, day, filename, ..] = &args[..] else { panic!("not enough args!") };
    println!("DAY: {}, FILENAME: {}", day, filename);
    match day.as_str() {
        "1" => drop(d01::solve(filename)),
        "2" => drop(d02::solve(filename)),
        "3" => drop(d03::solve(filename)),
        "4" => d04::solve(filename),
        "5" => d05::solve(filename),
        _ => panic!("No such task implemented")
    }
}
