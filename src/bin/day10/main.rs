use std::collections::{HashMap, HashSet, VecDeque};

use aoc_2024::input::read_lines;
use aoc_2024::utils::{Coord, CoordVec, Grid, DIRECTIONS};

fn calculate_trails(grid: &Grid<u8>) -> HashMap<Coord, Vec<Vec<Coord>>> {
    let zeros = grid
        .indexed_iter()
        .filter_map(|(c, v)| (*v == 0).then_some(c))
        .collect::<Vec<Coord>>();

    let mut trailhead_trails = HashMap::new();

    for start in zeros.into_iter() {
        let mut partial_trails: VecDeque<Vec<Coord>> = VecDeque::new();
        let mut trails = vec![];

        partial_trails.push_front(vec![start]);

        while let Some(trail) = partial_trails.pop_front() {
            let curr = *trail.last().unwrap();
            let height = grid.get(curr);
            for dir in DIRECTIONS {
                if let Some(neighbour) = CoordVec(dir.to_vec2()).transform_coord(curr, grid.size())
                {
                    let c = grid.get(neighbour);
                    if *c == *height + 1 {
                        let mut trail = trail.clone();
                        trail.push(neighbour);
                        if *c == 9 {
                            trails.push(trail);
                        } else {
                            partial_trails.push_front(trail);
                        }
                    }
                }
            }
        }

        trailhead_trails.insert(start, trails);
    }
    trailhead_trails
}

fn first_star(trails: &HashMap<Coord, Vec<Vec<Coord>>>) {
    let trailhead_peaks = trails
        .iter()
        .map(|(_, trails)| {
            let mut peaks = HashSet::<Coord>::new();
            trails.iter().for_each(|trail| {
                peaks.insert(*trail.last().unwrap());
            });
            peaks.len()
        })
        .sum::<usize>();

    println!(
        "Total number of peaks from each trailhead: {}",
        trailhead_peaks
    );
}

fn second_star(trails: &HashMap<Coord, Vec<Vec<Coord>>>) {
    let total_trails = trails.iter().map(|(_, trails)| trails.len()).sum::<usize>();

    println!(
        "Number of distinct trails from each trailhead: {}",
        total_trails
    );
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day10/input.txt") {
        // Parsing
        let input = lines.flatten().collect::<Vec<_>>();
        assert!(!input.is_empty() && !input[0].is_empty());
        for line in input.iter() {
            assert!(!line.is_empty());
            for c in line.chars() {
                assert!(c.is_digit(10));
            }
        }
        let (rows, cols) = (input.len(), input[0].len());

        let mut grid = Grid::<u8>::zeros(rows, cols);
        input.iter().enumerate().for_each(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .for_each(|(j, &b)| grid.set((i, j), b - 48))
        });

        let trails = calculate_trails(&grid);
        first_star(&trails);
        second_star(&trails);
    }
}
