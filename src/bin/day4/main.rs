use aoc_2024::input::read_lines;
use itertools::Itertools;

struct WordSearch {
    data: Vec<u8>,
    rows: usize,
    cols: usize,
}

impl WordSearch {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        WordSearch {
            data: vec![0; rows * cols],
            rows,
            cols,
        }
    }

    pub fn set(&mut self, row: usize, col: usize, value: u8) {
        let idx = self.idx(row, col);
        self.data[idx] = value;
    }

    pub fn get(&mut self, row: usize, col: usize) -> u8 {
        self.data[self.idx(row, col)]
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }

    pub fn horizontal<const N: usize>(&self, row: usize, col: usize) -> [u8; N] {
        assert!(row < self.rows);
        assert!(col < self.cols - N + 1);

        let mut ret = [0; N];
        (0..N).for_each(|offset| ret[offset] = self.data[self.idx(row, col + offset)]);

        ret
    }

    pub fn vertical<const N: usize>(&self, row: usize, col: usize) -> [u8; N] {
        assert!(col < self.cols);
        assert!(row < self.rows - N + 1);

        let mut ret = [0; N];
        (0..N).for_each(|offset| ret[offset] = self.data[self.idx(row + offset, col)]);

        ret
    }

    pub fn diagonal_right<const N: usize>(&self, row: usize, col: usize) -> [u8; N] {
        assert!(col < self.cols - N + 1);
        assert!(row < self.rows - N + 1);

        let mut ret = [0; N];
        (0..N).for_each(|offset| ret[offset] = self.data[self.idx(row + offset, col + offset)]);

        ret
    }

    pub fn diagonal_left<const N: usize>(&self, row: usize, col: usize) -> [u8; N] {
        assert!(col >= N - 1);
        assert!(row < self.rows - N + 1);

        let mut ret = [0; N];
        (0..N).for_each(|offset| ret[offset] = self.data[self.idx(row + offset, col - offset)]);

        ret
    }

    pub fn find_all_needles<const N: usize>(&self, needle: &str) -> usize {
        assert!(needle.len() == N);

        let cmp_needle = |word: &[u8; N]| {
            word.iter()
                .zip(needle.as_bytes().iter())
                .all(|(w, n)| w == n)
                || word
                    .iter()
                    .rev()
                    .zip(needle.as_bytes().iter())
                    .all(|(w, n)| w == n)
        };

        let h_cnt = (0..self.rows)
            .cartesian_product(0..self.cols - N + 1)
            .filter(|&(row, col)| cmp_needle(&self.horizontal::<N>(row, col)))
            .count();

        let v_cnt = (0..self.rows - N + 1)
            .cartesian_product(0..self.cols)
            .filter(|&(row, col)| cmp_needle(&self.vertical::<N>(row, col)))
            .count();

        let dr_cnt = (0..self.rows - N + 1)
            .cartesian_product(0..self.cols - N + 1)
            .filter(|&(row, col)| cmp_needle(&self.diagonal_right::<N>(row, col)))
            .count();

        let dl_cnt = (0..self.rows - N + 1)
            .cartesian_product(N - 1..self.cols)
            .filter(|&(row, col)| cmp_needle(&self.diagonal_left::<N>(row, col)))
            .count();

        h_cnt + v_cnt + dr_cnt + dl_cnt
    }

    pub fn find_all_sams(&self) -> usize {
        let needle = "MAS";

        let cmp_needle = |word: &[u8; 3]| {
            word.iter()
                .zip(needle.as_bytes().iter())
                .all(|(w, n)| w == n)
                || word
                    .iter()
                    .rev()
                    .zip(needle.as_bytes().iter())
                    .all(|(w, n)| w == n)
        };

        let dr_cnt = (0..self.rows - 2)
            .cartesian_product(0..self.cols - 2)
            .filter(|&(row, col)| {
                cmp_needle(&self.diagonal_right::<3>(row, col))
                    && cmp_needle(&self.diagonal_left::<3>(row, col + 2))
            })
            .count();

        dr_cnt
    }
}

fn main() {
    let mut ws = WordSearch::zeros(140, 140);
    if let Ok(lines) = read_lines("./inputs/day4/input.txt") {
        // Consumes the iterator, returns an (Optional) String
        lines.flatten().enumerate().for_each(|(row, line)| {
            assert!(line.is_ascii());
            assert!(line.bytes().count() == 140);
            line.bytes()
                .enumerate()
                .for_each(|(col, byte)| ws.set(row, col, byte));
        });

        dbg!(ws.find_all_needles::<4>("XMAS"));
        dbg!(ws.find_all_sams());
    }
}
