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

    let result = left.iter().zip(right.iter()).map(|(l, r)| {
       (l.parse::<i32>().unwrap() - r.parse::<i32>().unwrap()).abs()
    }).sum::<i32>();

    println!("{}", result);
}
