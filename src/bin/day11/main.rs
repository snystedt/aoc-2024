use std::collections::HashMap;

use aoc_2024::input::read_lines;
use itertools::Either;

fn split_number_if_even(number: usize) -> Option<(usize, usize)> {
    if number == 0 {
        return None;
    }
    let n_log10 = number.ilog10() + 1;
    if n_log10.rem_euclid(2) != 0 {
        None
    } else {
        let divider = 10usize.pow(n_log10 / 2);

        let left = number / divider;
        let right = number - left * divider;

        Some((left, right))
    }
}

fn blink(stone: usize) -> Either<usize, (usize, usize)> {
    if let Some((left, right)) = split_number_if_even(stone) {
        Either::Right((left, right))
    } else if stone == 0 {
        Either::Left(1)
    } else {
        Either::Left(stone * 2024)
    }
}

fn insert_or_increment(map: &mut HashMap<usize, usize>, key: &usize, inc: usize) {
    if let Some(cnt) = map.get_mut(key) {
        *cnt += inc;
    } else {
        map.insert(*key, inc);
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day11/input.txt") {
        // Parsing
        let input = lines.flatten().collect::<Vec<_>>();
        assert!(input.len() == 1);

        let mut cache: HashMap<usize, Either<usize, (usize, usize)>> = HashMap::new();

        let mut stones = input[0]
            .split(" ")
            .map(|d| (d.parse::<usize>().unwrap(), 1usize))
            .collect::<HashMap<usize, usize>>();

        let mut new_stones: HashMap<usize, usize> = HashMap::new();

        for _ in 0..100 {
            for (stone, num) in stones.iter() {
                let res = if let Some(res) = cache.get(stone) {
                    res
                } else {
                    let res = blink(*stone);
                    cache.insert(*stone, res);
                    cache.get(&stone).unwrap()
                };
                match res {
                    Either::Left(s) => {
                        insert_or_increment(&mut new_stones, s, *num);
                    }
                    Either::Right((l, r)) => {
                        insert_or_increment(&mut new_stones, l, *num);
                        insert_or_increment(&mut new_stones, r, *num);
                    }
                }
            }

            stones.iter_mut().for_each(|(_, cnt)| *cnt = 0);
            new_stones.iter_mut().for_each(|(stone, cnt)| {
                insert_or_increment(&mut stones, stone, *cnt);
                *cnt = 0;
            });
            println!("Number of unique stones seen: {}", stones.len());
        }

        let mut keys = stones.keys().cloned().collect::<Vec<_>>();
        keys.sort();
        println!("Stones: {:?}", keys);

        println!(
            "Number of stones: {}",
            stones.iter().map(|(_, cnt)| *cnt).sum::<usize>()
        );
    }
}
