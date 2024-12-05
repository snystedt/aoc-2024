use std::{cmp::Ordering, collections::BTreeMap};

use aoc_2024::input::read_lines;
fn main() {
    if let Ok(lines) = read_lines("./inputs/day5/input.txt") {
        // Parsing
        let mut map = BTreeMap::<u32, Vec<u32>>::new();
        let mut seq: Vec<Vec<u32>> = vec![];
        lines.flatten().for_each(|line| {
            if line.is_empty() {
                return;
            } else if line.contains("|") {
                let mut it = line.split("|");
                let first = it.next().unwrap().parse::<u32>().unwrap();
                let second = it.next().unwrap().parse::<u32>().unwrap();
                if let Some(e) = map.get_mut(&first) {
                    e.push(second);
                } else {
                    map.insert(first, vec![second]);
                }
            } else {
                seq.push(
                    line.split(",")
                        .map(|s| s.parse::<u32>().unwrap())
                        .collect::<Vec<_>>(),
                );
            }
        });

        // First star
        let (valid, invalid): (Vec<Vec<u32>>, Vec<Vec<u32>>) =
            seq.iter().cloned().partition(|seq| {
                seq.is_sorted_by(|a, b| !map.get(b).is_some_and(|rule| rule.contains(a)))
            });
        let res = valid.iter().map(|seq| seq[seq.len() / 2]).sum::<u32>();

        println!("First star: {}", res);

        // Second star
        let res = invalid
            .into_iter()
            .map(|mut seq| {
                seq.sort_by(|a, b| {
                    map.get(b)
                        .is_some_and(|rule| rule.contains(a))
                        .then_some(Ordering::Less)
                        .unwrap_or(Ordering::Equal)
                });
                seq
            })
            .map(|seq| seq[seq.len() / 2])
            .sum::<u32>();

        println!("Second star: {}", res);
    }
}
