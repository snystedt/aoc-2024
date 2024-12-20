use std::collections::{BinaryHeap, HashMap, HashSet};

use aoc_2024::{
    input::read_lines,
    utils::{Direction, Grid, DIRECTIONS},
};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
enum Entity {
    #[default]
    None,
    Wall,
    Start,
    End,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseTileError {
    #[error("{0} is out of range for Entity")]
    OutOfRange(u8),
}

impl TryFrom<&u8> for Entity {
    type Error = ParseTileError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            &b'.' => Ok(Entity::None),
            &b'#' => Ok(Entity::Wall),
            &b'S' => Ok(Entity::Start),
            &b'E' => Ok(Entity::End),
            _ => Err(ParseTileError::OutOfRange(*value)),
        }
    }
}

fn parse_map(lines: &[String]) -> Result<Grid<Entity>, ParseTileError> {
    assert!(!lines.is_empty());
    assert!(!lines[0].is_empty());

    let (rows, cols) = (lines.len(), lines[0].len());

    let mut world = Grid::<Entity>::zeros(rows, cols);

    lines.iter().enumerate().for_each(|(i, line)| {
        line.as_bytes().iter().enumerate().for_each(|(j, c)| {
            world.set((i, j), c.try_into().unwrap());
        })
    });

    Ok(world)
}

fn print_world(world: &Grid<Entity>) {
    world.rows().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|entity| match entity {
                    Entity::None => ".",
                    Entity::Wall => "#",
                    Entity::Start => "S",
                    Entity::End => "E",
                })
                .join("")
        )
    });
}

#[allow(dead_code)]
fn print_world_and_tiles(world: &Grid<Entity>, tiles: &HashSet<(usize, usize)>) {
    world.rows().enumerate().for_each(|(i, row)| {
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(j, entity)| if tiles.contains(&(i, j)) {
                    "O"
                } else {
                    match entity {
                        Entity::None => ".",
                        Entity::Wall => "#",
                        Entity::Start => "S",
                        Entity::End => "E",
                    }
                })
                .join("")
        )
    });
}

fn print_distances(distances: &HashMap<((usize, usize), Direction), usize>, world: &Grid<Entity>) {
    world.rows().enumerate().for_each(|(i, row)| {
        println!("{}", "+------".repeat(row.len()) + "+");
        println!(
            "{}",
            row.iter()
                .enumerate()
                .map(|(j, entity)| {
                    if let Some(d) = DIRECTIONS
                        .iter()
                        .filter_map(|d| distances.get(&((i, j), *d)))
                        .min()
                    {
                        format!("|{:^6}", d)
                    } else if entity == &Entity::Wall {
                        format!("|######")
                    } else {
                        format!("|      ")
                    }
                })
                .join("")
                + "|"
        );
    });
    println!("{}", "+------".repeat(world.row(0).len()) + "+");
}

#[derive(PartialEq, Eq, Clone)]
struct PathElement {
    pub score: usize,
    pub path: Vec<((usize, usize), Direction)>,
    pub goal: (usize, usize),
}

impl PathElement {
    fn distance_to_goal(&self) -> usize {
        let pos = self.path.last().unwrap().0;

        pos.0.abs_diff(self.goal.0) + pos.1.abs_diff(self.goal.1)
    }
}

impl Ord for PathElement {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .score
            .cmp(&self.score)
            .then_with(|| other.distance_to_goal().cmp(&self.distance_to_goal()))
            .then_with(|| self.path.last().unwrap().cmp(&other.path.last().unwrap()))
    }
}

impl PartialOrd for PathElement {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn find_options(world: &Grid<Entity>, pos: (usize, usize)) -> Vec<((usize, usize), Direction)> {
    DIRECTIONS
        .iter()
        .filter_map(|dir| {
            if let Some(new_pos) = dir.to_coord_vec().transform_coord(pos, world.size()) {
                match world.get(new_pos) {
                    Entity::None => Some((new_pos, *dir)),
                    Entity::Wall => None,
                    Entity::Start => None,
                    Entity::End => Some((new_pos, *dir)),
                }
            } else {
                None
            }
        })
        .collect()
}

fn find_shortest_path(world: &Grid<Entity>) -> (usize, usize) {
    let mut q: BinaryHeap<PathElement> = BinaryHeap::new();

    let start = world
        .indexed_iter()
        .find(|(_, e)| e == &&Entity::Start)
        .unwrap()
        .0;

    let end = world
        .indexed_iter()
        .find(|(_, e)| e == &&Entity::Start)
        .unwrap()
        .0;

    let mut distances: HashMap<((usize, usize), Direction), usize> = HashMap::default();

    q.push(PathElement {
        score: 0,
        path: vec![(start, Direction::Right)],
        goal: end,
    });
    distances.insert((start, Direction::Right), 0);

    let mut shortest_paths: Vec<PathElement> = vec![];

    loop {
        let path_elem = q.pop().unwrap();

        if !shortest_paths.is_empty()
            && path_elem.score > shortest_paths.first().as_ref().unwrap().score
        {
            dbg!(shortest_paths.first().as_ref().unwrap().score);
            dbg!(path_elem.score);
            dbg!(q.iter().map(|e| e.score).collect_vec());
            break;
        }

        let (curr_pos, curr_dir) = **path_elem.path.last().as_ref().unwrap();

        if world.get(curr_pos) == &Entity::End {
            shortest_paths.push(path_elem);
            continue;
        }

        for (pos, dir) in find_options(world, curr_pos).into_iter() {
            if dir == curr_dir {
                if path_elem.score + 1 > *distances.get(&(pos, dir)).unwrap_or(&usize::MAX) {
                    continue;
                }

                let mut new_elem = path_elem.clone();
                new_elem.path.push((pos, dir));
                new_elem.score += 1;

                distances.insert((pos, dir), new_elem.score);
                q.push(new_elem);
            } else {
                if path_elem.score + 1001 > *distances.get(&(pos, dir)).unwrap_or(&usize::MAX) {
                    continue;
                }

                let mut new_elem = path_elem.clone();
                new_elem.path.push((curr_pos, dir));
                new_elem.path.push((pos, dir));
                new_elem.score += 1001;

                distances.insert((pos, dir), new_elem.score);
                q.push(new_elem);
            };
        }
    }

    print_distances(&distances, &world);

    let mut tiles = HashSet::new();

    shortest_paths
        .iter()
        .for_each(|path| tiles.extend(path.path.iter().map(|(pos, _)| *pos)));

    //print_world_and_tiles(world, &tiles);

    (shortest_paths.first().unwrap().score, tiles.len())
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day16/input.txt") {
        // Parsing
        let input = lines.flatten().into_iter().collect_vec();
        let world = parse_map(&input).expect("Failed to parse input");
        print_world(&world);

        let score = find_shortest_path(&world);

        println!("Score: {}", score.0);
        println!("Tiles: {}", score.1);
    }
}
