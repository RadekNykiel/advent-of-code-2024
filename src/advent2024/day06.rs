use std::collections::HashSet;

use crate::utils2d::*;

fn next_dir(d: Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        _ => panic!("not expected other directions"),
    }
}

pub fn solve(input: &String) {
    let rows = i16::try_from(input.lines().count()).unwrap();
    let cols = i16::try_from(input.lines().next().unwrap().trim().len()).unwrap();

    let mut found_pos = None;
    let mut obstacles: Vec<Coordinate> = Vec::new();
    let mut current_direction = Direction::Up;
    let mut visited: HashSet<Coordinate> = HashSet::new();

    for (x, row) in input.lines().enumerate() {
        for (y, c) in row.chars().enumerate() {
            if c == '^' {
                found_pos = Some(Coordinate(x.try_into().unwrap(), y.try_into().unwrap()));
            } else if c == '#' {
                obstacles.push(Coordinate(x.try_into().unwrap(), y.try_into().unwrap()));
            }
        }
    }

    let Some(init_pos) = found_pos else {
        panic!("Init position not found")
    };
    let mut pos = init_pos.clone();

    while pos.in_bounds(rows - 1, cols - 1) {
        visited.insert(pos.to_owned());
        let mut next_pos = pos.clone() + get_direction_vector(current_direction);
        while obstacles.contains(&next_pos) {
            current_direction = next_dir(current_direction);
            next_pos = pos.clone() + get_direction_vector(current_direction);
        }
        pos = next_pos;
    }

    println!("Visited {} unique fields", visited.len());
    visited.remove(&init_pos);

    let mut looped = 0;
    for extra in &visited {
        let mut obstacles_extra = obstacles.clone();
        obstacles_extra.push(*extra);
        let mut visited_with_dir: HashSet<(Coordinate, Direction)> = HashSet::new();
        pos = init_pos.to_owned();
        current_direction = Direction::Up;
        while pos.in_bounds(rows - 1, cols - 1) {
            if visited_with_dir.contains(&(pos, current_direction)) {
                looped += 1;
                break;
            }
            visited_with_dir.insert((pos.to_owned(), current_direction.to_owned()));
            let mut next_pos = pos.clone() + get_direction_vector(current_direction);
            while obstacles_extra.contains(&next_pos) {
                current_direction = next_dir(current_direction);
                next_pos = pos.clone() + get_direction_vector(current_direction);
            }
            pos = next_pos;
        }
    }
    println!(
        "There are {} places to put obstacle and loop the guard",
        looped
    );
}
