use crate::utils2d::*;
use std::collections::{HashMap, HashSet, VecDeque};

#[derive(Debug)]
struct Step {
    pos: (usize, usize),
    score: usize,
    dir: Direction,
    history: Vec<(usize, usize)>,
}

fn rot_clockwise(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Right,
        Direction::Right => Direction::Down,
        Direction::Down => Direction::Left,
        Direction::Left => Direction::Up,
        _ => panic!("not needed"),
    }
}

fn rot_ctrclockwise(d: &Direction) -> Direction {
    match d {
        Direction::Up => Direction::Left,
        Direction::Left => Direction::Down,
        Direction::Down => Direction::Right,
        Direction::Right => Direction::Up,
        _ => panic!("not needed"),
    }
}

pub fn get_next(curr: &(usize, usize), d: &Direction) -> (usize, usize) {
    let vec = get_direction_vector(*d);
    (
        (curr.0 as i32 + vec.0 as i32) as usize,
        (curr.1 as i32 + vec.1 as i32) as usize,
    )
}

pub fn solve(input: &String) {
    let maze: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    let mut start: (usize, usize) = (0, 0);
    for x in 0..maze.len() {
        for y in 0..maze[0].len() {
            if maze[x][y] == 'S' {
                start = (x, y);
            }
        }
    }

    let mut steps: VecDeque<Step> = VecDeque::new();
    steps.push_front(Step {
        pos: start,
        score: 0usize,
        dir: Direction::Right,
        history: vec![start],
    });
    let mut visited: HashMap<(usize, usize), usize> = HashMap::new();
    let mut sits: HashSet<(usize, usize)> = HashSet::new();

    let mut score = None;
    while !steps.is_empty() {
        let curr = steps.pop_front().unwrap();
        if maze[curr.pos.0][curr.pos.1] == 'E' {
            let prev_score = score.unwrap_or(curr.score);
            if curr.score < prev_score {
                score = Some(curr.score);
                sits.clear();
                curr.history.iter().for_each(|c| drop(sits.insert(*c)));
            } else if curr.score == prev_score {
                score = Some(curr.score);
                curr.history.iter().for_each(|c| drop(sits.insert(*c)));
            }
            continue;
        }
        if let Some(maze_val) = visited.get(&curr.pos) {
            if *maze_val < curr.score {
                continue; // we visited this with shorter path
            }
        }
        visited.insert(curr.pos, curr.score);
        let front = get_next(&curr.pos, &curr.dir);
        if maze[front.0][front.1] != '#' {
            let mut next_step = Step {
                pos: front,
                score: curr.score + 1,
                dir: curr.dir,
                history: curr.history.clone(),
            };
            next_step.history.push(front);
            steps.push_back(next_step);
        }
        let turn_clockwise_dir = rot_clockwise(&curr.dir);
        let after_clockwise = get_next(&curr.pos, &turn_clockwise_dir);
        if maze[after_clockwise.0][after_clockwise.1] != '#' {
            let mut next_step = Step {
                pos: after_clockwise,
                score: curr.score + 1001,
                dir: turn_clockwise_dir,
                history: curr.history.clone(),
            };
            next_step.history.push(after_clockwise);
            steps.push_front(next_step);
        }

        let turn_ctrclockwise_dir = rot_ctrclockwise(&curr.dir);
        let after_ctrclockwise = get_next(&curr.pos, &turn_ctrclockwise_dir);
        if maze[after_ctrclockwise.0][after_ctrclockwise.1] != '#' {
            let mut next_step = Step {
                pos: after_ctrclockwise,
                score: curr.score + 1001,
                dir: turn_ctrclockwise_dir,
                history: curr.history.clone(),
            };
            next_step.history.push(after_ctrclockwise);
            steps.push_front(next_step);
        }
    }
    println!("Part 1 score: {}", score.unwrap_or(0));
    println!("Sits: {}", sits.len());
}
