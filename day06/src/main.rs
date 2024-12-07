use std::{
    collections::HashSet,
    iter::{once, repeat},
    mem,
    sync::atomic::{AtomicUsize, Ordering},
    thread,
    time::Instant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Obstacle,
    Visited(u8),
    Outside,
    Empty,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    pub x: usize,
    pub y: usize,
}

impl Position {
    pub fn step(self, dir: Direction) -> Position {
        let Self { x, y } = self;

        match dir {
            Direction::Up => Position {
                x,
                y: y.saturating_sub(1),
            },
            Direction::Down => Position {
                x,
                y: y.saturating_add(1),
            },
            Direction::Left => Position {
                x: x.saturating_sub(1),
                y,
            },
            Direction::Right => Position {
                x: x.saturating_add(1),
                y,
            },
        }
    }
}

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Direction {
    Up = 1 << 0,
    Down = 1 << 1,
    Left = 1 << 2,
    Right = 1 << 3,
}

impl Direction {
    pub fn rotate(self) -> Self {
        match self {
            Direction::Up => Direction::Right,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Right => Direction::Down,
        }
    }

    #[inline]
    pub fn mask(self) -> u8 {
        self as u8
    }
}

#[derive(Debug, Clone)]
struct Map {
    tiles: Vec<Tile>,
    pub width: usize,
    pub height: usize,
}

impl Map {
    pub fn new(tiles: Vec<Tile>, width: usize, height: usize) -> Self {
        Self {
            tiles,
            width,
            height,
        }
    }

    pub fn reset(&mut self) {
        self.tiles
            .iter_mut()
            .filter(|tile| matches!(tile, Tile::Visited(_)))
            .for_each(|tile| *tile = Tile::Empty);
    }

    pub fn mark_as_visited(&mut self, pos: Position, dir: Direction) {
        let Position { x, y } = pos;

        assert!(x < self.width, "x is out of bounds");
        assert!(y < self.height, "y is out of bounds");

        let index = y * self.width + x;

        self.tiles.get_mut(index).map(|tile| match tile {
            empty @ Tile::Empty => {
                *empty = Tile::Visited(dir.mask());
                false
            }
            Tile::Visited(visited_dirs) => {
                let has_been_here = (*visited_dirs & dir.mask()) != 0;
                *visited_dirs |= dir.mask();
                has_been_here
            }
            _ => false,
        });
    }

    pub fn get_tile(&self, pos: Position) -> Tile {
        let Position { x, y } = pos;

        assert!(x < self.width, "x is out of bounds");
        assert!(y < self.height, "y is out of bounds");

        let index = y * self.width + x;

        self.tiles.get(index).copied().unwrap_or(Tile::Outside)
    }

    pub fn set_tile(&mut self, pos: Position, tile: Tile) {
        let Position { x, y } = pos;

        assert!(x < self.width, "x is out of bounds");
        assert!(y < self.height, "y is out of bounds");

        let index = y * self.width + x;

        let Some(tile_ref) = self.tiles.get_mut(index) else {
            return;
        };

        *tile_ref = tile;
    }

    pub fn replace_tile(&mut self, pos: Position, tile: Tile) -> Tile {
        let Position { x, y } = pos;

        assert!(x < self.width, "x is out of bounds");
        assert!(y < self.height, "y is out of bounds");

        let index = y * self.width + x;

        let Some(tile_ref) = self.tiles.get_mut(index) else {
            return Tile::Outside;
        };

        mem::replace(tile_ref, tile)
    }

    pub fn count_visited(&self) -> usize {
        self.tiles
            .iter()
            .filter(|tile| matches!(tile, Tile::Visited(_)))
            .count()
    }

    pub fn visited_iter<'a>(&'a self) -> impl Iterator<Item = Position> + 'a {
        self.tiles
            .iter()
            .enumerate()
            .filter(|(_, tile)| matches!(tile, Tile::Visited(_)))
            .map(|(idx, _)| {
                let y = idx / self.width;
                let x = idx % self.width;

                Position { x, y }
            })
    }
}

struct Guard<'a> {
    map: &'a mut Map,
    pos: Position,
    dir: Direction,
}

enum StepResult {
    Step,
    Finish,
    Loop,
}

impl<'a> Guard<'a> {
    pub fn new(map: &'a mut Map, pos: Position, dir: Direction) -> Self {
        Self { map, pos, dir }
    }

    pub fn step(&mut self) -> StepResult {
        self.map.mark_as_visited(self.pos, self.dir);

        let new_pos = self.pos.step(self.dir);

        match self.map.get_tile(new_pos) {
            Tile::Obstacle => {
                self.dir = self.dir.rotate();
                StepResult::Step
            }
            Tile::Empty => {
                self.pos = new_pos;
                StepResult::Step
            }
            Tile::Visited(dir) => {
                if (dir & self.dir as u8) != 0 {
                    // we have been here before looking in the same direction, that means we found loop
                    StepResult::Loop
                } else {
                    self.pos = new_pos;
                    StepResult::Step
                }
            }
            Tile::Outside => {
                self.pos = new_pos;
                StepResult::Finish
            }
        }
    }

    pub fn simulate(&mut self) -> bool {
        loop {
            match self.step() {
                StepResult::Step => (),
                StepResult::Finish => break false,
                StepResult::Loop => break true,
            }
        }
    }
}

fn load() -> (Map, Position) {
    let input = include_str!("input.txt");

    let height = 2 + input.lines().filter(|line| !line.is_empty()).count();

    let width = 2 + input
        .lines()
        .nth(0)
        .map(str::trim)
        .map(|line| line.chars().count())
        .unwrap();

    let guard_x = AtomicUsize::new(0);
    let guard_y = AtomicUsize::new(0);

    let guard_x_ref = &guard_x;
    let guard_y_ref = &guard_y;

    let tile_iter = repeat(Tile::Outside)
        .take(width)
        .chain(
            input
                .lines()
                .map(str::trim)
                .filter(|line| !line.is_empty())
                .enumerate()
                .map(|(y, line)| {
                    let line_tiles = line.chars().enumerate().map(move |(x, ch)| match ch {
                        '.' => Tile::Empty,
                        '^' => {
                            guard_x_ref.store(x + 1, Ordering::Relaxed);
                            guard_y_ref.store(y + 1, Ordering::Relaxed);
                            Tile::Empty
                        }
                        '#' => Tile::Obstacle,
                        _ => unreachable!(),
                    });

                    once(Tile::Outside)
                        .chain(line_tiles)
                        .chain(once(Tile::Outside))
                })
                .flatten(),
        )
        .chain(repeat(Tile::Outside).take(width));

    let map = Map::new(tile_iter.collect(), width, height);
    let guard_position = Position {
        x: guard_x.into_inner(),
        y: guard_y.into_inner(),
    };

    (map, guard_position)
}

fn main() {
    let load_start = Instant::now();

    let (mut map, start_pos) = load();

    let load_end = Instant::now();

    println!("Loading:");
    println!("   Time: {:?}", load_end - load_start);
    println!();

    let part_1_start = Instant::now();

    // simulate the guard on our map
    let mut guard = Guard::new(&mut map, start_pos, Direction::Up);
    guard.simulate();

    let visited_count = map.count_visited();

    let part_1_end = Instant::now();

    println!("Part 1: {}", visited_count);
    println!("  Time: {:?}", part_1_end - part_1_start);
    println!();

    let part_2_start = Instant::now();

    // dedupe visited fields
    let mut visited_fields = map.visited_iter().collect::<Vec<_>>();
    let visited_fields_set: HashSet<_> = visited_fields.drain(..).collect();
    visited_fields.extend(visited_fields_set.into_iter());

    map.reset();

    // calculate chunk size for n threads
    let thread_count = 16;
    let chunk_size = match visited_fields.len() % thread_count {
        0 => visited_fields.len() / thread_count,
        left_over => (visited_fields.len() + thread_count - left_over) / thread_count,
    };

    // each thread checks its part of the chunks for loops
    let mut handles = Vec::new();
    for chunk in visited_fields.chunks(chunk_size) {
        let mut thread_map = map.clone();
        let chunk = chunk.into_iter().copied().collect::<Vec<_>>();

        let handle = thread::spawn(move || {
            let mut loop_count: usize = 0;

            for position in chunk {
                // cannot place obstacle at start position
                if position == start_pos {
                    continue;
                }

                // change tile
                let previous = thread_map.replace_tile(position, Tile::Obstacle);

                // check if the guard loops
                let mut guard = Guard::new(&mut thread_map, start_pos, Direction::Up);
                if guard.simulate() {
                    loop_count += 1;
                }

                // revert tile change
                thread_map.set_tile(position, previous);
                thread_map.reset();
            }

            loop_count
        });

        handles.push(handle);
    }

    // calculate sum of loops
    let loop_count: usize = handles
        .into_iter()
        .map(|handle| handle.join().expect("failed to join handle"))
        .sum();

    let part_2_end = Instant::now();

    println!("Part 2: {}", loop_count);
    println!("  Time: {:?}", part_2_end - part_2_start);
    println!();

    println!(
        "Total: {:?}",
        (part_2_end - part_2_start) + (part_1_end - part_1_start) + (load_end - load_start)
    );
}
