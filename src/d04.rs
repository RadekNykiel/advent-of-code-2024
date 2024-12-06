use std::fs::File;
use std::io::Read;

#[derive(Debug)]
struct Array2D {
    row_size: usize,
    col_size: usize,
    arr: Vec<Vec<u8>>,
}

enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

const ALL_DIRECTION: [Direction; 8] = [
    Direction::Up,
    Direction::UpRight,
    Direction::Right,
    Direction::DownRight,
    Direction::Down,
    Direction::DownLeft,
    Direction::Left,
    Direction::UpLeft,
];

#[derive(Clone, Copy, Debug)]
struct Coordinate(i16, i16);

impl std::ops::AddAssign for Coordinate {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl std::ops::Add for Coordinate {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Coordinate(self.0 + rhs.0, self.1 + rhs.1)
    }
}

fn get_direction_vector(d: Direction) -> Coordinate {
    match d {
        Direction::Up => Coordinate(-1, 0),
        Direction::UpRight => Coordinate(-1, 1),
        Direction::Right => Coordinate(0, 1),
        Direction::DownRight => Coordinate(1, 1),
        Direction::Down => Coordinate(1, 0),
        Direction::DownLeft => Coordinate(1, -1),
        Direction::Left => Coordinate(0, -1),
        Direction::UpLeft => Coordinate(-1, -1),
    }
}

fn get_vector_points(start: Coordinate, len: usize, d: Direction) -> Vec<Coordinate> {
    let mut v = Vec::new();
    let mut curr = start.clone();
    let dir_vec = get_direction_vector(d);

    for _i in 0..len {
        v.push(curr.clone());
        curr += dir_vec;
    }

    v
}

impl Array2D {
    pub fn new(row_size: usize, col_size: usize) -> Array2D {
        Array2D {
            row_size,
            col_size,
            arr: vec![vec![b'0'; col_size]; row_size],
        }
    }

    pub fn get(&self, i: usize, j: usize) -> Option<u8> {
        if let Some(row) = self.arr.get(i) {
            return row.get(j).copied();
        }
        return None;
    }

    pub fn get_coordinate(&self, c: Coordinate) -> Option<u8> {
        let Coordinate(x, y) = c;
        if let (Ok(x_usize), Ok(y_usize)) = (usize::try_from(x), usize::try_from(y)) {
            return self.get(x_usize, y_usize);
        }

        None
    }
}

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
                for d in ALL_DIRECTION {
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
                let diagonal_letters: Vec<Option<u8>> = vec![
                    Direction::UpLeft,
                    Direction::UpRight,
                    Direction::DownRight,
                    Direction::DownLeft,
                ]
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
