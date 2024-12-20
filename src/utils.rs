use std::{
    io::{stdin, stdout, Read, Write},
    ops::Mul,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd, Default)]
pub enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_vec2(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        }
    }

    pub fn to_coord_vec(&self) -> CoordVec {
        CoordVec(match self {
            Direction::Up => (-1, 0),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Right => (0, 1),
        })
    }
}

pub const DIRECTIONS: [Direction; 4] = [
    Direction::Up,
    Direction::Down,
    Direction::Left,
    Direction::Right,
];

pub type Coord = (usize, usize);
pub struct CoordVec(pub (i32, i32));

impl Mul<i32> for &CoordVec {
    type Output = CoordVec;

    fn mul(self, rhs: i32) -> Self::Output {
        CoordVec((self.0 .0 * rhs, self.0 .1 * rhs))
    }
}

impl CoordVec {
    pub fn transform_coord(&self, coord: Coord, bounds: (usize, usize)) -> Option<Coord> {
        let new_row = coord.0 as i32 + self.0 .0;
        let new_col = coord.1 as i32 + self.0 .1;

        if !(0..bounds.0 as i32).contains(&new_row) || !(0..bounds.1 as i32).contains(&new_col) {
            None
        } else {
            Some((new_row as usize, new_col as usize))
        }
    }
}

pub struct Grid<T: Copy + Default> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}

impl<T: Copy + Default> Grid<T> {
    pub fn zeros(rows: usize, cols: usize) -> Self {
        Grid {
            data: vec![T::default(); rows * cols],
            rows,
            cols,
        }
    }

    pub fn size(&self) -> (usize, usize) {
        (self.rows, self.cols)
    }

    fn idx(&self, coord: Coord) -> usize {
        coord.0 * self.cols + coord.1
    }

    pub fn set(&mut self, coord: Coord, value: T) {
        let idx = self.idx(coord);
        self.data[idx] = value;
    }

    pub fn get(&self, coord: Coord) -> &T {
        &self.data[self.idx(coord)]
    }

    pub fn get_mut<'a>(&'a mut self, coord: Coord) -> &'a mut T {
        let idx = self.idx(coord);
        &mut self.data[idx]
    }

    pub fn row(&self, row: usize) -> &[T] {
        &self.data[row * self.cols..(row + 1) * self.cols]
    }

    pub fn rows<'a>(&'a self) -> GridRowIterator<'a, T> {
        GridRowIterator { grid: self, row: 0 }
    }

    pub fn iter<'a>(&'a self) -> impl Iterator<Item = &'a T> {
        self.data.iter()
    }

    pub fn iter_mut<'a>(&'a mut self) -> impl Iterator<Item = &'a mut T> {
        self.data.iter_mut()
    }

    pub fn indexed_iter<'a>(&'a self) -> IndexedGridIterator<'a, T> {
        IndexedGridIterator {
            grid: self,
            index: (0, 0),
        }
    }
}

pub struct IndexedGridIterator<'a, T: Copy + Default> {
    grid: &'a Grid<T>,
    index: Coord,
}

impl<'a, T: Copy + Default> Iterator for IndexedGridIterator<'a, T> {
    type Item = (Coord, &'a T);

    fn next(&mut self) -> Option<Self::Item> {
        if (0..self.grid.rows).contains(&self.index.0)
            && (0..self.grid.cols).contains(&self.index.1)
        {
            let ret = Some((self.index, self.grid.get(self.index)));
            if self.index.1 + 1 == self.grid.cols {
                self.index = (self.index.0 + 1, 0);
            } else {
                self.index = (self.index.0, self.index.1 + 1);
            }

            ret
        } else {
            None
        }
    }
}

pub struct GridRowIterator<'a, T: Copy + Default> {
    grid: &'a Grid<T>,
    row: usize,
}

impl<'a, T: Copy + Default> Iterator for GridRowIterator<'a, T> {
    type Item = &'a [T];

    fn next(&mut self) -> Option<Self::Item> {
        if self.row < self.grid.size().0 {
            let ret = Some(self.grid.row(self.row));

            self.row += 1;

            ret
        } else {
            None
        }
    }
}

pub fn pause() {
    let mut stdout = stdout();
    stdout.write(b"Press Enter to continue...").unwrap();
    stdout.flush().unwrap();
    stdin().read(&mut [0, 0]).unwrap();
}
