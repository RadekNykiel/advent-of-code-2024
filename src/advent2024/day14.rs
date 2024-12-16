const WIDTH: i32 = 101;
const HEIGHT: i32 = 103;
const SIMULATION_TIME: i32 = 100;

const HALF_WIDTH: i32 = WIDTH / 2;
const HALF_HEIGHT: i32 = HEIGHT / 2;
const HALF_WIDTH_PLUS: i32 = HALF_WIDTH + 1;
const HALF_HEIGHT_PLUS: i32 = HALF_HEIGHT + 1;

#[allow(non_contiguous_range_endpoints)]
fn get_quadrant(x: i32, y: i32) -> usize {
    let quad = match (x, y) {
        (0..HALF_WIDTH,         0..HALF_HEIGHT)         => 1,
        (HALF_WIDTH_PLUS..WIDTH,   0..HALF_HEIGHT)         => 2,
        (HALF_WIDTH_PLUS..WIDTH,   HALF_HEIGHT_PLUS..HEIGHT)  => 3,
        (0..HALF_WIDTH,         HALF_HEIGHT_PLUS..HEIGHT)  => 4,
        _                                               => 0,
    };
    return quad;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Robot {
    initial_x: i32,
    initial_y: i32,
    v_x: i32,
    v_y: i32,
}

impl Robot {
    pub fn new(initial_x: i32, initial_y: i32, v_x: i32, v_y: i32) -> Robot {
        Robot {
            initial_x,
            initial_y,
            v_x,
            v_y,
        }
    }

    pub fn get_pos_after_time(&self, time: i32) -> (i32, i32) {
        let mut final_x = (self.initial_x + self.v_x * time) % WIDTH;
        let mut final_y = (self.initial_y + self.v_y * time) % HEIGHT;
        if final_x < 0 {
            final_x += WIDTH;
        }
        if final_y < 0 {
            final_y += HEIGHT;
        }
        (final_x, final_y)
    }
}

#[allow(dead_code)]
fn print_robots_position(robots: &Vec<Robot>, time: i32) {
    let mut my_array = [[0u32; HEIGHT as usize]; WIDTH as usize];

    for robot in &robots[..] {
        let (final_x, final_y) = robot.get_pos_after_time(time);
        my_array[final_x as usize][final_y as usize] += 1;
    }
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            print!("{}", if my_array[x as usize][y as usize] != 0 { my_array[x as usize][y as usize].to_string() } else { ".".to_string() });
        }
        print!("\n");
    }
}


pub fn solve(input: &String) {
    let mut robots: Vec<Robot> = vec![];
    for line in input.lines() {
        let v: Vec<i32> = line.trim().replace("p=", "").replace("v=", "").replace(" ", ",").split(',').map(|s| s.parse::<i32>().unwrap()).collect();
        let [px, py, vx, vy] = v[..] else { panic!() };
        let robot = Robot::new(px, py, vx, vy);
        robots.push(robot);

    }

    let mut quadrants_count = vec![0,0,0,0,0];
    for robot in &robots[..] {
        let (final_x, final_y) = robot.get_pos_after_time(SIMULATION_TIME);
        let quadrant = get_quadrant(final_x, final_y);
        quadrants_count[quadrant] += 1;
    }
    let score = &quadrants_count[1..].iter().copied().reduce(|acc, curr| acc * curr).unwrap();
    println!("Part 1 score: {}", score);

    /*
    for dude in &robots[..] {
        println!("{:?}, 99: {:?}, 100: {:?}, 101: {:?}", dude, dude.get_pos_after_time(99), dude.get_pos_after_time(100), dude.get_pos_after_time(101));
        let combinations = (1..).into_iter().map(|t| dude.get_pos_after_time(t)).take_while(|(x,y)| !(*x == dude.initial_x && *y == dude.initial_y)).count();
        println!("COUNT: {}", combinations);
    }
    */

    let mut res2 = 0;
    let mut found = false;
    for i in 1..{
        if found == true {
            break;
        }
        let mut my_array = [[0u32; HEIGHT as usize]; WIDTH as usize];

        for robot in &robots[..] {
            let (final_x, final_y) = robot.get_pos_after_time(i);
            my_array[final_x as usize][final_y as usize] += 1;
        }

        /*
         * search for robots in such pattern
         * __#__
         * _###_
         * #####
         */
        for y in 0..(HEIGHT-2) {
            for x in 2..(WIDTH-2) {
                let xu = x as usize;
                let yu = y as usize;
                
                if my_array[xu][yu] > 0 
                    && my_array[xu+1][yu+1] > 0 && my_array[xu][yu+1] > 0 && my_array[xu-1][yu+1] > 0 
                    && my_array[xu-2][yu+2] > 0 && my_array[xu-1][yu+2] > 0 && my_array[xu][yu+2] > 0 && my_array[xu+1][yu+2] > 0 && my_array[xu+2][yu+2]> 0 {
                    found = true;
                    res2 = i;
                }
            }
        }
        /*
        if found {
            print_robots_position(&robots, i);
        }
        */
    }
    println!("Part 2 solution: {}", res2);
}
