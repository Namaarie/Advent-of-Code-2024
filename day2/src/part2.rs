use crate::part1::check_if_safe;
use std::fs::read_to_string;

pub fn main() {
    let num_safe = read_to_string("input.txt")
        .unwrap()
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .map(|str| str.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|original| {
            let mut perms = vec![original.clone()];
            perms.extend((0..original.len()).map(|i| {
                let mut new = original.clone();
                new.remove(i);
                new
            }));

            perms.into_iter().any(|nums| check_if_safe(nums))
        })
        .filter(|s| *s == true)
        .count();

    println!("{}", num_safe);
}
