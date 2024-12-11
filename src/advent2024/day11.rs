use std::collections::HashMap;

fn is_even_digits(n: u64) -> bool {
    n.to_string().len() % 2 == 0
}

fn split_digits(n: u64) -> (u64, Option<u64>) {
    let s = n.to_string();
    let first_half = &s[..s.len() / 2].parse().unwrap();
    let second_half = &s[s.len() / 2..].parse().unwrap();
    (*first_half, Some(*second_half))
}

fn map_next(n: u64) -> (u64, Option<u64>) {
    match n {
        0 => (1, None),
        x if is_even_digits(x) => split_digits(x),
        _ => (n * 2024, None),
    }
}

pub fn solve_with_cache(numbers: &Vec<u64>, iterations_count: i32) {
    let mut cache: HashMap<(u64, i32), u128> = HashMap::new();
    let mut score: u128 = 0;
    for num in &numbers[..] {
        score += recursive_solve(&mut cache, *num, iterations_count);
    }
    println!("Result for {} iterations: {}", iterations_count, score);
}

fn recursive_solve(cache: &mut HashMap<(u64, i32), u128>, n: u64, it: i32) -> u128 {
    if it == 0 {
        return 1;
    }
    if let Some(result) = cache.get(&(n, it)) {
        return *result;
    }
    let (next1, possible_next2) = map_next(n);
    let mut result = recursive_solve(cache, next1, it - 1);
    if let Some(next2) = possible_next2 {
        result += recursive_solve(cache, next2, it - 1);
    }
    cache.insert((n, it), result);
    return result;
}

pub fn solve(input: &String) {
    let numbers: Vec<u64> = input
        .trim()
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    solve_with_cache(&numbers, 25i32);
    solve_with_cache(&numbers, 75i32);
}
