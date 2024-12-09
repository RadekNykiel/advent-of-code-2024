use crate::utils2d::Coordinate;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

pub fn solve(input_filename: &String) {
    let mut input = String::new();
    File::open(&input_filename)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let max_x = input.lines().count() - 1;
    let max_y = input.lines().next().unwrap().chars().count() - 1;

    let mut all_antennas: HashMap<char, Vec<Coordinate>> = HashMap::new();

    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            let this_coord = Coordinate(x.try_into().unwrap(), y.try_into().unwrap());
            if let Some(found) = all_antennas.get_mut(&c) {
                found.push(this_coord);
            } else {
                all_antennas.insert(c, vec![this_coord]);
            }
        }
    }

    let mut antinodes: HashSet<Coordinate> = HashSet::new();
    let mut antinodes_with_harmonics: HashSet<Coordinate> = HashSet::new();

    for (_, same_fq_antennas) in all_antennas {
        for pair in same_fq_antennas.iter().combinations(2) {
            let [a, b] = &pair[..] else {
                panic!("Should not happen")
            };
            let diff = **b - **a;
            let mut first_antinode = **a - diff;
            let mut second_antinode = **b + diff;
            antinodes_with_harmonics.insert(**a);
            antinodes_with_harmonics.insert(**b);
            if first_antinode.in_bounds(max_x.try_into().unwrap(), max_y.try_into().unwrap()) {
                antinodes.insert(first_antinode);
            }
            if second_antinode.in_bounds(max_x.try_into().unwrap(), max_y.try_into().unwrap()) {
                antinodes.insert(second_antinode);
            }
            while first_antinode.in_bounds(max_x.try_into().unwrap(), max_y.try_into().unwrap()) {
                antinodes_with_harmonics.insert(first_antinode);
                first_antinode = first_antinode - diff;
            }
            while second_antinode.in_bounds(max_x.try_into().unwrap(), max_y.try_into().unwrap()) {
                antinodes_with_harmonics.insert(second_antinode);
                second_antinode = second_antinode + diff;
            }
        }
    }

    println!("Result for the first part: {}", antinodes.len());
    println!(
        "Result for the second part: {}",
        antinodes_with_harmonics.len()
    );
}
