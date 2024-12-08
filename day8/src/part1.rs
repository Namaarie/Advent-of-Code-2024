use std::{collections::HashSet, fs::*, iter::zip};

pub fn main() {
    let binding = &read_to_string("input.txt").unwrap();
    let mut antennas: Vec<char> = Vec::new();
    let mut posx: Vec<i32> = Vec::new();
    let mut posy: Vec<i32> = Vec::new();
    let width = binding.lines().nth(0).unwrap().len();
    let height = binding.lines().collect::<Vec<_>>().len();

    // loads world  
    for (y, line) in zip(
        0..binding.lines().collect::<Vec<_>>().len(),
        binding.lines(),
    ) {
        for (x, char) in zip(0..line.len(), line.chars()) {
            if char.is_ascii_alphanumeric() {
                antennas.push(char);
                posx.push(x as i32);
                posy.push(y as i32);
            }
        }
    }

    let mut antinodes = HashSet::new();

    let len = antennas.len();
    for (i, antenna1) in zip(0..len,&antennas) {
        for (j, antenna2) in zip(0..len,&antennas) {
            // skip same indices
            if i == j {
                continue;
            }

            // skip different frequencies
            if antenna1 != antenna2 {
                continue;
            }

            // calculate vector
            let dx = posx[j] - posx[i];
            let dy = posy[j] - posy[i];

            let antinodex = posx[j] + dx;
            let antinodey = posy[j] + dy;

            if antinodex >= 0 && antinodex < width as i32 && antinodey >= 0 && antinodey < height as i32 {
                antinodes.insert((antinodex, antinodey));
            }
        }
    }

    println!("{}", antinodes.len());
}