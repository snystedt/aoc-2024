use std::collections::{BTreeSet, HashSet};

use aoc_2024::{
    input::read_lines,
    utils::{CoordVec, Direction, Grid, DIRECTIONS},
};
use itertools::Itertools;

fn print_grid(grid: &Grid<u8>) {
    grid.rows()
        .for_each(|row| println!("{}", row.iter().map(|s| *s as char).join(", ")));
}

fn find_gardens(grid: &Grid<u8>) {
    let mut gardens = vec![];
    let mut visited = HashSet::new();
    let mut tasks = BTreeSet::new();
    tasks.insert((0usize, 0usize));

    while let Some(task) = tasks.pop_first() {
        let plant = grid.get(task);

        let mut to_visit: Vec<(usize, usize)> = vec![task];
        let mut perimeter: HashSet<((i32, i32), (i32, i32))> = HashSet::new();
        let mut area: HashSet<(usize, usize)> = HashSet::new();
        area.insert(task);

        while let Some(curr) = to_visit.pop() {
            visited.insert(curr);
            DIRECTIONS.iter().for_each(|dir| {
                if let Some(neighbour_idx) =
                    CoordVec(dir.to_vec2()).transform_coord(curr, grid.size())
                {
                    if false && plant == &b'R' {
                        println!("R - current: {:?}, neighbour: {:?}", curr, neighbour_idx);
                    }
                    if grid.get(neighbour_idx) == plant {
                        tasks.remove(&neighbour_idx);
                        area.insert(neighbour_idx);
                        if !visited.contains(&neighbour_idx) {
                            to_visit.push(neighbour_idx);
                        }
                    } else {
                        perimeter.insert((
                            (curr.0 as i32, curr.1 as i32),
                            (neighbour_idx.0 as i32, neighbour_idx.1 as i32),
                        ));
                        if !visited.contains(&neighbour_idx) {
                            tasks.insert(neighbour_idx);
                        }
                    }
                } else {
                    perimeter.insert((
                        (curr.0 as i32, curr.1 as i32),
                        match dir {
                            Direction::Up => (-1, curr.1 as i32),
                            Direction::Down => (grid.size().0 as i32, curr.1 as i32),
                            Direction::Left => (curr.0 as i32, -1),
                            Direction::Right => (curr.0 as i32, grid.size().1 as i32),
                        },
                    ));
                }
            });
        }
        gardens.push((area, perimeter));
    }

    if false {
        gardens.iter().for_each(|garden| {
            let pos = *garden.0.iter().next().expect("Empty garden area");
            let plant = grid.get(pos);

            println!(
                "========= Plant {} ({}, {}) =========",
                *plant as char, pos.0, pos.1
            );
            println!("Area: {:?}", garden.0.len());
            println!("Perimeter count: {:?}", garden.1.len());
        });
    }

    let cost = gardens
        .iter()
        .map(|garden| garden.0.len() * garden.1.len())
        .sum::<usize>();

    println!("Total cost: {}", cost);

    let cost_with_sides = gardens
        .iter()
        .map(|garden| {
            let perimiter = &garden.1;

            let cmp_fns = [
                |this: &(i32, i32), other: &(i32, i32)| (other.0 > this.0).then_some(*this),
                |this: &(i32, i32), other: &(i32, i32)| (this.0 > other.0).then_some(*this),
                |this: &(i32, i32), other: &(i32, i32)| {
                    (other.1 > this.1).then_some((this.1, this.0))
                },
                |this: &(i32, i32), other: &(i32, i32)| {
                    (this.1 > other.1).then_some((this.1, this.0))
                },
            ];

            let mut sides = 0usize;
            for cmp_fn in cmp_fns {
                let mut v = perimiter
                    .iter()
                    .filter_map(|(this, other)| cmp_fn(this, other))
                    .collect_vec();
                v.sort();

                let mut last = v[0];

                for p in v {
                    if last.0 != p.0 {
                        sides += 1;
                    } else if p.1 - last.1 > 1 {
                        sides += 1;
                    }

                    last = p;
                }

                sides += 1;
            }

            //println!("Sides: {}", sides);

            sides * garden.0.len()
        })
        .sum::<usize>();

    println!("Total cost with sides: {}", cost_with_sides);
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day12/input.txt") {
        // Parsing
        let input = lines.flatten().collect::<Vec<_>>();
        assert!(!input.is_empty() && !input[0].is_empty());
        let (rows, cols) = (input.len(), input[0].len());

        let mut grid = Grid::<u8>::zeros(rows, cols);
        input.iter().enumerate().for_each(|(i, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .for_each(|(j, &b)| grid.set((i, j), b))
        });

        print_grid(&grid);

        find_gardens(&grid);
    }
}
