#[derive(Debug)]
struct Computer {
    code: Vec<u128>,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct ComputerState {
    reg_a: u128,
    reg_b: u128,
    reg_c: u128,
    ptr: usize,
}

impl ComputerState {
    fn get_combo(&self, u: u128) -> u128 {
        match u {
            0..=3 => u,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => panic!("Wrong arg"),
        }
    }
}

impl Computer {
    pub fn calculate(&self, cs: &ComputerState) -> Vec<u128> {
        if cs.ptr == self.code.len() {
            return vec![];
        }
        let instruction = self.code[cs.ptr];
        let operand = self.code[cs.ptr + 1];
        let (ns, curr_out) = self.perform_operation(&cs, instruction, operand);
        let rest_out = self.calculate(&ns);
        curr_out
            .map(|x| vec![x])
            .unwrap_or(vec![])
            .iter()
            .chain(rest_out.iter())
            .copied()
            .collect()
    }

    fn perform_operation(
        &self,
        s: &ComputerState,
        instruction: u128,
        operand: u128,
    ) -> (ComputerState, Option<u128>) {
        let mut ns = s.clone();
        let mut out = None;
        match instruction {
            0 => {
                ns.reg_a = ns.reg_a / 2u128.pow(ns.get_combo(operand).try_into().unwrap());
                ns.ptr += 2;
            }
            1 => {
                ns.reg_b ^= operand;
                ns.ptr += 2;
            }
            2 => {
                ns.reg_b = ns.get_combo(operand) % 8;
                ns.ptr += 2;
            }
            3 => {
                if ns.reg_a != 0 {
                    ns.ptr = usize::try_from(operand).unwrap();
                } else {
                    ns.ptr += 2;
                }
            }
            4 => {
                ns.reg_b ^= ns.reg_c;
                ns.ptr += 2;
            }
            5 => {
                out = Some(ns.get_combo(operand) % 8);
                ns.ptr += 2;
            }
            6 => {
                ns.reg_b = ns.reg_a / 2u128.pow(ns.get_combo(operand).try_into().unwrap());
                ns.ptr += 2;
            }
            7 => {
                ns.reg_c = ns.reg_a / 2u128.pow(ns.get_combo(operand).try_into().unwrap());
                ns.ptr += 2;
            }
            _ => panic!("Wrong instruction!"),
        }
        (ns, out)
    }
}

pub fn solve(input: &String) {
    let mut lines_iter = input.lines();
    let reg: Vec<u128> = lines_iter
        .by_ref()
        .take_while(|l| !l.is_empty())
        .map(|l| l.trim().split(' ').last().unwrap().parse().unwrap())
        .collect();
    let code: Vec<u128> = lines_iter
        .flat_map(|l| {
            l.trim()
                .split(' ')
                .last()
                .unwrap()
                .split(',')
                .map(|s| s.parse().unwrap())
        })
        .collect();
    let &[reg_a, reg_b, reg_c] = &reg[..] else {
        panic!("error")
    };
    let init_state = ComputerState {
        reg_a,
        reg_b,
        reg_c,
        ptr: 0,
    };
    let computer = Computer { code: code.clone() };
    let nums: Vec<String> = computer
        .calculate(&init_state)
        .iter()
        .map(|v| v.to_string())
        .collect();
    println!("Part 1 solution: {}", nums.join(","));

    let mut i = 0;
    let mut x = 1;
    loop {
        let i_state = ComputerState {
            reg_a: i,
            reg_b,
            reg_c,
            ptr: 0,
        };
        let out = computer.calculate(&i_state);
        if out == code[code.len() - x..] {
            if x == code.len() {
                break;
            }
            i = i.checked_mul(8).unwrap();
            x += 1;
        } else {
            i += 1;
        }
    }
    println!("Part 2 solution: {}", i);
}
