use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_2024::input::read_lines;
use itertools::Itertools;
#[derive(PartialEq, Eq, Clone, Copy)]
struct PathElement {
    pub score: usize,
    pub pos: (usize, usize),
}

impl Ord for PathElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| self.pos.cmp(&other.pos))
    }
}

impl PartialOrd for PathElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_shortest_path(
    start: (usize, usize),
    end: (usize, usize),
    bounds: (usize, usize),
    fallen_bytes: &HashSet<(usize, usize)>,
) -> Option<usize> {
    let mut q: BinaryHeap<PathElement> = BinaryHeap::new();

    let mut distances: HashMap<(usize, usize), usize> = HashMap::default();

    q.push(PathElement {
        score: 0,
        pos: start,
    });
    distances.insert(start, 0);

    let res = loop {
        if q.is_empty() {
            break None;
        }
        let path_elem = q.pop().unwrap();

        let (x, y) = path_elem.pos;

        if (x, y) == end {
            break Some(path_elem.score);
        }

        if x > 0 {
            let option = (x - 1, y);
            if !fallen_bytes.contains(&option)
                && path_elem.score + 1 < *distances.get(&option).unwrap_or(&usize::MAX)
            {
                let mut new_elem = path_elem;
                new_elem.score += 1;
                new_elem.pos = option;

                distances.insert(option, new_elem.score);
                q.push(new_elem);
            }
        }
        if x < bounds.0 - 1 {
            let option = (x + 1, y);
            if !fallen_bytes.contains(&option)
                && path_elem.score + 1 < *distances.get(&option).unwrap_or(&usize::MAX)
            {
                let mut new_elem = path_elem;
                new_elem.score += 1;
                new_elem.pos = option;

                distances.insert(option, new_elem.score);
                q.push(new_elem);
            }
        }
        if y > 0 {
            let option = (x, y - 1);
            if !fallen_bytes.contains(&option)
                && path_elem.score + 1 < *distances.get(&option).unwrap_or(&usize::MAX)
            {
                let mut new_elem = path_elem;
                new_elem.score += 1;
                new_elem.pos = option;

                distances.insert(option, new_elem.score);
                q.push(new_elem);
            }
        }
        if y < bounds.0 - 1 {
            let option = (x, y + 1);
            if !fallen_bytes.contains(&option)
                && path_elem.score + 1 < *distances.get(&option).unwrap_or(&usize::MAX)
            {
                let mut new_elem = path_elem;
                new_elem.score += 1;
                new_elem.pos = option;

                distances.insert(option, new_elem.score);
                q.push(new_elem);
            }
        }
    };

    res
}

fn print_map(
    fallen_bytes: &HashSet<(usize, usize)>,
    visited: &HashMap<(usize, usize), usize>,
    bounds: (usize, usize),
) {
    for i in 0..bounds.0 {
        println!(
            "{}",
            (0..bounds.1)
                .map(|j| if fallen_bytes.contains(&(i, j)) {
                    "#"
                } else if visited.contains_key(&(i, j)) {
                    "O"
                } else {
                    "."
                })
                .join("")
        )
    }
    println!("");
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day18/input.txt") {
        // Parsing
        const BOUNDS: (usize, usize) = (71, 71);
        const START: (usize, usize) = (0, 0);
        const END: (usize, usize) = (BOUNDS.0 - 1, BOUNDS.1 - 1);
        const NUM_BYTES_FALLEN: usize = 1024;
        let line_vec = lines.flatten().into_iter().collect_vec();
        let mut fallen_bytes = line_vec[0..NUM_BYTES_FALLEN]
            .iter()
            .map(|line| {
                let mut it = line.split(",");
                let (x, y) = (
                    it.next()
                        .expect("no x coord")
                        .parse::<usize>()
                        .expect("x coord isnt uint"),
                    it.next()
                        .expect("no y coord")
                        .parse::<usize>()
                        .expect("y coord isnt uint"),
                );
                assert!(it.next().is_none());
                (x, y)
            })
            .collect::<HashSet<_>>();

        print_map(&fallen_bytes, &HashMap::new(), BOUNDS);

        let score = find_shortest_path(START, END, BOUNDS, &fallen_bytes).unwrap();

        println!("Score: {}", score);

        for line in line_vec[NUM_BYTES_FALLEN..].iter() {
            println!("{}", line);
            let mut it = line.split(",");
            let (x, y) = (
                it.next()
                    .expect("no x coord")
                    .parse::<usize>()
                    .expect("x coord isnt uint"),
                it.next()
                    .expect("no y coord")
                    .parse::<usize>()
                    .expect("y coord isnt uint"),
            );
            assert!(it.next().is_none());
            fallen_bytes.insert((x, y));

            if find_shortest_path(START, END, BOUNDS, &fallen_bytes).is_none() {
                println!("First block {:?}", (x, y));
                break;
            }
        }
    }
}
