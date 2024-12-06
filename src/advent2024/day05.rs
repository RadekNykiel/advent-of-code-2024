use std::collections::HashMap;
use std::fs::File;
use std::io::Read;
use std::cmp::Ordering;


type PageOrdering = HashMap<i32, Vec<i32>>;

fn compare(ordering: &PageOrdering, lhs: i32, rhs: i32) -> Ordering {
    if let Some(found) = ordering.get(&lhs) {
        if found.contains(&rhs) {return Ordering::Less}
    }
    if let Some(found) = ordering.get(&rhs) {
        if found.contains(&lhs) {return Ordering::Greater}
    }
    return Ordering::Equal;
}

pub fn solve(input_file: &String) {
    let mut input_content = String::new();
    File::open(input_file)
        .unwrap()
        .read_to_string(&mut input_content)
        .unwrap();

    let mut it = input_content.lines();

    let mut ordering: PageOrdering = HashMap::new();

    for line in it.by_ref().take_while(|l| !l.trim().is_empty()) {
        let Some((k, v)) = line.split_once('|') else {
            panic!("Malformed input!")
        };
        let key_int: i32 = k.parse().unwrap();
        let value_int: i32 = v.parse().unwrap();

        if let Some(found) = ordering.get_mut(&key_int) {
found.push(value_int);
        } else {
            ordering.insert(key_int, vec![value_int]);
        }
    }

    let mut corrects_sum = 0;
    let mut incorr_sum = 0;
    for line in it {
        let mut numbers: Vec<i32> = line.split(",").map(|el| el.parse().unwrap()).collect();
        if check_line(&ordering, &numbers[..]) {
            let middle = usize::div_ceil(numbers.len(), 2);
            corrects_sum += numbers[middle -1 ];
        } else {
            numbers.sort_by(|a,b| compare(&ordering, *a, *b));
            let middle = usize::div_ceil(numbers.len(), 2);
            incorr_sum += numbers[middle -1 ];
        }
    }

    println!("Result for part 1: {}", corrects_sum);
    println!("Result for part 2: {}", incorr_sum);
}

fn check_line(ordering: &PageOrdering, input: &[i32]) -> bool {
    let (first, rest) = (&input[0], &input[1..]);
    if rest.len() == 0 {
        return true;
    }
    for other in rest {
        let status = ordering.get(other).map_or(false, |x| x.contains(first));
        if status {
            return false;
        }
    }
        
    return check_line(ordering, rest);
}

