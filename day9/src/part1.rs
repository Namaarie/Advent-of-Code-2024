use std::fs::*;

pub enum BlockType {
    FREE,
    FILE,
}

#[allow(unused)]
pub fn print_blocks(blocks: &Vec<Option<u32>>) {
    let str = blocks
        .iter()
        .map(|o| {
            if let Some(c) = o {
                c.to_string()
            } else {
                ".".to_string()
            }
        })
        .collect::<String>();
    println!("{}", str);
}

pub fn main() {
    let line = read_to_string("input.txt").unwrap();
    let mut blocks: Vec<Option<u32>> = Vec::new();
    let mut block_type = BlockType::FILE;
    let mut index = 0;
    for c in line.chars() {
        let amt = c.to_digit(10).unwrap();
        for _ in 0..amt {
            match block_type {
                BlockType::FREE => blocks.push(None),
                BlockType::FILE => blocks.push(Some(index)),
            }
        }
        match block_type {
            BlockType::FREE => block_type = BlockType::FILE,
            BlockType::FILE => {
                block_type = BlockType::FREE;
                index += 1;
            }
        }
    }

    let mut left = 0;
    let mut right = blocks.len() - 1;
    while left < right {
        // increment left until reaches free space
        while let Some(_) = blocks[left] {
            left += 1;
        }

        // decrement right until reaches non free space
        while let None = blocks[right] {
            right -= 1;
        }

        if left < right {
            blocks.swap(left, right);
        }
    }

    let sum = blocks
        .iter()
        .enumerate()
        .map(|(i, o)| o.unwrap_or(0) as u64 * i as u64)
        .sum::<u64>();

    println!("{}", sum);
}
