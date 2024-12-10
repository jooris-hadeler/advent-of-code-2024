use std::{iter::repeat_n, time::Instant, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    File(usize),
    Empty,
}

impl Cell {
    pub fn is_empty(&self) -> bool {
        match self {
            Cell::File(_) => false,
            Cell::Empty => true,
        }
    }

    pub fn is_file(&self) -> bool {
        match self {
            Cell::File(_) => true,
            Cell::Empty => false,
        }
    }

    pub fn as_file(&self) -> usize {
        match self {
            &Cell::File(id) => id,
            Cell::Empty => panic!(),
        }
    }
}

fn load() -> (Vec<Cell>, usize) {
    let input = include_str!("input.txt");

    let mut id_counter = 0;

    (
        input
            .trim()
            .chars()
            .map(|ch| ch.to_digit(10).expect("failed to convert char") as usize)
            .enumerate()
            .flat_map(|(idx, length)| {
                let id = id_counter;

                if idx % 2 == 0 {
                    id_counter += 1;
                    repeat_n(Cell::File(id), length)
                } else {
                    repeat_n(Cell::Empty, length)
                }
            })
            .collect(),
        id_counter,
    )
}

fn find_index_and_length_by_id(map: &Vec<Cell>, id: usize) -> Option<(usize, usize)> {
    let mut iter = map.iter().enumerate().rev();

    let (end_idx, _) = iter.find(|(_, cell)| cell.is_file() && cell.as_file() == id)?;

    let length = iter
        .take_while(|(_, cell)| cell.is_file() && cell.as_file() == id)
        .count();

    Some((end_idx - length, length + 1))
}

fn main() {
    let load_start = Instant::now();

    let (disk_map, max_file_id) = load();

    let load_end = Instant::now();

    println!("Loading:");
    println!("   Time: {:?}", load_end - load_start);
    println!();

    let part_1_start = Instant::now();

    let mut iterations: usize = 0;
    let mut last_empty_index = 0;

    let mut part_1_disk_map = disk_map.clone();

    while let Some(cell) = part_1_disk_map.pop() {
        iterations += 1;

        if cell.is_empty() {
            continue;
        }

        match part_1_disk_map
            .iter_mut()
            .skip(last_empty_index)
            .enumerate()
            .find(|(_, cell)| cell.is_empty())
        {
            Some((idx, empty_cell)) => {
                *empty_cell = cell;
                last_empty_index += idx + 1;
            }
            None => {
                part_1_disk_map.push(cell);
                break;
            }
        }
    }

    let part_1_solution: usize = part_1_disk_map
        .iter()
        .enumerate()
        .map(|(idx, cell)| match cell {
            &Cell::File(id) => id * idx,
            Cell::Empty => 0,
        })
        .sum();

    let part_1_end = Instant::now();

    println!(" Part 1: {} ({})", part_1_solution, iterations);
    println!("   Time: {:?}", part_1_end - part_1_start);
    println!();

    let part_2_start = Instant::now();

    let mut part_2_disk_map = disk_map;

    let mut empty_spaces = {
        let mut empty_spaces = Vec::new();

        let mut current_start = None;
        let mut current_length: usize = 0;

        for (i, cell) in part_2_disk_map.iter().enumerate() {
            match cell {
                Cell::Empty => {
                    if current_start.is_none() {
                        current_start = Some(i); // Start a new block
                    }
                    current_length += 1;
                }
                _ => {
                    // Reset block tracking on encountering a non-empty cell
                    if let Some(current_start) = current_start.take() {
                        empty_spaces.push((current_start, current_length));
                    }

                    current_length = 0;
                }
            }
        }

        empty_spaces
    };

    for file_id in (0..max_file_id).rev() {
        let (file_index, file_length) =
            find_index_and_length_by_id(&part_2_disk_map, file_id).expect("failed to find file");

        let Some((idx, (empty_index, empty_length))) = empty_spaces
            .iter()
            .copied()
            .enumerate()
            .find(|&(_, (_, empty_length))| empty_length >= file_length)
        else {
            continue;
        };

        if empty_index > file_index {
            continue;
        }

        let left_over = empty_length - file_length;

        for i in 0..file_length {
            part_2_disk_map.swap(file_index + i, empty_index + i);
        }

        if left_over > 0 {
            empty_spaces[idx] = (empty_index + file_length, left_over);
        } else {
            empty_spaces.remove(idx);
        }
    }

    let part_2_solution: usize = part_2_disk_map
        .iter()
        .enumerate()
        .map(|(idx, cell)| match cell {
            &Cell::File(id) => id * idx,
            Cell::Empty => 0,
        })
        .sum();

    let part_2_end = Instant::now();

    println!(" Part 2: {}", part_2_solution);
    println!("   Time: {:?}", part_2_end - part_2_start);
    println!();

    println!("  Total:");
    println!(
        "   Time: {:?}",
        (load_end - load_start) + (part_1_end - part_1_start) + (part_2_end - part_2_start)
    );
    println!();
}
