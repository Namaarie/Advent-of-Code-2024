use std::fs::*;

enum BlockType {
    FREE,
    FILE,
}

pub fn main() {
    let line = read_to_string("input.txt").unwrap();
    let mut blocks: Vec<Option<u32>> = Vec::new();
    let mut block_type = BlockType::FILE;
    let mut value = 0;
    let mut index = 0;
    let mut block_stack = Vec::new();
    for c in line.chars() {
        let amt = c.to_digit(10).unwrap();
        block_stack.push((amt, index));
        for _ in 0..amt {
            match block_type {
                BlockType::FREE => blocks.push(None),
                BlockType::FILE => {
                    blocks.push(Some(value));
                },
            };
            index += 1;
        }
        match block_type {
            BlockType::FREE => block_type = BlockType::FILE,
            BlockType::FILE => {
                block_type = BlockType::FREE;
                value += 1;
            },
        }
    }

    while let Some((occurences, index)) = block_stack.pop() {
        // find left most empty space that can fit occurences
        let mut left = 0;
        let mut spaces = 0;
        'outer: while left < index {
            match blocks[left] {
                None => {
                    spaces += 1;
                },
                Some(_) => {
                    spaces = 0
                },
            };
            left += 1;
            if spaces >= occurences {
                // empty start = left - spaces
                let start = left - spaces as usize;
                for i in 0..occurences {
                    blocks.swap(start+i as usize, index+i as usize);
                }
                break 'outer;
            }
        }
    }

    let sum = blocks.iter().enumerate().map(|(i, o)| o.unwrap_or(0) as u64 * i as u64).sum::<u64>();
    println!("{}", sum);
}