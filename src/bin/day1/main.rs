use std::collections::BTreeMap;

use aoc_2024::input::read_lines;

fn main() {
    if let Ok(lines) = read_lines("./inputs/day1/input.txt") {
        let (mut left, mut right): (Vec<u32>, Vec<u32>) = lines
            .flatten()
            .map(|line| {
                let mut it = line.split("   ");
                (
                    u32::from_str_radix(it.next().unwrap(), 10).unwrap(),
                    u32::from_str_radix(it.next().unwrap(), 10).unwrap(),
                )
            })
            .unzip();

        left.sort();
        right.sort();

        dbg!(left[..]
            .iter()
            .zip(right[..].iter())
            .map(|(&l, &r)| l.abs_diff(r))
            .sum::<u32>());

        let mut m = BTreeMap::<u32, usize>::new();
        right.iter().for_each(|v| {
            if let Some(e) = m.get_mut(v) {
                *e += 1;
            } else {
                m.insert(*v, 1);
            }
        });

        dbg!(left
            .iter()
            .map(|v| if let Some(e) = m.get(v) {
                *e as u32 * *v
            } else {
                0
            })
            .sum::<u32>());
    }
}
