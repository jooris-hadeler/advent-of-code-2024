use std::{env, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Cell {
    Wall,
    Box,
    LeftBox,
    RightBox,
    Robot,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Move {
    Left,
    Up,
    Right,
    Down,
}

impl Move {
    pub fn get_direction(&self) -> (isize, isize) {
        match self {
            Move::Left => (-1, 0),
            Move::Up => (0, -1),
            Move::Right => (1, 0),
            Move::Down => (0, 1),
        }
    }
}

fn print_map(map: &Vec<Vec<Cell>>) {
    for row in map.iter() {
        for cell in row.iter() {
            print!(
                "{}",
                match cell {
                    Cell::Box => 'O',
                    Cell::LeftBox => '[',
                    Cell::RightBox => ']',
                    Cell::Wall => '#',
                    Cell::Robot => '@',
                    Cell::Empty => '.',
                }
            )
        }
        println!()
    }
    println!()
}

fn load() -> (Vec<Vec<Cell>>, Vec<Move>) {
    let input = include_str!("small_example.txt");

    let mut map = Vec::new();
    let mut moves = Vec::new();

    let mut parse_map = true;

    for line in input.lines().map(str::trim) {
        if line.is_empty() {
            parse_map = false;
            continue;
        }

        if parse_map {
            map.push(
                line.chars()
                    .map(|ch| match ch {
                        '#' => Cell::Wall,
                        'O' => Cell::Box,
                        '@' => Cell::Robot,
                        '.' => Cell::Empty,
                        _ => panic!("invalid input"),
                    })
                    .collect(),
            );
        } else {
            moves.extend(line.chars().map(|ch| match ch {
                '<' => Move::Left,
                '>' => Move::Right,
                '^' => Move::Up,
                'v' => Move::Down,
                _ => panic!("invalid input"),
            }));
        }
    }

    (map, moves)
}

fn try_move(
    map: &mut Vec<Vec<Cell>>,
    robot_x: isize,
    robot_y: isize,
    robot_move: &Move,
    is_acting: bool,
) -> bool {
    let (move_x, move_y) = robot_move.get_direction();
    let new_x = robot_x + move_x;
    let new_y = robot_y + move_y;

    let can_move = match (map[new_y as usize][new_x as usize], robot_move) {
        (Cell::Empty, _) => true,
        (Cell::Wall, _) => false,
        (Cell::LeftBox, Move::Left | Move::Right)
        | (Cell::RightBox, Move::Left | Move::Right)
        | (Cell::Robot, _)
        | (Cell::Box, _) => try_move(map, new_x, new_y, robot_move, is_acting),
        (Cell::LeftBox, Move::Up | Move::Down) => {
            try_move(map, new_x, new_y, robot_move, is_acting)
                && try_move(map, new_x + 1, new_y, robot_move, is_acting)
        }
        (Cell::RightBox, Move::Up | Move::Down) => {
            try_move(map, new_x, new_y, robot_move, is_acting)
                && try_move(map, new_x - 1, new_y, robot_move, is_acting)
        }
    };

    if can_move && is_acting {
        map[new_y as usize][new_x as usize] = map[robot_y as usize][robot_x as usize];
        map[robot_y as usize][robot_x as usize] = Cell::Empty;
    }

    can_move
}

fn transform_map(map: &Vec<Vec<Cell>>) -> Vec<Vec<Cell>> {
    map.iter()
        .map(|row| {
            row.iter()
                .flat_map(|cell| match cell {
                    Cell::Wall => [Cell::Wall, Cell::Wall],
                    Cell::Box => [Cell::LeftBox, Cell::RightBox],
                    Cell::Robot => [Cell::Robot, Cell::Empty],
                    Cell::Empty => [Cell::Empty, Cell::Empty],
                    _ => panic!("invalid input"),
                })
                .collect()
        })
        .collect()
}

fn solve(
    mut map: Vec<Vec<Cell>>,
    moves: &Vec<Move>,
    acting_level: &[bool],
    verbose: bool,
) -> usize {
    let (mut robot_x, mut robot_y) = (0..map.len())
        .flat_map(|y| (0..map.len()).map(move |x| (x, y)))
        .find(|&(x, y)| map[y][x] == Cell::Robot)
        .map(|(x, y)| (x as isize, y as isize))
        .expect("failed to find robot");

    if verbose {
        print_map(&map);
    }

    for current_move in moves {
        if verbose {
            println!("Move: {current_move:?}");
        }

        if acting_level
            .iter()
            .all(|&acting| try_move(&mut map, robot_x, robot_y, current_move, acting))
        {
            let (move_x, move_y) = current_move.get_direction();
            robot_x += move_x;
            robot_y += move_y;
        }

        if verbose {
            print_map(&map);
        }
    }

    (0..map.len())
        .flat_map(|y| (0..map[y].len()).map(move |x| (x, y)))
        .filter_map(|(x, y)| match map[y][x] {
            Cell::Box | Cell::LeftBox => Some(100 * y + x),
            _ => None,
        })
        .sum()
}

fn main() {
    let verbose = env::args().nth(1).is_some_and(|arg| arg == "--verbose");

    let load_start = Instant::now();

    let (part_1_map, moves) = load();
    let part_2_map = transform_map(&part_1_map);

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution: usize = solve(part_1_map, &moves, &[true], verbose);

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    let part_2_start = Instant::now();

    let part_2_solution: usize = solve(part_2_map, &moves, &[false, true], verbose);

    let part_2_elapsed = part_2_start.elapsed();

    println!(" Part 2: {}", part_2_solution);
    println!("   Time: {:?}", part_2_elapsed);
    println!();

    println!("  Total:");
    println!(
        "   Time: {:?}",
        load_elapsed + part_1_elapsed + part_2_elapsed
    );
    println!();
}
