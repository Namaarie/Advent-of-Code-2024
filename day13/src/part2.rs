use regex::Regex;
use std::fs::*;

use crate::part1::solve;

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();
    let re = Regex::new(r"(?<c1>[0-9]+)(.*?)(?<c2>[0-9]+)").unwrap();

    let mut nums = Vec::new();

    for cap in re.captures_iter(&binding) {
        let c1 = cap["c1"].parse::<u64>().unwrap();
        let c2 = cap["c2"].parse::<u64>().unwrap();

        nums.push(c1);
        nums.push(c2);
    }

    let mut solutions = Vec::new();
    let mut minimum = 0;

    while nums.len() > 0 {
        let k2 = nums.pop().unwrap();
        let k1 = nums.pop().unwrap();
        let c4 = nums.pop().unwrap();
        let c3 = nums.pop().unwrap();
        let c2 = nums.pop().unwrap();
        let c1 = nums.pop().unwrap();
        if let Some(solution) = solve((c1, c3, c2, c4), (k1+10000000000000, k2+10000000000000)) {
            minimum += solution.0 * 3 + solution.1;
            solutions.push(solution);
        }
    }

    println!("{}", minimum);
}
