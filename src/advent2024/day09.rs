use std::fs::File;
use std::io::Read;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Block {
    FILE(u16),
    FREE,
}

pub fn solve(input_file: &String) {
    let mut input = String::new();
    File::open(&input_file)
        .unwrap()
        .read_to_string(&mut input)
        .unwrap();

    let mut next_is_file = true;
    let mut blocks: Vec<Block> = Vec::new();
    let mut next_file_id: u16 = 0;
    for c in input.trim().chars() {
        let next_count: usize = c.to_string().parse().unwrap();
        let next_elem = if next_is_file { Block::FILE(next_file_id) } else { Block::FREE };
        if next_is_file { next_file_id += 1; }
        blocks.append(vec![next_elem; next_count].as_mut());
        next_is_file = !next_is_file;
    }

    let mut magic = blocks.iter().filter_map(|b| match b {
        Block::FILE(v) => Some(v),
        Block::FREE => None,
    }).rev();
    let size = magic.clone().count();
    let mut formatted: Vec<u16> = Vec::new();
    for b in blocks.iter() {
        match b {
            Block::FILE(v) => formatted.push(*v),
            Block::FREE => formatted.push( *magic.next().unwrap()),
        }
    }
    formatted.truncate(size);
    let res = formatted.iter().enumerate().fold(0_u64, |acc, (id, val)| {
        acc + u64::try_from(id).unwrap() * u64::from(*val)
    });
    println!("{}", res);
}
