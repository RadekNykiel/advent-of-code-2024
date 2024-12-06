// Code for day 1 of advent of code 2024
// https://adventofcode.com/2024/day/1

use std::fs::File;
use std::io::Read;

// i want to define a type that is tuple of two vectors of i32
//here is the definition
type TwoVec = (Vec<i32>, Vec<i32>);


fn parse_input(input: &str) -> TwoVec {
    let mut lines = input.lines();
    let mut first_array = Vec::new();
    let mut second_array = Vec::new();
    while let Some(line) = lines.next() {
        let (a, b) = line.trim().split_once(' ').unwrap();
        let a = a.trim().parse().unwrap();
        let b = b.trim().parse().unwrap();
        first_array.push(a);
        second_array.push(b);
    }
    (first_array, second_array)
}

fn solve1(input: &mut TwoVec) {
    let (left_numbers, right_numbers) = input;
    left_numbers.sort();
    right_numbers.sort();
    let mut result = 0;
    let zip = left_numbers.iter().zip(right_numbers.iter());
    for (left, right) in zip {
        result += (left - right).abs();
    }
    println!("Result 1: {}", result);
}

fn solve2(input: &mut TwoVec) {
    let (left_numbers, right_numbers) = input;
    let mut similarity = 0;
    for i in left_numbers.iter() {
        let occurences: i32 = right_numbers.iter().filter(|&x| x == i).count().try_into().unwrap();
        similarity += occurences * (*i);
    }
    println!("Result 2: {}", similarity);
}

pub fn solve(input_file: &String) -> std::io::Result<()> {
    let mut input_content = String::new();
    File::open(input_file)?.read_to_string(&mut input_content)?;
    let mut numbers = parse_input(input_content.as_str());
    solve1(&mut numbers);
    // my output was Result: 2375403
    solve2(&mut numbers);
    // my output was Result: 23082277
    Ok(())
}
