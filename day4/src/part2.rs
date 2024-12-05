use std::{fs::read_to_string, panic::{self, catch_unwind}};

pub fn check_if_mas(input: Vec<char>) -> bool {
    let string = input.iter().collect::<String>();
    if string == "MAS" || string == "SAM" {
        true
    } else {
        false
    }
}

pub fn main() {
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));
    
    let binding = &read_to_string("input.txt").unwrap();

    let mut lines = Vec::new();

    for line in binding.lines() {
        let mut new_line = Vec::new();
        for char in line.chars() {
            new_line.push(char);
        }
        lines.push(new_line);
    }

    let mut count: i32 = 0;

    for y in 0..lines.len() {
        for x in 0..lines[y].len() {
            let mut d_chars = Vec::new();
            let mut d_chars2 = Vec::new();

            for i in 0..3 {
                if let Ok(c) = catch_unwind(|| lines[y+i][x+i]) {
                    d_chars.push(c);
                }

                if let Ok(c) = catch_unwind(|| lines[y+(2-i)][x+i]) {
                    d_chars2.push(c);
                }
            }

            if check_if_mas(d_chars) && check_if_mas(d_chars2) {
                count += 1;
            }
        }
    }

    println!("{}", count);
}