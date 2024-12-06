use std::fs::File;
use std::io::Read;

fn parse_input(input: &str) -> Vec<Vec<i32>> {
    let mut result = Vec::new();
    let mut it = input.lines();
    while let Some(line) = it.next() {
        let mut row = Vec::new();
        for num in line.split_whitespace() {
            row.push(num.parse().unwrap());
        }
        result.push(row);
    }
    result
}

fn check_safe(row: &Vec<i32>) -> bool {
    let it = row.iter().zip(row.iter().skip(1));
    let diffs = it.map(|(a, b)| a - b);
    let all_positives = diffs.clone().all(|x| (1..=3).contains(&x));
    let all_negatives = diffs.clone().all(|x| (-3..=-1).contains(&x));
    all_positives || all_negatives
}

fn check_safe_with_dampener(row: &Vec<i32>) -> bool {
    if check_safe(row) {
        return true;
    } else {
        let row_len = row.len();
        for i in 0..row_len {
            let mut row_copy = row.clone();
            row_copy.remove(i);
            if check_safe(&row_copy) {
                return true;
            }
        }
    }
    false
}

fn solve1(input: Vec<Vec<i32>>) {
    let mut safe_count = 0;
    let mut safe_with_dampener_count = 0;
    for row in input {
        let safe = check_safe(&row);
        let safe_with_dampener = check_safe_with_dampener(&row);
        if safe {
            safe_count += 1;
        }
        if safe_with_dampener {
            safe_with_dampener_count += 1;
        }

    }
    println!("Safe rows: {}", safe_count);
    println!("Safe rows with dampener: {}", safe_with_dampener_count);
}


pub fn solve(input_file: &String) -> std::io::Result<()> {
    let mut input_content = String::new();
    File::open(input_file)?.read_to_string(&mut input_content)?;
    let input = parse_input(&input_content);
    solve1(input);
    // res is 660, 689


    Ok(())
}
