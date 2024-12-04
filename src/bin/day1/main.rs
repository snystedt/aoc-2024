use aoc_2024::input::read_lines;

fn main() {
    // File hosts.txt must exist in the current path
    if let Ok(lines) = read_lines("./inputs/day1/input.txt") {
        // Consumes the iterator, returns an (Optional) String
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

        //dbg!(&left[..10]);
        //dbg!(&right[..10]);

        dbg!(left[..]
            .iter()
            .zip(right[..].iter())
            .map(|(&l, &r)| l.abs_diff(r))
            .sum::<u32>());
    }
}
