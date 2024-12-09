use std::fs::File;
use std::io::Read;

#[derive(Clone, PartialEq, Eq)]
enum Block {
    FILE(u16),
    FREE,
}

impl std::fmt::Debug for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Block::FILE(v) => v.to_string(),
            Block::FREE => ".".to_string(),
        })
    }
}

#[derive(Debug)]
struct BlockInfo {
    start_pos: usize,
    block_type: Block,
    len: usize,
}

fn move_blocks_to_left(blocks: &Vec<Block>) -> Vec<Block> {
    let block_count = blocks.len();
    let mut reverse_file_blocks_only = blocks.iter().filter(|b| matches!(b, Block::FILE(_))).rev();
    let file_block_count = reverse_file_blocks_only.clone().count();
    let mut formatted: Vec<Block> = Vec::new();
    for b in blocks.iter() {
        match b {
            Block::FILE(_) => formatted.push(b.clone()),
            Block::FREE => formatted.push( reverse_file_blocks_only.next().unwrap().clone()),
        }
    }
    formatted.truncate(file_block_count);
    formatted.append(vec![ Block::FREE ; block_count - file_block_count ].as_mut());

    formatted
}

fn blocks_to_block_infos(blocks: &Vec<Block>) -> Vec<BlockInfo> {
    let mut res: Vec<BlockInfo> = Vec::new();
    for (pos, b) in blocks.iter().enumerate() {
        if res.last().map_or(false, |last| last.block_type == *b) {
            res.last_mut().unwrap().len += 1;
        } else {
            res.push(BlockInfo {
                start_pos: pos,
                block_type: b.to_owned(),
                len: 1,
            });
        }
    }

    res
}

fn move_whole_files(blocks: &Vec<Block>) -> Vec<Block> {
    let mut res = blocks.clone();
    let mut greatest_file_id: i32 = blocks.iter().fold(0, |acc, b| match b {
        Block::FILE(v) => if i32::from(*v) > acc { i32::from(*v) } else { acc },
        Block::FREE => acc,
    });

    while greatest_file_id >= 0 {
        let block_infos = blocks_to_block_infos(&res);
        let to_move = block_infos.iter().find(|b| b.block_type == Block::FILE(greatest_file_id.try_into().unwrap())).unwrap();

        if let Some(free) = block_infos.iter().find(|b| b.block_type == Block::FREE && b.len >= to_move.len && b.start_pos < to_move.start_pos) {
            for i in free.start_pos .. free.start_pos + to_move.len {
                res[i] = to_move.block_type.to_owned();
            }
            for i in to_move.start_pos .. to_move.start_pos + to_move.len {
                res[i] = free.block_type.to_owned();
            }
        }
        greatest_file_id -= 1;
    }

    res
}

fn checksum(blocks: &Vec<Block>) -> u64 {
    blocks.iter().enumerate().fold(0_u64, |acc, (id, val)| {
        let block_checksum_val = match val {
            Block::FILE(v) => *v,
            Block::FREE => 0,
        };
        acc + u64::try_from(id).unwrap() * u64::from(block_checksum_val)
    })
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

    let formatted = move_blocks_to_left(&blocks);
    let res =  checksum(&formatted);
    println!("Checksum for moving blocks only: {}", res);

    let formatted2 = move_whole_files(&blocks);
    let res2 = checksum(&formatted2);
    println!("Checksum for moving whole files: {}", res2);
}
