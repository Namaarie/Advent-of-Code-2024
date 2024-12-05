use std::{
    fs::read_to_string,
    panic::{self, catch_unwind},
};

use regex::Regex;

pub fn main() {
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

    let input = &read_to_string("input.txt").unwrap();
    let re = Regex::new(r"(?<entire>mul\((?<first>[0-9]{1,3}),(?<second>[0-9]{1,3})\))").unwrap();

    let mut sum = 0;
    let mut mul_enabled = true;
    let mut i = 0;

    while i < input.len() - 4 {
        let do_slice = &input[i..(i + 4)];

        if do_slice == "do()" {
            mul_enabled = true;
        }

        if let Ok(dont_slice) = catch_unwind(|| &input[i..(i + 7)]) {
            if dont_slice == "don't()" {
                mul_enabled = false;
            }
        }

        if mul_enabled {
            if let Ok(slice) = catch_unwind(|| &input[i..(i + 12)]) {
                for cap in re.captures_iter(slice) {
                    let first: i32 = cap["first"].parse().unwrap();
                    let second: i32 = cap["second"].parse().unwrap();
                    let entire = &cap["entire"];
                    sum += first * second;
                    i += entire.len() - 1;
                }
            }
        }

        i += 1;
    }

    println!("{}", sum);
}
