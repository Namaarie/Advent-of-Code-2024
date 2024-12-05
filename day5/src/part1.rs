use std::{collections::HashMap, fs::read_to_string};

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();

    let mut dependency: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut middle_sum = 0;

    for line in binding.lines() {
        if line.contains("|") {
            let nums: Vec<i32> = line.split("|").map(|str| str.parse().unwrap()).collect();
            dependency
                .entry(nums[0])
                .or_insert_with(|| Vec::new())
                .push(nums[1]);
        }

        if line.contains(",") {
            let update: Vec<i32> = line
                .split(",")
                .into_iter()
                .map(|str| str.parse().unwrap())
                .collect();
            let result = (0..update.len())
                .map(|i| {
                    let empty: Vec<i32> = vec![];
                    let dependencies = dependency.get(&update[i]).unwrap_or(&empty);
                    let rest = &update[(i + 1)..];
                    rest.iter().filter(|num| dependencies.contains(num)).count() == rest.len()
                })
                .fold(true, |acc, res| acc && res);

            if result == true {
                // find middle value and add it
                let mid = update.len() / 2;
                middle_sum += update[mid];
            }
        }
    }

    println!("{:?}", middle_sum);
}
