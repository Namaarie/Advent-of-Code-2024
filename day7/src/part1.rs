use std::fs::read_to_string;

fn solve(mut nums: Vec<u64>, goal: u64) -> bool {
    let mut possibilities: Vec<u64> = Vec::new();
    nums.reverse();

    while let Some(num) = nums.pop() {
        let mut p2: Vec<u64> = Vec::new();
        if possibilities.len() == 0 {
            p2.push(num);
        }
        while let Some(p) = possibilities.pop() {
            p2.push(num + p);
            p2.push(num * p);
        }
        possibilities = p2;
    }

    possibilities.contains(&goal)
}

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();

    let mut sum = 0;

    for line in binding.lines() {
        let goal = line.split(": ").nth(0).unwrap().parse::<u64>().unwrap();
        let nums = line.split(": ").nth(1).unwrap().split(" ").into_iter().map(|str| str.parse::<u64>().unwrap()).collect::<Vec<_>>();

        if solve(nums.clone(), goal) {
            sum += goal;
        }    
    }

    println!("{}", sum);
}