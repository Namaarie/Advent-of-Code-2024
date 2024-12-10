use crate::part1::BlockType;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap};
use std::fs::*;

pub fn main() {
    let line = read_to_string("input.txt").unwrap();
    let mut blocks: Vec<Option<u32>> = Vec::new();
    let mut block_type = BlockType::FILE;
    let mut value = 0;
    let mut index: usize = 0;
    let mut file_blocks = Vec::new();
    let mut free_map: HashMap<u32, BinaryHeap<Reverse<usize>>> = HashMap::new();

    for c in line.chars() {
        let amt = c.to_digit(10).unwrap();
        match block_type {
            BlockType::FREE => {
                // adds free slot to map
                free_map
                    .entry(amt)
                    .or_insert_with(|| BinaryHeap::new())
                    .push(Reverse(index as usize));

                for _ in 0..amt {
                    blocks.push(None);
                    index += 1;
                }

                block_type = BlockType::FILE;
            }
            BlockType::FILE => {
                file_blocks.push((amt, index));

                for _ in 0..amt {
                    blocks.push(Some(value));
                    index += 1;
                }

                block_type = BlockType::FREE;
                value += 1;
            }
        }
    }

    // gets keys in sorted array
    let clone = free_map.clone();
    let mut keys = clone.keys().collect::<Vec<_>>();
    keys.sort();

    // for each file block
    while let Some((amt, index)) = file_blocks.pop() {
        // find left most empty space that can fit amt required
        let viable_start = keys.iter().position(|k| **k >= amt).unwrap_or(0);
        let viable_keys = &keys[viable_start..];
        let mut min_key = viable_keys.get(0).unwrap();
        let mut min_index = std::usize::MAX;

        // for each viable slot size, checks first element in heap to find min index
        for key in viable_keys {
            if let Some(free_heap) = &free_map.get_mut(&key) {
                if let Some(slot) = free_heap.peek() {
                    if slot.0 < min_index {
                        min_index = free_heap.peek().unwrap().0;
                        min_key = key;
                    }
                }
            }
        }

        // found min key/index
        // pop the min out of heap
        let _ = free_map.get_mut(&min_key).unwrap().pop();

        // swaps with file only if moving file to left
        if min_index < index {
            for i in 0..amt as usize {
                blocks.swap(min_index + i, index + i);
            }
        }

        // if size of slot is greater than size of file
        if **min_key > amt {
            let new_index = min_index + amt as usize;
            let new_amt = **min_key - amt;

            free_map
                .entry(new_amt)
                .or_insert_with(|| BinaryHeap::new())
                .push(Reverse(new_index));
        }
    }
    let sum = blocks
        .iter()
        .enumerate()
        .map(|(i, o)| o.unwrap_or(0) as u64 * i as u64)
        .sum::<u64>();
    println!("{}", sum);
}
