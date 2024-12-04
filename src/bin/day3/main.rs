use aoc_2024::input::read_lines;
use itertools::Itertools;
use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    if let Ok(lines) = read_lines("./inputs/day3/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let input = lines.flatten().join("");

        let res = re
            .captures_iter(&input)
            .map(|m| {
                (
                    u32::from_str_radix(m.get(1).unwrap().as_str(), 10).unwrap(),
                    u32::from_str_radix(m.get(2).unwrap().as_str(), 10).unwrap(),
                )
            })
            .map(|(a, b)| a * b)
            .sum::<u32>();

        dbg!(res);

        let res = input
            .split("do()")
            .map(|s| {
                re.captures_iter(s.split("don't()").take(1).next().unwrap())
                    .map(|m| {
                        (
                            u32::from_str_radix(m.get(1).unwrap().as_str(), 10).unwrap(),
                            u32::from_str_radix(m.get(2).unwrap().as_str(), 10).unwrap(),
                        )
                    })
                    .map(|(a, b)| a * b)
                    .collect::<Vec<_>>()
            })
            .flatten()
            .sum::<u32>();

        dbg!(res);
    }
}
