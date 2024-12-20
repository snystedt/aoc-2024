use aoc_2024::{input::read_lines, utils::Grid};
use itertools::Itertools;

fn match_towels_in_pattern<'a>(
    towels: &[&'a str],
    pattern: &str,
) -> Vec<(&'a str, (usize, usize))> {
    let mut towel_matches = vec![];

    for towel in towels {
        let mut idx = 0;
        // Each time a towel is found within the pattern, store (start, end)
        // IMPORTANT: only increment 1 step since a towel match can overlap itself e.g.
        // for a pattern 'rwrwr' towel 'rwr' matches (0,3) and (2, 5)
        while let Some(first) = pattern[idx..].find(towel) {
            towel_matches.push((*towel, (idx + first, idx + first + towel.len())));
            idx += first + 1;
            if idx >= pattern.len() {
                break;
            }
        }
    }

    towel_matches
}
fn count_all_towel_arrangements_matrix(towels: &[&str], pattern: &str) -> usize {
    // Find all substring matches of towels in the pattern
    let towel_matches = match_towels_in_pattern(towels, pattern);

    // Construct adjacency matrix, a 1 means there is a transition from posititon i to position j - (i, j) := (row_number, col_number)
    let mut adj_matrix: Grid<u8> = Grid::zeros(pattern.len() + 1, pattern.len() + 1);
    towel_matches
        .iter()
        .for_each(|(_, (i, j))| adj_matrix.set((*i, *j), 1));

    // Calculate the different ways of arriving at pattern positions
    let mut paths_to_rows = vec![0usize; pattern.len() + 1];
    // There is exactly one way to arrive at the 0th position
    paths_to_rows[0] = 1;

    for i in 1..pattern.len() + 1 {
        let mut paths_to_row = 0;
        // The number of ways to arrive at this position is equal to the sum of the number of ways to arrive at any position leading to this one
        for col in 0..i {
            paths_to_row += (*adj_matrix.get((col, i)) as usize) * paths_to_rows[col];
        }

        paths_to_rows[i] = paths_to_row;
    }

    paths_to_rows[pattern.len()]
}

fn is_pattern_possible<'a>(towels: &[&'a str], pattern: &str) -> bool {
    // A pattern is possible if there are more than zero ways to arrive at the end position
    count_all_towel_arrangements_matrix(towels, pattern) > 0
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let lines = read_lines("./inputs/day19/input.txt")?
        .flatten()
        .into_iter()
        .collect_vec();

    assert!(lines.len() > 3);
    let towel_patterns = lines[0].split(", ").map(|s| s).collect_vec();

    assert!(lines[1].is_empty());
    let patterns = lines[2..].iter().map(|s| s.as_str()).collect_vec();

    let possible_patterns = patterns
        .iter()
        .filter(|p| is_pattern_possible(&towel_patterns, p))
        .collect_vec();

    println!("Possible patterns: {}", possible_patterns.len());

    let cnt = patterns
        .iter()
        .map(|p| count_all_towel_arrangements_matrix(&towel_patterns, p))
        .sum::<usize>();

    println!("Possible arrangements: {}", cnt);

    Ok(())
}
