fn remove_except_digits_or_comma(input: &String) -> String {
    input
        .chars()
        .filter(|c| c.is_ascii_digit() || *c == ',')
        .collect()
}

fn split_into_two_numbers(input: &String) -> (i64, i64) {
    let (part_a, part_b) = input.split_once(',').unwrap();
    (part_a.parse().unwrap(), part_b.parse().unwrap())
}

fn get_cost(a: (i64, i64), b: (i64, i64), goal: (i64, i64)) -> i64 {
    //if we treat those as lines, if their slope is different that means they cross at some point
    let slope1 = (goal.0.div_euclid(a.0), goal.0.rem_euclid(a.0));
    let slope2 = (goal.1.div_euclid(a.1), goal.1.rem_euclid(a.1));
    if slope1 == slope2 {
        panic!("there is either none or infinite solutions");
    } else {
        //Cramer's rule
        let determinant = a.0 * b.1 - a.1 * b.0;
        let determinant_a = goal.0 * b.1 - goal.1 * b.0;
        let determinant_b = a.0 * goal.1 - a.1 * goal.0;
        if determinant_a % determinant == 0 && determinant_b % determinant == 0 {
            let res_a = determinant_a / determinant;
            let res_b = determinant_b / determinant;
            if res_a >= 0 && res_b >= 0 {
                return res_a * 3 + res_b;
            }
        }
    }

    return 0;
}

pub fn solve(input: &String) {
    let extra_distance: i64 = 10000000000000;
    let mut it = input.lines();
    let mut total_part_1: i64 = 0;
    let mut total_part_2: i64 = 0;
    loop {
        let a: (i64, i64) = split_into_two_numbers(&remove_except_digits_or_comma(
            &it.next().unwrap().to_string(),
        ));
        let b: (i64, i64) = split_into_two_numbers(&remove_except_digits_or_comma(
            &it.next().unwrap().to_string(),
        ));
        let goal: (i64, i64) = split_into_two_numbers(&remove_except_digits_or_comma(
            &it.next().unwrap().to_string(),
        ));

        total_part_1 += get_cost(a, b, goal);
        let goal_part_2 = (goal.0 + extra_distance, goal.1 + extra_distance);
        total_part_2 += get_cost(a, b, goal_part_2);

        if it.next().is_none() {
            break;
        }
    }
    println!("Part 1 total: {}", total_part_1);
    println!("Part 2 total: {}", total_part_2);
}
