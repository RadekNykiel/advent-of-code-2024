use crate::utils2d::{get_direction_vector, Array2D, Coordinate, STRAIGHT_DIRECTIONS};
use std::collections::HashSet;

pub fn solve(input: &String) {
    let rows = input.lines().count();
    let cols = input.lines().next().unwrap().trim().len();
    let mut arr2d = Array2D::new(rows, cols);
    let mut start_points = Vec::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            let parsed = c.to_digit(10).unwrap().try_into().unwrap();
            if parsed == 0 {
                start_points.push(Coordinate(x.try_into().unwrap(), y.try_into().unwrap()));
            }
            arr2d.set(x, y, parsed);
        }
    }

    let mut routes_to_reach_tip = 0;
    let mut unique_tips_reachable = 0;
    for zero in &start_points[..] {
        let mut to_check: Vec<Coordinate> = Vec::new();
        to_check.push(*zero);
        let mut found_tips = HashSet::new();

        while to_check.len() != 0 {
            let curr = to_check.pop().unwrap();
            let curr_value = arr2d.get_coordinate(curr).unwrap();
            for dir in STRAIGHT_DIRECTIONS {
                let next = curr + get_direction_vector(dir);
                if let Some(next_value) = arr2d.get_coordinate(next) {
                    if next_value == (curr_value + 1u8) {
                        if next_value == 9u8 {
                            if !found_tips.contains(&next) {
                                found_tips.insert(next);
                                unique_tips_reachable += 1;
                            }
                            routes_to_reach_tip += 1;
                        } else {
                            to_check.push(next);
                        }
                    }
                }
            }
        }
    }

    println!(
        "part 1 score: {}\npart 2 score: {}",
        unique_tips_reachable, routes_to_reach_tip
    );
}
