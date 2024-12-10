use std::{
    fs::*,
    iter::zip,
    panic::{self, catch_unwind},
};

pub fn main() {
    panic::set_hook(Box::new(|_info| {
        // do nothing
    }));

    let binding = &read_to_string("input.txt").unwrap();
    let mut world = Vec::new();

    let mut traverse_stack = Vec::new();

    for (y, line) in zip(
        0..binding.lines().collect::<Vec<_>>().len(),
        binding.lines(),
    ) {
        let mut new_line = Vec::new();
        for (x, char) in zip(0..line.len(), line.chars()) {
            if char == '0' {
                traverse_stack.push((x, y));
            }
            new_line.push(char);
        }
        world.push(new_line);
    }

    let mut trailhead_sum = 0;

    while let Some(pos) = traverse_stack.pop() {
        let (x, y) = pos;
        let val = world[y][x];

        if val == '9' {
            trailhead_sum += 1;
            continue;
        }

        // adds neighbors to stack only if the side is val+1
        if let Ok(char) = catch_unwind(|| world[y][x + 1]) {
            if char == (val as u8 + 1) as char {
                traverse_stack.push((x + 1, y));
            }
        }

        if let Ok(char) = catch_unwind(|| world[y][x - 1]) {
            if char == (val as u8 + 1) as char {
                traverse_stack.push((x - 1, y));
            }
        }

        if let Ok(char) = catch_unwind(|| world[y + 1][x]) {
            if char == (val as u8 + 1) as char {
                traverse_stack.push((x, y + 1));
            }
        }

        if let Ok(char) = catch_unwind(|| world[y - 1][x]) {
            if char == (val as u8 + 1) as char {
                traverse_stack.push((x, y - 1));
            }
        }
    }

    println!("{}", trailhead_sum);
}
