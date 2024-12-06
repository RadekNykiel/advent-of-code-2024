use std::fs::File;
use std::io::Read;

use crate::utils2d::*;


fn parse_input_to_arr(input_file: &str) -> Array2D {
    let mut input_content = String::new();
    File::open(input_file)
        .unwrap()
        .read_to_string(&mut input_content)
        .unwrap();
    let rows = input_content.lines().count();
    let cols = input_content.lines().next().unwrap().len();
    let mut ret = Array2D::new(rows, cols);
    for (i, row) in input_content.lines().enumerate() {
        ret.arr[i] = row.try_into().unwrap();
    }

    ret
}

pub fn solve(input_filename: &String) {
    let input_bytes = parse_input_to_arr(input_filename);

    let mut res: i16 = 0;
    let mut res2: i16 = 0;
    for x in 0..(input_bytes.row_size) {
        for y in 0..(input_bytes.col_size) {
            let coord = Coordinate(x.try_into().unwrap(), y.try_into().unwrap());
            let found = input_bytes.get_coordinate(coord);
            if found.map(|v| v == b'X').unwrap_or_default() {
                for d in ALL_DIRECTIONS {
                    let letters: Vec<Option<u8>> = get_vector_points(coord, 4, d)
                        .into_iter()
                        .map(|c| input_bytes.get_coordinate(c))
                        .collect();
                    match letters[..] {
                        [Some(b'X'), Some(b'M'), Some(b'A'), Some(b'S')] => res += 1,
                        _ => (),
                    }
                }
            }

            if found.map(|v| v == b'A').unwrap_or_default() {
                let diagonal_letters: Vec<Option<u8>> =  DIAGONAL_DIRECTIONS
                    .into_iter()
                .map(|d| get_direction_vector(d) + coord)
                .map(|v| input_bytes.get_coordinate(v))
                .collect();
                match diagonal_letters[..] {
                    [Some(b'M'), Some(b'M'), Some(b'S'), Some(b'S')] => res2 += 1,
                    [Some(b'S'), Some(b'M'), Some(b'M'), Some(b'S')] => res2 += 1,
                    [Some(b'S'), Some(b'S'), Some(b'M'), Some(b'M')] => res2 += 1,
                    [Some(b'M'), Some(b'S'), Some(b'S'), Some(b'M')] => res2 += 1,
                    _ => (),
                }
            }
        }
    }
    println!("Found {} XMAS", res);
    println!("Found {} XMAS in square", res2);
}
