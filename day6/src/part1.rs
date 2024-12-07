use std::{fs::read_to_string, iter::zip};

enum Direction {
    UP,
    DOWN,
    LEFT,
    RIGHT,
}

fn turn_right(dir: Direction) -> Direction {
    match dir {
        Direction::UP => Direction::RIGHT,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
        Direction::RIGHT => Direction::DOWN,
    }
}

pub fn main() {
    let binding = &read_to_string("input.txt").unwrap();
    let mut world = Vec::new();
    let mut cur_direction = Direction::UP;
    let mut posx: i32 = 0;
    let mut posy: i32 = 0;

    for (y, line) in zip(
        0..binding.lines().collect::<Vec<_>>().len(),
        binding.lines(),
    ) {
        let mut new_line = Vec::new();
        for (x, char) in zip(0..line.len(), line.chars()) {
            if char == '^' {
                posx = x as i32;
                posy = y as i32;
                new_line.push('.');
            } else {
                new_line.push(char);
            }
        }
        world.push(new_line);
    }

    // iter until leaves map
    let mut in_map = true;
    const BORDER: char = '%';
    let mut num_x = 0;

    while in_map {
        let fronty: i32 = match cur_direction {
            Direction::UP => posy - 1,
            Direction::DOWN => posy + 1,
            _ => posy,
        };
        let frontx: i32 = match cur_direction {
            Direction::LEFT => posx - 1,
            Direction::RIGHT => posx + 1,
            _ => posx,
        };
        let in_front: char = {
            if let Some(slice) = world.get(fronty as usize) {
                *slice.get(frontx as usize).unwrap_or(&BORDER)
            } else {
                BORDER
            }
        };

        if in_front == '#' {
            // rotate
            cur_direction = turn_right(cur_direction);
        } else if in_front == BORDER {
            in_map = false;
            num_x += 1;
        } else {
            if !(world[posy as usize][posx as usize] == 'X') {
                num_x += 1;
            }
            world[posy as usize][posx as usize] = 'X';
            posx = frontx;
            posy = fronty;
        }
    }

    println!("{}", num_x);
}
