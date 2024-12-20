use aoc_2024::input::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::sync::LazyLock;

#[derive(Debug)]
struct ClawMachine {
    pub btn_a: (i64, i64),
    pub btn_b: (i64, i64),
    pub prize: (i64, i64),
}

fn parse_button(line: &str) -> Option<(i64, i64)> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^Button [A|B]: X\+(?<X>\d+), Y\+(?<Y>\d+)$").unwrap());

    RE.captures(line)
        .map(|caps| caps.extract())
        .and_then(|(_, [x, y])| match (x.parse::<i64>(), y.parse::<i64>()) {
            (Ok(x), Ok(y)) => Some((x, y)),
            _ => None,
        })
}

fn parse_prize(line: &str) -> Option<(i64, i64)> {
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^Prize: X=(?<X>\d+), Y=(?<Y>\d+)$").unwrap());

    RE.captures(line)
        .map(|caps| caps.extract())
        .and_then(|(_, [x, y])| match (x.parse::<i64>(), y.parse::<i64>()) {
            (Ok(x), Ok(y)) => Some((x, y)),
            _ => None,
        })
}

fn solve_system(a: (i64, i64), b: (i64, i64), c: (i64, i64)) -> Option<(i64, i64)> {
    if (a.1 * c.0 - a.0 * c.1).rem_euclid(a.1 * b.0 - a.0 * b.1) != 0 {
        return None;
    }
    let v = (a.1 * c.0 - a.0 * c.1) / (a.1 * b.0 - a.0 * b.1);

    if (c.1 - b.1 * v).rem_euclid(a.1) != 0 {
        return None;
    }

    let u = (c.1 - b.1 * v) / a.1;

    Some((u, v))
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day13/input.txt") {
        // Parsing
        let input = lines
            .flatten()
            .filter(|s| !s.is_empty())
            .chunks(3)
            .into_iter()
            .map(|mut chunk| ClawMachine {
                btn_a: chunk
                    .next()
                    .and_then(|line| parse_button(&line))
                    .expect("Failed parsing Button A"),
                btn_b: chunk
                    .next()
                    .and_then(|line| parse_button(&line))
                    .expect("Failed parsing Button B"),
                prize: chunk
                    .next()
                    .and_then(|line| parse_prize(&line))
                    .expect("Failed parsing Prize"),
            })
            .collect_vec();

        let star1 = input
            .iter()
            .map(|cm| solve_system(cm.btn_a, cm.btn_b, cm.prize))
            .flatten()
            .inspect(|v| {
                dbg!(v);
            })
            .map(|v| v.0 * 3 + v.1)
            .sum::<i64>();

        println!("Star 1 cost: {}", star1);

        let star2 = input
            .iter()
            .map(|cm| {
                solve_system(
                    cm.btn_a,
                    cm.btn_b,
                    (cm.prize.0 + 10000000000000, cm.prize.1 + 10000000000000),
                )
            })
            .flatten()
            .inspect(|v| {
                dbg!(v);
            })
            .map(|v| v.0 * 3 + v.1)
            .sum::<i64>();

        println!("Star 2 cost: {}", star2);
    }
}
