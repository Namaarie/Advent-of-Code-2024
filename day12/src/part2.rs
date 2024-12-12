use std::{collections::HashSet, fs::*, iter::zip};

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();
    let mut world: Vec<Vec<char>> = Vec::new();
    let mut need_to_visit: Vec<(usize, usize)> = Vec::new();

    let height = binding.lines().collect::<Vec<_>>().len() + 2;
    let width = binding.lines().collect::<Vec<_>>().first().unwrap().len() + 2;

    // preps large world
    for _ in 0..height {
        let mut row = Vec::new();
        for _ in 0..width {
            row.push('*');
        }
        world.push(row);
    }

    // loads world
    for (y, line) in zip(
        0..binding.lines().collect::<Vec<_>>().len(),
        binding.lines(),
    ) {
        for (x, char) in zip(0..line.len(), line.chars()) {
            world[y + 1][x + 1] = char;
            need_to_visit.push((x + 1, y + 1));
        }
    }

    let mut total = 0;
    let mut visited: HashSet<(usize, usize)> = HashSet::new();

    while let Some(tile) = need_to_visit.pop() {
        if visited.contains(&tile) {
            continue;
        }

        // floodfill until found all connecting tiles
        let mut flood_fill_stack: Vec<(usize, usize)> = Vec::new();
        let char = world[tile.1][tile.0];
        flood_fill_stack.push(tile);
        let mut area = 0;
        let mut perimeter = 0;

        while let Some(node) = flood_fill_stack.pop() {
            if visited.contains(&node) {
                continue;
            }
            visited.insert(node);

            // pushes neighbors onto stack
            // up
            if world[node.1 - 1][node.0] == char {
                flood_fill_stack.push((node.0, node.1 - 1));
            }

            // down
            if world[node.1 + 1][node.0] == char {
                flood_fill_stack.push((node.0, node.1 + 1));
            }

            // left
            if world[node.1][node.0 - 1] == char {
                flood_fill_stack.push((node.0 - 1, node.1));
            }

            // right
            if world[node.1][node.0 + 1] == char {
                flood_fill_stack.push((node.0 + 1, node.1));
            }

            // top left
            let mut b = Vec::new();
            for y in 0..2 {
                for x in 0..2 {
                    b.push(world[node.1 - 1 + y][node.0 - 1 + x]);
                }
            }
            if (b[1] != char && char != b[2])
                || (char == b[2] && b[3] == char && char == b[1] && b[0] != char)
            {
                perimeter += 1;
            }

            // bottom left

            let mut b = Vec::new();
            for y in 0..2 {
                for x in 0..2 {
                    b.push(world[node.1 + y][node.0 - 1 + x]);
                }
            }
            if (b[3] != char && char != b[0])
                || (char == b[0] && b[1] == char && char == b[3] && b[2] != char)
            {
                perimeter += 1;
            }

            // top right

            let mut b = Vec::new();
            for y in 0..2 {
                for x in 0..2 {
                    b.push(world[node.1 - 1 + y][node.0 + x]);
                }
            }
            if (b[0] != char && char != b[3])
                || (char == b[3] && b[2] == char && char == b[0] && b[1] != char)
            {
                perimeter += 1;
            }

            // bottom right

            let mut b = Vec::new();
            for y in 0..2 {
                for x in 0..2 {
                    b.push(world[node.1 + y][node.0 + x]);
                }
            }
            if (b[2] != char && char != b[1])
                || (char == b[2] && b[0] == char && char == b[1] && b[3] != char)
            {
                perimeter += 1;
            }

            area += 1;
        }
        total += area * perimeter;
    }

    println!("{}", total);
}
