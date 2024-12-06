#[derive(Clone, Copy, Debug)]
pub struct Coordinate(pub i16, pub i16);

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

#[derive(Debug)]
pub struct Array2D {
    pub row_size: usize,
    pub col_size: usize,
    pub arr: Vec<Vec<u8>>,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
}

pub const DIAGONAL_DIRECTIONS: [Direction; 4] = [
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownRight,
    Direction::DownLeft,
];

pub const STRAIGHT_DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

pub const ALL_DIRECTIONS: [Direction; 8] = [
    Direction::UpLeft,
    Direction::UpRight,
    Direction::DownRight,
    Direction::DownLeft,
    Direction::Up,
    Direction::Right,
    Direction::Down,
    Direction::Left,
];

pub fn get_direction_vector(d: Direction) -> Coordinate {
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

pub fn get_vector_points(start: Coordinate, len: usize, d: Direction) -> Vec<Coordinate> {
    let mut v = Vec::new();
    let mut curr = start.clone();
    let dir_vec = get_direction_vector(d);

    for _i in 0..len {
        v.push(curr.clone());
        curr += dir_vec;
    }

    v
}

