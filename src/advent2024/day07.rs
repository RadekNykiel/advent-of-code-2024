use itertools::{izip, Itertools};

#[derive(Debug, Clone)]
enum Operation {
    ADD,
    MUL,
    CONCAT,
}

impl Operation {
    fn perform(&self, lhs: &i64, rhs: &i64) -> i64 {
        match self {
            Operation::ADD => lhs + rhs,
            Operation::MUL => lhs * rhs,
            Operation::CONCAT => {
                let mut buff = String::new();
                buff.push_str(&lhs.to_string());
                buff.push_str(&rhs.to_string());
                buff.parse().unwrap()
            }
        }
    }
}

const FIRST_OPS: [Operation; 2] = [Operation::ADD, Operation::MUL];

const MORE_OPS: [Operation; 3] = [Operation::ADD, Operation::MUL, Operation::CONCAT];

#[derive(Debug, Eq, PartialEq, Clone)]
struct Equation {
    expected: i64,
    numbers: Vec<i64>,
}

impl Equation {
    fn get_if_valid(&self, allowed_operations: Vec<Operation>) -> i64 {
        let count = self.numbers.len() - 1;
        let possible_operations =
            std::iter::repeat_n(allowed_operations.iter(), count).multi_cartesian_product();
        for ops in possible_operations {
            let it = izip!(&self.numbers[1..], ops);
            let res = it.fold(self.numbers[0], |acc, (rhs, op)| op.perform(&acc, &rhs));
            if res == self.expected {
                return self.expected;
            }
        }

        0
    }
}

pub fn solve(input: &String) {
    let mut task_equations: Vec<Equation> = Vec::new();
    for l in input.lines() {
        let (expected_string, numbers_string) = l.split_once(":").unwrap();
        let expected: i64 = expected_string.trim().parse().unwrap();
        let numbers: Vec<i64> = numbers_string
            .trim()
            .split(' ')
            .map(|x| x.parse().unwrap())
            .collect();
        task_equations.push(Equation { expected, numbers });
    }

    let mut sum: i64 = 0;
    for e in &task_equations[..] {
        sum += e.get_if_valid(FIRST_OPS.to_vec());
    }
    println!("Sum for part one is: {}", sum);
    let mut sum2: i64 = 0;
    for e in &task_equations[..] {
        sum2 += e.get_if_valid(MORE_OPS.to_vec());
    }
    println!("Sum for part two is: {}", sum2);
}
