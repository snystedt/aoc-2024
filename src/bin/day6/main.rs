use std::{
    collections::{HashMap, HashSet},
    fmt,
};

use aoc_2024::input::read_lines;
use itertools::Itertools;

struct Grid<T: Copy + Default> {
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

    pub fn set(&mut self, row: usize, col: usize, value: T) {
        let idx = self.idx(row, col);
        self.data[idx] = value;
    }

    pub fn get(&self, row: usize, col: usize) -> &T {
        &self.data[self.idx(row, col)]
    }

    pub fn row(&self, row: usize) -> &[T] {
        &self.data[row * self.cols..(row + 1) * self.cols]
    }

    fn idx(&self, row: usize, col: usize) -> usize {
        row * self.cols + col
    }
}

#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
enum Direction {
    #[default]
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn rotate_ccw(&self) -> Self {
        match *self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    pub fn move_point(
        &self,
        point: (usize, usize),
        bounds: (usize, usize),
    ) -> Option<(usize, usize)> {
        match self {
            Direction::Up => {
                if point.0 == 0 {
                    None
                } else {
                    Some((point.0 - 1, point.1))
                }
            }
            Direction::Down => {
                if point.0 == bounds.0 - 1 {
                    None
                } else {
                    Some((point.0 + 1, point.1))
                }
            }
            Direction::Left => {
                if point.1 == 0 {
                    None
                } else {
                    Some((point.0, point.1 - 1))
                }
            }
            Direction::Right => {
                if point.1 == bounds.1 - 1 {
                    None
                } else {
                    Some((point.0, point.1 + 1))
                }
            }
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Entity {
    Empty { visited: bool },
    Object,
    Guard(Direction),
}

impl Default for Entity {
    fn default() -> Self {
        Entity::Empty { visited: false }
    }
}

impl Entity {
    pub fn from_ascii(c: u8) -> Self {
        match c {
            b'.' => Self::Empty { visited: false },
            b'X' => Self::Empty { visited: true },
            b'#' => Self::Object,
            b'^' => Self::Guard(Direction::Up),
            b'v' => Self::Guard(Direction::Down),
            b'<' => Self::Guard(Direction::Left),
            b'>' => Self::Guard(Direction::Right),
            _ => unreachable!(),
        }
    }

    pub fn to_ascii(&self) -> u8 {
        match self {
            Entity::Empty { visited } => match visited {
                true => b'X',
                false => b'.',
            },
            Entity::Object => b'#',
            Entity::Guard(direction) => match direction {
                Direction::Up => b'^',
                Direction::Down => b'v',
                Direction::Left => b'<',
                Direction::Right => b'>',
            },
        }
    }
}

struct World {
    pub grid: Grid<Entity>,
    pub guard_position: Option<(usize, usize)>,
    pub guard_direction: Direction,
}

impl World {
    pub fn new(grid: Grid<Entity>) -> Self {
        let mut guard_position = None;
        let mut guard_direction = Direction::Up;
        (0..grid.rows)
            .cartesian_product(0..grid.cols)
            .for_each(|(row, col)| match grid.get(row, col) {
                Entity::Guard(dir) => {
                    guard_position = Some((row, col));
                    guard_direction = *dir;
                }
                _ => (),
            });

        assert!(guard_position.is_some());
        Self {
            grid,
            guard_position,
            guard_direction,
        }
    }

    pub fn walk(&self) -> Option<Vec<((usize, usize), Direction)>> {
        let mut path = vec![];
        path.reserve(self.grid.rows * self.grid.cols);
        let mut path_len = path.len();

        let mut visited_spaces = HashSet::with_capacity(self.grid.rows * self.grid.cols);

        if let Some(guard_position) = self.guard_position {
            let mut pos = guard_position;
            let mut dir = self.guard_direction;
            while let Some((next_pos, new_dir)) = self.find_next_turnpoint(&mut path, pos, dir) {
                for (p, d) in &path[path_len..] {
                    if !visited_spaces.insert((*p, *d)) {
                        return None;
                    }
                }
                path_len = path.len();

                pos = next_pos;
                dir = new_dir;
            }

            for (p, d) in &path[path_len..] {
                visited_spaces.insert((*p, *d));
            }
        }

        Some(path)
    }

    fn find_next_turnpoint(
        &self,
        path: &mut Vec<((usize, usize), Direction)>,
        pos: (usize, usize),
        dir: Direction,
    ) -> Option<((usize, usize), Direction)> {
        let mut pos = pos;
        loop {
            path.push((pos, dir));
            let new_pos = dir.move_point(pos, (self.grid.rows, self.grid.cols));
            if let Some(new_pos) = new_pos {
                if self.grid.get(new_pos.0, new_pos.1) == &Entity::Object {
                    return Some((pos, dir.rotate_ccw()));
                } else {
                    pos = new_pos;
                }
            } else {
                return None;
            }
        }
    }
}

// Implement `Display` for `MinMax`.
impl fmt::Display for World {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let out = (0..self.grid.rows)
            .map(|row| {
                self.grid
                    .row(row)
                    .iter()
                    .map(|entity| Into::<char>::into(entity.to_ascii()))
                    .collect::<String>()
            })
            .join("\n");
        write!(f, "{}", out)
    }
}

fn parse_input(input: Vec<String>) -> World {
    let (rows, cols) = (input.len(), input[0].len());

    let mut grid = Grid::<Entity>::zeros(rows, cols);

    input.iter().enumerate().for_each(|(i, row)| {
        row.as_bytes()
            .iter()
            .enumerate()
            .for_each(|(j, c)| grid.set(i, j, Entity::from_ascii(*c)))
    });

    World::new(grid)
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day6/input.txt") {
        // Parsing
        let input = lines.flatten().collect::<Vec<_>>();
        assert!(!input.is_empty());

        let mut world = parse_input(input);

        let path = if let Some(path) = world.walk() {
            let visited = path
                .iter()
                .map(|(pos, _)| *pos)
                .collect::<HashSet<_>>()
                .len();
            println!("Number of visited spaces: {}", visited);

            path
        } else {
            return;
        };

        let mut new_object_results = HashMap::with_capacity(path.len());
        for i in 0..path.len() - 1 {
            if path[i]
                .1
                .move_point(path[i].0, (world.grid.rows, world.grid.cols))
                .is_some_and(|p| p == path[i + 1].0)
            {
                let (pos, _) = path[i + 1];

                if new_object_results.get(&pos).is_some_and(|res| *res) {
                    continue;
                }

                world.guard_position = Some(path[i].0);
                world.guard_direction = path[i].1;
                world.grid.set(pos.0, pos.1, Entity::Object);

                if world.walk().is_none() {
                    new_object_results.insert(pos, true);
                } else {
                    new_object_results.insert(pos, false);
                }

                world
                    .grid
                    .set(pos.0, pos.1, Entity::Empty { visited: false });
            }
        }

        println!(
            "There are {} ways to create a guard loop.",
            new_object_results.values().filter(|v| **v).count()
        );
    }
}
