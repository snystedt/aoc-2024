use std::collections::HashSet;

use aoc_2024::{
    input::read_lines,
    utils::{Direction, Grid},
};
use itertools::Itertools;

#[derive(Debug, Copy, Clone, Default, Eq, PartialEq, Hash)]
enum Entity {
    #[default]
    None,
    Box,
    Wall,
    Robot,
    WideBoxLeft,
    WideBoxRight,
}

#[derive(Debug, thiserror::Error)]
pub enum ParseEntityError {
    #[error("{0} is out of range for Entity")]
    OutOfRange(u8),
}

impl TryFrom<&u8> for Entity {
    type Error = ParseEntityError;

    fn try_from(value: &u8) -> Result<Self, Self::Error> {
        match value {
            &b'.' => Ok(Entity::None),
            &b'O' => Ok(Entity::Box),
            &b'#' => Ok(Entity::Wall),
            &b'@' => Ok(Entity::Robot),
            &b'[' => Ok(Entity::WideBoxLeft),
            &b']' => Ok(Entity::WideBoxRight),
            _ => Err(ParseEntityError::OutOfRange(*value)),
        }
    }
}

fn parse_map(lines: &[String]) -> Result<(Grid<Entity>, (usize, usize)), ParseEntityError> {
    assert!(!lines.is_empty());
    assert!(!lines[0].is_empty());

    let (rows, cols) = (lines.len(), lines[0].len());

    let mut world = Grid::<Entity>::zeros(rows, cols);
    let mut robot_pos = None;

    lines.iter().enumerate().for_each(|(i, line)| {
        line.as_bytes().iter().enumerate().for_each(|(j, c)| {
            if c == &b'@' {
                robot_pos = Some((i, j));
            }
            world.set((i, j), c.try_into().unwrap());
        })
    });

    assert!(robot_pos.is_some());

    Ok((world, robot_pos.unwrap()))
}

fn transform_map_to_wide(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|row| {
            row.as_bytes()
                .iter()
                .map(|c| match c {
                    &b'#' => "##",
                    &b'.' => "..",
                    &b'O' => "[]",
                    &b'@' => "@.",
                    _ => unreachable!(),
                })
                .join("")
        })
        .collect()
}

#[allow(dead_code)]
fn print_world(world: &Grid<Entity>) {
    world.rows().for_each(|row| {
        println!(
            "{}",
            row.iter()
                .map(|entity| match entity {
                    Entity::None => ".",
                    Entity::Box => "O",
                    Entity::Wall => "#",
                    Entity::Robot => "@",
                    Entity::WideBoxLeft => "[",
                    Entity::WideBoxRight => "]",
                })
                .join("")
        )
    });
}

fn simulate_robot(world: &mut Grid<Entity>, mut robot_pos: (usize, usize), moves: &String) {
    let move_to_dir = |mv: &u8| match mv {
        b'^' => Direction::Up,
        b'v' => Direction::Down,
        b'<' => Direction::Left,
        b'>' => Direction::Right,
        _ => unreachable!(),
    };

    for mv in moves.as_bytes() {
        assert!(world.get(robot_pos) == &Entity::Robot);

        // Find boxes or walls
        let dir = move_to_dir(mv);
        let dir_vec = dir.to_coord_vec();

        let mut curr = dir_vec
            .transform_coord(robot_pos, world.size())
            .expect("Robot is inside a wall");

        if world.get(curr) == &Entity::None {
            world.set(curr, Entity::Robot);
            world.set(robot_pos, Entity::None);
            robot_pos = curr;
        } else if world.get(curr) == &Entity::Box {
            loop {
                curr = dir_vec
                    .transform_coord(curr, world.size())
                    .expect("Moved out-of-bounds");

                if world.get(curr) == &Entity::None {
                    world.set(curr, Entity::Box);
                    curr = dir_vec
                        .transform_coord(robot_pos, world.size())
                        .expect("Moved out-of-bounds when setting robot pos");
                    world.set(curr, Entity::Robot);
                    world.set(robot_pos, Entity::None);
                    robot_pos = curr;
                    break;
                } else if world.get(curr) == &Entity::Wall {
                    break;
                }
            }
        } else if [Entity::WideBoxLeft, Entity::WideBoxRight].contains(world.get(curr)) {
            if [Direction::Left, Direction::Right].contains(&dir) {
                let mut entities = vec![*world.get(curr)];
                loop {
                    curr = dir_vec
                        .transform_coord(curr, world.size())
                        .expect("Moved out-of-bounds");

                    if world.get(curr) == &Entity::None {
                        curr = dir_vec.transform_coord(robot_pos, world.size()).unwrap();
                        world.set(robot_pos, Entity::None);
                        world.set(curr, Entity::Robot);
                        robot_pos = curr;

                        for i in 0..entities.len() {
                            curr = dir_vec.transform_coord(curr, world.size()).unwrap();
                            world.set(curr, entities[i]);
                        }
                        break;
                    } else if world.get(curr) == &Entity::Wall {
                        break;
                    } else {
                        entities.push(*world.get(curr));
                    }
                }
            } else {
                let mut entity_rows = {
                    let mut row_entities = HashSet::new();
                    if world.get(curr) == &Entity::WideBoxLeft {
                        let pos = Direction::Right
                            .to_coord_vec()
                            .transform_coord(curr, world.size())
                            .unwrap();
                        assert!(world.get(pos) == &Entity::WideBoxRight);
                        row_entities.insert((curr, Entity::WideBoxLeft));
                        row_entities.insert((pos, Entity::WideBoxRight));
                        vec![row_entities]
                    } else {
                        let pos = Direction::Left
                            .to_coord_vec()
                            .transform_coord(curr, world.size())
                            .unwrap();
                        assert!(world.get(pos) == &Entity::WideBoxLeft);
                        row_entities.insert((pos, Entity::WideBoxLeft));
                        row_entities.insert((curr, Entity::WideBoxRight));
                        vec![row_entities]
                    }
                };

                'outer: loop {
                    let last_row = entity_rows.last().unwrap();
                    let mut row_entities = HashSet::new();
                    for (pos, _) in last_row.iter() {
                        let new_pos = dir_vec.transform_coord(*pos, world.size()).unwrap();
                        if world.get(new_pos) == &Entity::WideBoxLeft {
                            let pos = Direction::Right
                                .to_coord_vec()
                                .transform_coord(new_pos, world.size())
                                .unwrap();

                            assert!(world.get(pos) == &Entity::WideBoxRight);
                            row_entities.insert((new_pos, Entity::WideBoxLeft));
                            row_entities.insert((pos, Entity::WideBoxRight));
                        } else if world.get(new_pos) == &Entity::WideBoxRight {
                            let pos = Direction::Left
                                .to_coord_vec()
                                .transform_coord(new_pos, world.size())
                                .unwrap();

                            assert!(world.get(pos) == &Entity::WideBoxLeft);
                            row_entities.insert((pos, Entity::WideBoxLeft));
                            row_entities.insert((new_pos, Entity::WideBoxRight));
                        } else if world.get(new_pos) == &Entity::Wall {
                            entity_rows.clear();
                            break 'outer;
                        }
                    }

                    if row_entities.is_empty() {
                        break;
                    }

                    entity_rows.push(row_entities);
                }

                if !entity_rows.is_empty() {
                    while let Some(row_entities) = entity_rows.pop() {
                        for (old_pos, entity) in row_entities {
                            let new_pos = dir_vec.transform_coord(old_pos, world.size()).unwrap();
                            world.set(new_pos, entity);
                            world.set(old_pos, Entity::None);
                        }
                    }

                    curr = dir_vec.transform_coord(robot_pos, world.size()).unwrap();
                    world.set(robot_pos, Entity::None);
                    world.set(curr, Entity::Robot);
                    robot_pos = curr;
                }
            }
        }

        //print_world(world);
        //pause();
    }
}

fn main() {
    if let Ok(lines) = read_lines("./inputs/day15/input.txt") {
        // Parsing
        let mut input = lines.flatten().into_iter();

        let mut world_input = vec![];
        while let Some(line) = input.next() {
            if line.is_empty() {
                break;
            }

            world_input.push(line);
        }

        let moves = input.join("");

        let (mut world, robot_pos) = parse_map(&world_input).unwrap();

        simulate_robot(&mut world, robot_pos, &moves);

        let res = world
            .indexed_iter()
            .filter_map(|((i, j), entity)| (entity == &Entity::Box).then_some(i * 100 + j))
            .sum::<usize>();

        println!("Star 1: {}", res);

        let (mut world, robot_pos) = parse_map(&transform_map_to_wide(&world_input)).unwrap();

        simulate_robot(&mut world, robot_pos, &moves);

        let res = world
            .indexed_iter()
            .filter_map(|((i, j), entity)| (entity == &Entity::WideBoxLeft).then_some(i * 100 + j))
            .sum::<usize>();

        println!("Star 2: {}", res);
    }
}
