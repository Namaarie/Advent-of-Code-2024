use std::fs::read_to_string;

pub fn main() {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let binding = read_to_string("input.txt").unwrap();
    for line in binding.lines() {
        left.push(line.split_ascii_whitespace().nth(0).unwrap());
        right.push(line.split_ascii_whitespace().nth(1).unwrap());
    }

    left.sort();
    right.sort();

    let left = left.iter().map(|l| l.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let right = right.iter().map(|r| r.parse::<i32>().unwrap()).collect::<Vec<_>>();

    let sum = left.iter().map(|l| right.iter().filter(|r| *r == l).count() as i32 * l).sum::<i32>();

    println!("{}", sum);
}
