use std::{collections::HashMap, fs::read_to_string};

fn check_if_broken(update: &Vec<i32>, dependency: HashMap<i32, Vec<i32>>) -> bool {
    let result = (0..update.len())
        .map(|i| {
            let empty: Vec<i32> = vec![];
            let dependencies = dependency.get(&update[i]).unwrap_or(&empty);
            let rest = &update[(i + 1)..];
            rest.iter().filter(|num| dependencies.contains(num)).count() == rest.len()
        })
        .fold(true, |acc, res| acc && res);
    !result
}

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();

    let mut dependency: HashMap<i32, Vec<i32>> = HashMap::new();

    let mut broken_but_fixed = Vec::new();

    for line in binding.lines() {
        if line.contains("|") {
            let nums: Vec<i32> = line.split("|").map(|str| str.parse().unwrap()).collect();
            dependency
                .entry(nums[0])
                .or_insert_with(|| Vec::new())
                .push(nums[1]);
        }

        if line.contains(",") {
            let mut update: Vec<i32> = line
                .split(",")
                .into_iter()
                .map(|str| str.parse().unwrap())
                .collect();

            let broken = check_if_broken(&update, dependency.clone());

            if !broken {
                continue;
            }

            while check_if_broken(&update, dependency.clone()) {
                let mut clone = update.clone();
                for i in 0..update.len() {
                    let empty: Vec<i32> = vec![];
                    let dependencies = dependency.get(&update[i]).unwrap_or(&empty);
                    let rest = &update[(i + 1)..];

                    for j in 0..rest.len() {
                        if !dependencies.contains(&rest[j]) {
                            clone.swap(i, i + (j + 1));
                        }
                    }
                }
                update = clone;
            }

            broken_but_fixed.push(update);
        }
    }

    let middle_sum: i32 = broken_but_fixed
        .iter()
        .map(|update| {
            let mid = update.len() / 2;
            update[mid]
        })
        .sum();

    println!("{:?}", middle_sum);
}
