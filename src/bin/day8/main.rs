use std::collections::{HashMap, HashSet};

use aoc_2024::input::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./inputs/day8/input.txt") {
        // Parsing
        let input = lines.flatten().collect::<Vec<_>>();

        assert!(!input.is_empty());
        assert!(!input[0].is_empty());

        let bounds = (input.len() as i32, input[0].len() as i32);
        let mut antennas = HashMap::<u8, Vec<(i32, i32)>>::new();

        input.into_iter().enumerate().for_each(|(i, col)| {
            col.as_bytes()
                .iter()
                .enumerate()
                .filter(|(_, c)| *c != &b'.')
                .for_each(|(j, c)| {
                    if let Some(v) = antennas.get_mut(c) {
                        v.push((i as i32, j as i32));
                    } else {
                        antennas.insert(*c, vec![(i as i32, j as i32)]);
                    }
                })
        });

        let out_of_bounds =
            |pos: &(i32, i32)| pos.0 < 0 || pos.0 >= bounds.0 || pos.1 < 0 || pos.1 >= bounds.1;

        let mut antinodes = HashSet::<(i32, i32)>::new();
        antennas.iter().for_each(|(_, v)| {
            let length = v.len();

            for i in 0..length {
                for j in i + 1..length {
                    let (first, second) = (v[i], v[j]);

                    let (d_y, d_x) = (second.0 - first.0, second.1 - first.1);

                    let mut antinode = second;
                    while !out_of_bounds(&antinode) {
                        antinodes.insert(antinode);

                        antinode = (antinode.0 + d_y, antinode.1 + d_x);
                    }

                    let mut antinode = first;
                    while !out_of_bounds(&antinode) {
                        antinodes.insert(antinode);

                        antinode = (antinode.0 - d_y, antinode.1 - d_x);
                    }
                }
            }
        });

        println!("Num antinodes: {}", antinodes.len());
    }
}
