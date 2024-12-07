use std::{fs::read_to_string, iter::zip};

#[derive(Clone, PartialEq)]
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

fn is_loop(
    mut cur_direction: Direction,
    world: Vec<Vec<char>>,
    mut posy: i32,
    mut posx: i32,
) -> bool {
    // iter until leaves map
    let mut in_map = true;
    const BORDER: char = '%';
    let mut world_directions: Vec<Vec<Vec<Direction>>> = Vec::new();

    for i in 0..world.len() {
        let mut vec = Vec::new();
        for _ in 0..world[i].len() {
            vec.push(Vec::<Direction>::new());
        }
        world_directions.push(vec);
    }

    while in_map {
        if world_directions[posy as usize][posx as usize].contains(&cur_direction) {
            return true;
        }
        world_directions[posy as usize][posx as usize].push(cur_direction.clone());

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
        } else {
            posx = frontx;
            posy = fronty;
        }
    }

    false
}

pub fn main() {
    let binding = &read_to_string("input.txt").unwrap();
    let mut world = Vec::new();
    let cur_direction = Direction::UP;
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

    let mut num_obstacles = 0;

    println!("slow approach");

    for y in 0..world.len() {
        for x in 0..world[y].len() {
            let mut clone = world.clone();
            clone[y][x] = '#';

            let result = is_loop(cur_direction.clone(), clone, posy, posx);
            if result {
                num_obstacles += 1;
            }
        }
    }

    println!("{}", num_obstacles);
}
