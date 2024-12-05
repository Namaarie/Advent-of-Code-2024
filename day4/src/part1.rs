use std::{fs::read_to_string, panic::{self, catch_unwind}};

pub fn check_if_xmas(input: Vec<char>, counter: &mut i32) {
    let string = input.iter().collect::<String>();
    if string == "XMAS" || string == "SAMX" {
        *counter += 1;
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
            let mut h_chars = Vec::new();
            let mut v_chars = Vec::new();
            let mut d_chars = Vec::new();
            let mut d_chars2 = Vec::new();
            
            for i in 0..4 {
                if let Ok(c) = catch_unwind(|| lines[y][x+i]) {
                    h_chars.push(c);
                }

                if let Ok(c) = catch_unwind(|| lines[y+i][x]) {
                    v_chars.push(c);
                }

                if let Ok(c) = catch_unwind(|| lines[y+i][x+i]) {
                    d_chars.push(c);
                }

                if let Ok(c) = catch_unwind(|| lines[y+(3-i)][x+i]) {
                    d_chars2.push(c);
                }
            }

            check_if_xmas(h_chars, &mut count);
            check_if_xmas(v_chars, &mut count);
            check_if_xmas(d_chars, &mut count);
            check_if_xmas(d_chars2, &mut count);
        }
    }

    println!("{}", count);
}