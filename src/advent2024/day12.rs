use crate::utils2d::*;
use std::collections::{HashMap, HashSet};

pub fn solve(input: &String) {
    let mut plant_map: Vec<Vec<(char, i32)>> = input
        .lines()
        .map(|line| line.chars().map(|c| (c, 0)).collect())
        .collect();

    let mut current_region_id = 1;
    let mut score = 0;
    let mut score_part2 = 0;
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

                let mut to_process: Vec<(usize, usize)> = vec![(iter_x, iter_y)];
                let mut visited: HashSet<(usize, usize)> = HashSet::new();
                let mut edges: Vec<(Direction, i32, i32)> = Vec::new();
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
                    }

                    for (curr_dir, Coordinate(del_x, del_y)) in STRAIGHT_DIRECTIONS
                        .iter()
                        .map(|d| (d, get_direction_vector(*d)))
                    {
                        let to_check_x: i32 =
                            i32::try_from(next_to_process_coord.0).unwrap() + i32::from(del_x);
                        let to_check_y: i32 =
                            i32::try_from(next_to_process_coord.1).unwrap() + i32::from(del_y);
                        if to_check_x < 0 || to_check_y < 0 {
                            // to check is out of bounds
                            let is_border_vertical =
                                *curr_dir == Direction::Left || *curr_dir == Direction::Right;
                            let bord_common = i32::try_from(if is_border_vertical {
                                next_to_process_coord.1
                            } else {
                                next_to_process_coord.0
                            })
                            .unwrap();
                            let bord_distinct = i32::try_from(if is_border_vertical {
                                next_to_process_coord.0
                            } else {
                                next_to_process_coord.1
                            })
                            .unwrap();
                            edges.push((*curr_dir, bord_common, bord_distinct));
                            continue;
                        }
                        let to_check_x_u: usize = to_check_x.try_into().unwrap();
                        let to_check_y_u: usize = to_check_y.try_into().unwrap();
                        if let Some(n) = plant_map
                            .get(to_check_x_u)
                            .and_then(|row| row.get(to_check_y_u))
                            .filter(|n| n.0 == type_to_find)
                        {
                            // to check has same type
                            let x = (
                                to_check_x.try_into().unwrap(),
                                to_check_y.try_into().unwrap(),
                            );

                            if n.1 == 0 && !visited.contains(&x) {
                                to_process.push(x);
                                visited.insert(x);
                            }
                        } else {
                            //to check has different type
                            let is_border_vertical =
                                *curr_dir == Direction::Left || *curr_dir == Direction::Right;
                            let bord_common = i32::try_from(if is_border_vertical {
                                next_to_process_coord.1
                            } else {
                                next_to_process_coord.0
                            })
                            .unwrap();
                            let bord_distinct = i32::try_from(if is_border_vertical {
                                next_to_process_coord.0
                            } else {
                                next_to_process_coord.1
                            })
                            .unwrap();
                            edges.push((*curr_dir, bord_common, bord_distinct));
                        }
                    }
                }

                current_region_id += 1;
                edges.sort();
                let mut edges_joined: Vec<(Direction, i32, Vec<i32>)> = Vec::new();
                for curr_edge in &edges[..] {
                    if let Some(same_edge) = edges_joined.iter_mut().find(|e| {
                        e.0 == curr_edge.0 && e.1 == curr_edge.1 && e.2.contains(&(curr_edge.2 - 1))
                    }) {
                        same_edge.2.push(curr_edge.2);
                    } else {
                        edges_joined.push((curr_edge.0, curr_edge.1, vec![curr_edge.2]));
                    }
                }
                score += curr_region_area * edges.len();
                score_part2 += curr_region_area * edges_joined.len();
            }
        }
    }

    println!("Score: {}", score);
    println!("Score part 2: {}", score_part2);
}
