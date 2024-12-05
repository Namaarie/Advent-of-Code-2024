use regex::Regex;
use std::fs::read_to_string;

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();
    let re = Regex::new(r"mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\)").unwrap();

    let mut sum = 0;

    for cap in re.captures_iter(&binding) {
        let first: i32 = cap["first"].parse().unwrap();
        let second: i32 = cap["second"].parse().unwrap();
        sum += first * second;
    }

    println!("{}", sum);
}
