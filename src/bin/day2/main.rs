use aoc_2024::input::read_lines;

fn main() {
    let all_increasing = |report: &Vec<u32>| report.windows(2).all(|w| w[0] < w[1]);
    let all_decreasing = |report: &Vec<u32>| report.windows(2).all(|w| w[0] > w[1]);
    let gradual = |report: &Vec<u32>| {
        report
            .windows(2)
            .all(|w| w[0].abs_diff(w[1]) >= 1 && w[0].abs_diff(w[1]) <= 3)
    };

    if let Ok(lines) = read_lines("./inputs/day2/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        let reports: Vec<Vec<u32>> = lines
            .flatten()
            .map(|line| {
                line.split(" ")
                    .map(|s| u32::from_str_radix(s, 10).unwrap())
                    .collect::<Vec<_>>()
            })
            .collect();

        let ret = reports
            .iter()
            .filter(|&report| (all_increasing(report) || all_decreasing(report)) && gradual(report))
            .count();
        dbg!(&ret);

        let ret = reports
            .iter()
            .filter(|&report| {
                let mut tmp = vec![0; report.len() - 1];
                (0..report.len()).any(|idx| {
                    (0usize..report.len())
                        .filter(|i| *i != idx)
                        .enumerate()
                        .for_each(|(i, j)| tmp[i] = report[j]);
                    (all_increasing(&tmp) || all_decreasing(&tmp)) && gradual(&tmp)
                })
            })
            .count();
        dbg!(&ret);
    }
}
