use std::collections::VecDeque;

use aoc_2024::input::read_lines;
use itertools::Itertools;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Operator {
    Add,
    Mult,
    Concat,
}

fn concat_i64(left: i64, right: i64) -> i64 {
    left * 10i64.pow(right.ilog10() + 1) + right
}

fn find_operators_with_op_sequences(result: i64, terms: &[i64]) -> Vec<Vec<Operator>> {
    let mut sequences: Vec<Vec<Operator>> = vec![];
    let num_terms = terms.len();

    let mut options: VecDeque<(i64, Vec<Operator>)> = VecDeque::new();
    options.push_back((terms[0], vec![]));
    while let Some((res, mut ops)) = options.pop_front() {
        let next_term = terms[ops.len() + 1];

        if res * next_term <= result {
            let mut ops = ops.clone();
            ops.push(Operator::Mult);
            if ops.len() == num_terms - 1 {
                if res * next_term == result {
                    sequences.push(ops);
                }
            } else {
                options.push_back((res * next_term, ops));
            }
        }

        if res + next_term <= result {
            let mut ops = ops.clone();
            ops.push(Operator::Add);
            if ops.len() == num_terms - 1 {
                if res + next_term == result {
                    sequences.push(ops);
                }
            } else {
                options.push_back((res + next_term, ops));
            }
        }

        {
            let cat = concat_i64(res, next_term);
            if cat <= result {
                ops.push(Operator::Concat);
                if ops.len() == num_terms - 1 {
                    if cat == result {
                        sequences.push(ops);
                    }
                } else {
                    options.push_back((cat, ops));
                }
            }
        }
    }

    sequences
}

fn find_operators(result: i64, terms: &[i64]) -> bool {
    let num_terms = terms.len();

    let mut options: VecDeque<(i64, usize)> = VecDeque::new();
    options.push_back((terms[0], 0));
    while let Some((res, num_ops)) = options.pop_front() {
        let next_term = terms[num_ops + 1];

        if res * next_term <= result {
            if num_ops + 1 == num_terms - 1 {
                if res * next_term == result {
                    return true;
                }
            } else {
                options.push_front((res * next_term, num_ops + 1));
            }
        }

        if res + next_term <= result {
            if num_ops + 1 == num_terms - 1 {
                if res + next_term == result {
                    return true;
                }
            } else {
                options.push_back((res + next_term, num_ops + 1));
            }
        }

        {
            let cat = concat_i64(res, next_term);
            if cat <= result {
                if num_ops + 1 == num_terms - 1 {
                    if cat == result {
                        return true;
                    }
                } else {
                    options.push_front((cat, num_ops + 1));
                }
            }
        }
    }

    false
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day7/input.txt") {
        let input = lines
            .flatten()
            .map(|line| {
                let mut split = line.split(": ");
                let result = split
                    .next()
                    .expect("malformed input")
                    .parse::<i64>()
                    .expect("failed to parse result to i64");
                let terms = split
                    .next()
                    .expect("malformed input")
                    .split(" ")
                    .map(|s| s.parse::<i64>().expect("failed to parse term to i64"))
                    .collect_vec();
                (result, terms)
            })
            .collect_vec();

        let total = input
            .iter()
            .filter_map(|(result, terms)| find_operators(*result, terms).then_some(*result))
            .sum::<i64>();

        dbg!(total);
    }
}
