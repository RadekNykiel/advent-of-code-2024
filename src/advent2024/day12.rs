use crate::utils2d::*;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &String) {
    let mut plant_map: Vec<Vec<(char, i32)>> = input
        .lines()
        .map(|line| line.chars().map(|c| (c, 0)).collect())
        .collect();

    let mut current_region_id = 1;
    let mut score = 0;
    let mut region_map: HashMap<i32, Vec<(usize, usize)>> = HashMap::new();

    let rows = plant_map.len();
    let cols = plant_map[0].len();
    for iter_x in 0..rows {
        for iter_y in 0..cols {
            let (iter_node_type, iter_node_region) = plant_map
                .get(iter_x)
                .and_then(|row| row.get(iter_y))
                .unwrap()
                .to_owned();
            if iter_node_region == 0 {
                let type_to_find = iter_node_type;
                let mut curr_region_area = 0;
                let mut curr_region_border = 0;

                let mut to_process: Vec<(usize, usize)> = vec![(iter_x, iter_y)];
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                visited.insert((iter_x, iter_y));

                region_map.insert(current_region_id, vec![]);
                while to_process.len() != 0 {
                    let next_to_process_coord = to_process.pop().unwrap();
                    curr_region_area += 1;
                    region_map
                        .get_mut(&current_region_id)
                        .unwrap()
                        .push((next_to_process_coord.0, next_to_process_coord.1));

                    if let Some(n) = plant_map
                        .get_mut(next_to_process_coord.0)
                        .and_then(|row| row.get_mut(next_to_process_coord.1))
                    {
                        n.1 = current_region_id;
                        curr_region_border += 4;
                    }

                    for Coordinate(del_x, del_y) in
                        STRAIGHT_DIRECTIONS.iter().map(|d| get_direction_vector(*d))
                    {
                        let to_check_x: i32 =
                            i32::try_from(next_to_process_coord.0).unwrap() + i32::from(del_x);
                        let to_check_y: i32 =
                            i32::try_from(next_to_process_coord.1).unwrap() + i32::from(del_y);
                        if to_check_x < 0 || to_check_y < 0 {
                            continue;
                        }
                        let to_check_x_u: usize = to_check_x.try_into().unwrap();
                        let to_check_y_u: usize = to_check_y.try_into().unwrap();
                        if let Some(n) = plant_map
                            .get(to_check_x_u)
                            .and_then(|row| row.get(to_check_y_u))
                        {
                            if n.0 == type_to_find {
                                let x = (
                                    to_check_x.try_into().unwrap(),
                                    to_check_y.try_into().unwrap(),
                                );
                                curr_region_border -= 1;
                                if n.1 == 0 && !visited.contains(&x) {
                                    to_process.push(x);
                                    visited.insert(x);
                                }
                            }
                        }
                    }
                }
                current_region_id += 1;
                score += curr_region_area * curr_region_border;
            }
        }
    }

    println!("Score: {}", score);
}
