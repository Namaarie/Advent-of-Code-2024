use std::fs::*;

fn iterate(nums: Vec<u64>) -> Vec<u64> {
    let mut new_nums: Vec<u64> = Vec::new();
    for num in nums {
        // rule 1
        if num == 0 {
            new_nums.push(1);
        }
        // rule 2
        else if num.to_string().len() % 2 == 0 {
            let num_str = num.to_string();
            let mid = num_str.len() / 2;
            new_nums.push(num_str[0..mid].parse().unwrap());
            new_nums.push(num_str[mid..].parse().unwrap());
        } else {
            new_nums.push(num * 2024);
        }
    }

    new_nums
}

pub fn main() {
    let binding: String = read_to_string("input.txt").unwrap();

    let mut nums: Vec<u64> = binding
        .split_ascii_whitespace()
        .into_iter()
        .map(|str| str.parse().unwrap())
        .collect::<Vec<_>>();

    for _ in 0..25 {
        nums = iterate(nums);
    }

    println!("{}", nums.len());
}
