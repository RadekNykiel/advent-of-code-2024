use crate::utils2d::*;
use std::collections::{HashSet, VecDeque};

fn char_to_dir(input: char) -> Direction {
    match input {
        '^' => Direction::Up,
        '<' => Direction::Left,
        'v' => Direction::Down,
        '>' => Direction::Right,
        _ => panic!("unknown direction char"),
    }
}

pub fn get_next(curr: &(usize, usize), d: &Direction) -> (usize, usize) {
    let vec = get_direction_vector(*d);
    (
        (curr.0 as i32 + vec.0 as i32) as usize,
        (curr.1 as i32 + vec.1 as i32) as usize,
    )
}

pub fn get_score(arr: &Vec<Vec<char>>) -> usize {
    let mut score = 0;
    for x in 0..arr.len() {
        for y in 0..arr[0].len() {
            if arr[x][y] == '[' || arr[x][y] == 'O' {
                score += 100 * x + y;
            }
        }
    }

    score
}

pub fn solve(input: &String) {
    let mut arr: Vec<Vec<char>> = input
        .lines()
        .take_while(|l| !l.is_empty())
        .map(|l| l.chars().collect())
        .collect();
    let mut arr2: Vec<Vec<char>> = arr
        .iter()
        .map(|l| {
            l.iter()
                .flat_map(|c| {
                    (match c {
                        '#' => vec!['#', '#'],
                        'O' => vec!['[', ']'],
                        '@' => vec!['@', '.'],
                        '.' => vec!['.', '.'],
                        _ => panic!("unrecognized char"),
                    })
                })
                .collect()
        })
        .collect();
    let moves: Vec<Direction> = input
        .lines()
        .skip_while(|l| !l.is_empty())
        .flat_map(|l| l.trim().chars())
        .map(|c| char_to_dir(c))
        .collect();

    let mut curr_pos = (0usize, 0usize);
    'outer: for x in 0..arr.len() {
        for y in 0..arr[0].len() {
            if arr[x][y] == '@' {
                arr[x][y] = '.';
                curr_pos = (x, y);
                break 'outer;
            }
        }
    }

    for move_dir in &moves[..] {
        let next_pos = get_next(&curr_pos, move_dir);
        let mut chain_end_pos = next_pos.clone();
        let mut should_move = true;
        loop {
            if arr[chain_end_pos.0][chain_end_pos.1] == '.' {
                break;
            } else if arr[chain_end_pos.0][chain_end_pos.1] == '#' {
                should_move = false;
                break;
            }
            chain_end_pos = get_next(&chain_end_pos, move_dir);
        }
        if should_move {
            arr[next_pos.0][next_pos.1] = '.';
            curr_pos = next_pos;
            if next_pos != chain_end_pos {
                arr[chain_end_pos.0][chain_end_pos.1] = 'O';
            }
        }
    }
    println!("Part 1 res: {}", get_score(&arr));

    'outer: for x in 0..arr2.len() {
        for y in 0..arr2[0].len() {
            if arr2[x][y] == '@' {
                arr2[x][y] = '.';
                curr_pos = (x, y);
                break 'outer;
            }
        }
    }
    for move_dir in &moves[..] {
        let next_pos = get_next(&curr_pos, move_dir);
        let mut should_move = true;
        let mut boxes_to_move = vec![];
        let mut checked: HashSet<(usize, usize)> = HashSet::new();
        let mut fields_to_check: VecDeque<(usize, usize)> = VecDeque::new();
        fields_to_check.push_back(next_pos);
        while !fields_to_check.is_empty() {
            let to_check = fields_to_check.pop_front().unwrap();
            let check_val = arr2[to_check.0][to_check.1];
            match check_val {
                '[' => {
                    let second_box_part = get_next(&to_check, &Direction::Right);
                    if !checked.contains(&to_check) {
                        checked.insert(to_check);
                        checked.insert(second_box_part);
                        boxes_to_move.push(to_check);
                        boxes_to_move.push(second_box_part);
                        if !checked.contains(&get_next(&to_check, move_dir)) {
                            fields_to_check.push_back(get_next(&to_check, move_dir));
                        }
                        if !checked.contains(&get_next(&second_box_part, move_dir)) {
                            fields_to_check.push_back(get_next(&second_box_part, move_dir));
                        }
                    }
                }
                ']' => {
                    let second_box_part = get_next(&to_check, &Direction::Left);
                    if !checked.contains(&to_check) {
                        checked.insert(to_check);
                        checked.insert(second_box_part);
                        boxes_to_move.push(to_check);
                        boxes_to_move.push(second_box_part);
                        if !checked.contains(&get_next(&to_check, move_dir)) {
                            fields_to_check.push_back(get_next(&to_check, move_dir));
                        }
                        if !checked.contains(&get_next(&second_box_part, move_dir)) {
                            fields_to_check.push_back(get_next(&second_box_part, move_dir));
                        }
                    }
                }
                '.' => (),
                '#' => should_move = false,
                _ => panic!("not known elem"),
            }
        }
        if should_move {
            for to_move in boxes_to_move.iter().rev() {
                let next = get_next(&to_move, move_dir);
                arr2[next.0][next.1] = arr2[to_move.0][to_move.1];
                arr2[to_move.0][to_move.1] = '.';
            }
            curr_pos = next_pos;
        }
    }
    println!("Part 2 res: {}", get_score(&arr2));
}
