use std::{collections::HashSet, fs::*, iter::zip};

pub fn main() {
    let binding = read_to_string("input.txt").unwrap();
    let mut world: Vec<Vec<char>> = Vec::new();
    let mut need_to_visit: Vec<(usize, usize)> = Vec::new();

    // loads world
    for (y, line) in zip(
        0..binding.lines().collect::<Vec<_>>().len(),
        binding.lines(),
    ) {
        world.push(line.chars().collect::<Vec<char>>());
        for x in 0..line.len() {
            need_to_visit.push((x, y));
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
            if world[node.1][node.0] == char {
                visited.insert(node);

                area += 1;
                perimeter += 4;

                // pushes neighbors onto stack
                // up
                if node.1 > 0 {
                    if world[node.1 - 1][node.0] == char {
                        flood_fill_stack.push((node.0, node.1 - 1));
                        perimeter -= 1;
                    }
                }

                // down
                if node.1 + 1 < world.len() {
                    if world[node.1 + 1][node.0] == char {
                        flood_fill_stack.push((node.0, node.1 + 1));
                        perimeter -= 1;
                    }
                }

                // left
                if node.0 > 0 {
                    if world[node.1][node.0 - 1] == char {
                        flood_fill_stack.push((node.0 - 1, node.1));
                        perimeter -= 1;
                    }
                }

                // right
                if node.0 + 1 < world[node.1].len() {
                    if world[node.1][node.0 + 1] == char {
                        flood_fill_stack.push((node.0 + 1, node.1));
                        perimeter -= 1;
                    }
                }
            }
        }
        total += area * perimeter;
    }

    println!("{}", total);
}
