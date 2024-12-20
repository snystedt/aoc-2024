use aoc_2024::input::read_lines;
use itertools::Itertools;
use regex::Regex;
use std::{
    collections::HashSet,
    io::{stdin, stdout, Read, Write},
    sync::LazyLock,
};

fn parse_pos_vel(line: &str) -> Option<((i64, i64), (i64, i64))> {
    dbg!(line);
    static RE: LazyLock<Regex> =
        LazyLock::new(|| Regex::new(r"^p=(\-?\d+),(\-?\d+) v=(\-?\d+),(\-?\d+)$").unwrap());

    RE.captures(line)
        .map(|caps| caps.extract())
        .and_then(|(_, [p_x, p_y, v_x, v_y])| {
            match (
                p_x.parse::<i64>(),
                p_y.parse::<i64>(),
                v_x.parse::<i64>(),
                v_y.parse::<i64>(),
            ) {
                (Ok(p_x), Ok(p_y), Ok(v_x), Ok(v_y)) => Some(((p_x, p_y), (v_x, v_y))),
                _ => None,
            }
        })
}

fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0, 0]).unwrap();
}

const WIDTH: i64 = 101;
const HEIGHT: i64 = 103;

fn show_config(config: &Vec<((i64, i64), (i64, i64))>) {
    let occupied = config.iter().map(|(p, _)| *p).collect::<HashSet<_>>();
    (0..HEIGHT).for_each(|i| {
        println!(
            "{}",
            (0..WIDTH)
                .map(|j| occupied.contains(&(i, j)).then_some("#").unwrap_or("."))
                .join("")
        )
    });
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day14/input.txt") {
        // Parsing
        let mut input = lines
            .flatten()
            .into_iter()
            .map(|line| parse_pos_vel(&line).expect("Failed to parse line"))
            .collect_vec();

        /*
        let star1 = input
            .iter()
            .map(|(p, v)| {
                (
                    (p.0 + v.0 * 100).rem_euclid(WIDTH as i64),
                    (p.1 + v.1 * 100).rem_euclid(HEIGHT as i64),
                )
            })
            .collect_vec();

        let indexes = [
            (0..WIDTH / 2, 0..HEIGHT / 2),
            (WIDTH / 2 + 1..WIDTH, 0..HEIGHT / 2),
            (0..WIDTH / 2, HEIGHT / 2 + 1..HEIGHT),
            (WIDTH / 2 + 1..WIDTH, HEIGHT / 2 + 1..HEIGHT),
        ];

        let res = indexes
            .into_iter()
            .map(|r| {
                star1
                    .iter()
                    .filter(|&i| r.0.contains(&i.0) && r.1.contains(&i.1))
                    .count()
            })
            .product::<usize>();

        println!("Star 1: {}", &res);
        */

        let christmas_tree = (1..5)
            .map(|row| [(WIDTH / 2 + row, row), (WIDTH / 2 - row, row)].into_iter())
            .flatten()
            .chain([((WIDTH / 2, 0))])
            .collect_vec();

        // Second star solved by finding the frequency of vertical and horizontal lines appearing:
        //
        // First two horizontal lines at: t=27, t=128 => delta_t = 101
        // First two vertical lines at: t=75, t=178 => delta_t = 103
        //
        // Thus, horizontal lines appear at times t=27 + 101k, for integer k
        // and, vertical lines appear at times t=75 + 103l, for integer l
        //
        // Christmas tree image probably appears when these appear at the same time
        //
        // I.e. when 27 + 101k = 75 + 103l
        // or 101k - 103l = 48
        // Solve diophantine equation, extended euclid's give gcd(101, -103) = 1 and
        // 101 * 51 - 50 * 103 = 1
        //
        // Particular solution is thus
        // (k, l)_p = (2448, 2400)
        //
        // Solution to the homogeneous equation (101k - 103l = 0) is
        // (k, l)_h = (103, 101)
        //
        // Total solution is thus:
        // (k, l) = (2448, 2400) + (103, 101)m
        //
        // Smallest m where both x and y are positive integers gives m = -23
        //
        // (k, l) = (2448 - 103 * 23, 2400 - 101 * 23)
        //
        // So the 23rd peaks will coincide, using the horizontal ones we get
        // t_xmas = 27 + 101 * 23 = 8006

        let mut t = 0usize;
        while t < 1_000_000 {
            let occupied = input.iter().map(|(p, _)| *p).collect::<HashSet<_>>();
            if t == 8006 || christmas_tree.iter().all(|p| occupied.contains(p)) {
                show_config(&input);
                println!("T = {}", t);
                pause();
            }
            input.iter_mut().for_each(|pos_vel| {
                *pos_vel = (
                    (
                        (pos_vel.0 .0 + pos_vel.1 .0).rem_euclid(WIDTH as i64),
                        (pos_vel.0 .1 + pos_vel.1 .1).rem_euclid(HEIGHT as i64),
                    ),
                    pos_vel.1,
                )
            });
            t += 1;
        }
    }
}
