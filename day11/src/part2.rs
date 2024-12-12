use std::{collections::HashMap, fs::*};

fn solve(num: u64, iterations: usize, memory: &mut HashMap<(usize, u64), u64>) -> u64 {
    // base case
    // if 0 iteration, then length is 1 since 1 number
    if iterations == 0 {
        return 1;
    }

    // checks memory
    if let Some(memorized) = memory.get(&(iterations, num)) {
        return *memorized;
    }

    // calculates next iteration
    let mut new_nums = Vec::new();

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
    }
    // rule 3
    else {
        new_nums.push(num * 2024);
    }

    // recursive case
    let mut result = 0;
    for new_num in new_nums {
        result += solve(new_num, iterations - 1, memory);
    }

    // insert into memory
    memory.entry((iterations, num)).or_insert_with(|| result);

    // return result
    result
}

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();
    let mut memory = HashMap::new();

    let result: u64 = binding
        .split_ascii_whitespace()
        .into_iter()
        .map(|str| solve(str.parse().unwrap(), 75, &mut memory))
        .sum();

    println!("{}", result);
}
