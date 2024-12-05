use std::fs::read_to_string;

pub fn check_if_safe(nums: Vec<i32>) -> bool {
    let mut is_ascending = false;
    let mut is_descending = false;
    let mut is_safe = true;

    for i in 1..nums.len() {
        let difference = nums[i] - nums[i - 1];
        is_ascending = is_ascending || difference > 0;
        is_descending = is_descending || difference < 0;

        if difference.abs() > 3 || difference == 0 || (is_ascending && is_descending) {
            is_safe = false;
            break;
        }
    }

    is_safe
}

pub fn main() {
    let safe_count = read_to_string("input.txt").unwrap()
        .lines()
        .map(|line| {
            check_if_safe(
                line.split_ascii_whitespace()
                    .map(|str| str.parse::<i32>().unwrap())
                    .collect::<Vec<_>>(),
            )
        })
        .filter(|s| *s == true)
        .count();

    println!("{}", safe_count);
}
