use aoc_2024::input::read_lines;
use regex::Regex;

fn main() {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    if let Ok(lines) = read_lines("./inputs/day3/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let res = lines
            .flatten()
            .map(|line| {
                re.captures_iter(&line)
                    .map(|m| {
                        (
                            u32::from_str_radix(m.get(1).unwrap().as_str(), 10).unwrap(),
                            u32::from_str_radix(m.get(2).unwrap().as_str(), 10).unwrap(),
                        )
                    })
                    .map(|(a, b)| a * b)
                    .collect::<Vec<u32>>()
            })
            .flatten()
            .sum::<u32>();

        dbg!(res);
    }
}
