// Advent of Code: 2024 day 3
// task: https://adventofcode.com/2024/day/3

use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone)]
enum Token {
    #[allow(dead_code)]
    INVALID(String),
    VALUE(i32),
    KEYWORD(String),
    SPECIAL(char),
}

const L_PAREN: char = '(';
const R_PAREN: char = ')';
const COMMA: char = ',';
const SPECIAL_CHARS: [char; 3] = [L_PAREN, R_PAREN, COMMA];

struct Parser {
    buffer: String,
    parsed: Vec<Token>,
}

impl Parser {
    fn new() -> Parser {
        Parser {
            buffer: String::new(),
            parsed: Vec::new(),
        }
    }

    fn parse(&mut self, input: &String) -> Vec<Token> {
        let mut it = input.chars();

        while let Some(current_char) = it.next() {
            if SPECIAL_CHARS.contains(&current_char) {
                self.process_buffer();
                self.save_token_special(current_char);
            } else {
                self.buffer.push(current_char);
            }
        }
        self.process_buffer();

        self.parsed.clone()
    }

    fn has_buffer(&self) -> bool {
        !self.buffer.is_empty()
    }

    fn is_buffer_numeric(&self) -> bool {
        self.buffer.chars().all(|c| c.is_numeric())
    }

    fn clear_buffer(&mut self) {
        self.buffer.clear();
    }

    fn process_buffer(&mut self) {
        if self.has_buffer() {
            if self.is_buffer_numeric() {
                self.process_buffer_numeric();
            } else {
                self.process_buffer_keyword_with_prefix();
            }
            self.clear_buffer();
        }

    }

    fn process_buffer_numeric(&mut self) {
        self.save_token_value(self.buffer.parse().unwrap());
        self.clear_buffer();
    }

    fn process_buffer_keyword_with_prefix(&mut self) {
        let keywords = ["mul", "don't", "do"];
        let mut found_keyword = None;
        let mut found_prefix = None;
        for kwd in keywords.iter() {
            if self.buffer.ends_with(kwd) {
                found_keyword = Some(kwd);
                self.buffer.truncate(self.buffer.len() - kwd.len());
                break;
            }
        }
        if !self.buffer.is_empty() {
            found_prefix = Some(self.buffer.clone());
        }
        self.clear_buffer();
        if let Some(prefix) = found_prefix {
            self.save_token_invalid(&prefix);
        }
        if let Some(keyword) = found_keyword {
            self.save_token_keyword(keyword);
        }
    }

    fn save_token_invalid(&mut self, val: &str) {
        self.parsed.push(Token::INVALID(val.to_owned()));
    }

    fn save_token_value(&mut self, val: i32) {
        self.parsed.push(Token::VALUE(val));
    }

    fn save_token_keyword(&mut self, val: &str) {
        self.parsed.push(Token::KEYWORD(val.to_owned()));
    }

    fn save_token_special(&mut self, val: char) {
        self.parsed.push(Token::SPECIAL(val));
    }
}

fn calculate(expression: &Vec<Token>, enable_do: bool) -> i32 {
    let mut clone = expression.clone();
    let mut sum = 0;
    let mut enable = 1;

    while !clone.is_empty() {
        match &clone[..] {
            [Token::KEYWORD(kwd), Token::SPECIAL(L_PAREN), Token::VALUE(left), Token::SPECIAL(COMMA), Token::VALUE(right), Token::SPECIAL(R_PAREN), ..]
                if kwd == "mul" =>
            {
                if enable_do {
                    sum += left * right * enable;
                } else {
                    sum += left * right;
                }
                clone.drain(0..6);
            }
            [Token::KEYWORD(kwd), Token::SPECIAL(L_PAREN), Token::SPECIAL(R_PAREN), ..]
                if kwd == "do" =>
            {
                enable = 1;
                clone.drain(0..3);
            }
            [Token::KEYWORD(kwd), Token::SPECIAL(L_PAREN), Token::SPECIAL(R_PAREN), ..]
                if kwd == "don't" =>
            {
                enable = 0;
                clone.drain(0..3);
            }
            _ => {
                drop(clone.remove(0));
            }
        }
    }
    sum
}

fn solve(input_file: &str) -> (i32, i32) {
    let mut input = String::new();
    File::open(input_file)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();
    let mut parser = Parser::new();

    let expression = parser.parse(&input);
    let sum = calculate(&expression, false);
    let sum_with_do = calculate(&expression, true);

    (sum, sum_with_do)
}

fn main() -> std::io::Result<()> {
    let input_file = "input.txt";
    let (sum, sum_with_do) = solve(input_file);
    assert_eq!(sum, 183788984);
    assert_eq!(sum_with_do, 62098619);

    println!("Answer part 1: {}", sum);
    println!("Answer part 2: {}", sum_with_do);
    Ok(())
}
