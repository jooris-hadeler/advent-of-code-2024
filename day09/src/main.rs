use std::{iter::repeat_n, time::Instant, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    File(usize),
    Empty,
}

fn load() -> Vec<Cell> {
    let input = include_str!("input.txt");

    let mut id_counter = 0;

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
        .collect()
}

fn main() {
    let load_start = Instant::now();

    let disk_map = load();

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

        if matches!(cell, Cell::Empty) {
            continue;
        }

        match part_1_disk_map
            .iter_mut()
            .skip(last_empty_index)
            .enumerate()
            .find(|(_, cell)| matches!(cell, Cell::Empty))
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

    let mut empty_spaces = Vec::new();
    let mut file_spaces = Vec::new();

    let mut empty_start = None;
    let mut empty_length: usize = 0;

    let mut file_start = None;
    let mut file_id = 0;
    let mut file_length: usize = 0;

    for (i, cell) in part_2_disk_map.iter().enumerate() {
        match cell {
            Cell::Empty => {
                if let Some(file_start) = file_start.take() {
                    file_spaces.push((file_start, file_length));
                }

                // Start a new empty block
                if empty_start.is_none() {
                    empty_start = Some(i);
                    empty_length = 0;
                }

                empty_length += 1;
            }
            &Cell::File(id) => {
                if let Some(empty_start) = empty_start.take() {
                    empty_spaces.push((empty_start, empty_length));
                }

                if id != file_id {
                    if let Some(file_start) = file_start.take() {
                        file_spaces.push((file_start, file_length));
                        file_length = 0;
                    }
                }

                // Start a new file block
                if file_start.is_none() {
                    file_start = Some(i);
                    file_length = 0;
                    file_id = id;
                }

                file_length += 1;
            }
        }
    }

    if let Some(file_start) = file_start.take() {
        file_spaces.push((file_start, file_length));
    }

    if let Some(empty_start) = empty_start.take() {
        empty_spaces.push((empty_start, empty_length));
    }

    while let Some((file_index, file_length)) = file_spaces.pop() {
        let result = empty_spaces
            .iter()
            .copied()
            .position(|(_, empty_length)| empty_length >= file_length);

        if result.is_none() {
            continue;
        }

        let idx = result.unwrap();
        let (empty_index, empty_length) = empty_spaces[idx];

        if empty_index > file_index {
            continue;
        }

        let left_over_empty_space = empty_length - file_length;

        for i in 0..file_length {
            part_2_disk_map.swap(file_index + i, empty_index + i);
        }

        if left_over_empty_space > 0 {
            empty_spaces[idx] = (empty_index + file_length, left_over_empty_space);
        } else {
            empty_spaces[idx] = (usize::MAX, 0);
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
