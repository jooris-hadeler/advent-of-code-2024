use std::{time::Instant, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct FileBlock {
    pub id: usize,
    pub length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct EmptyBlock {
    pub length: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    File(FileBlock),
    Empty(EmptyBlock),
}

impl Block {
    pub const fn new_file(id: usize, length: usize) -> Self {
        Self::File(FileBlock { id, length })
    }

    pub const fn new_empty(length: usize) -> Self {
        Self::Empty(EmptyBlock { length })
    }

    pub fn is_file(&self) -> bool {
        match self {
            Block::File { .. } => true,
            _ => false,
        }
    }

    pub fn is_empty(&self) -> bool {
        match self {
            Block::Empty { .. } => true,
            _ => false,
        }
    }

    #[must_use]
    pub fn as_file(self) -> FileBlock {
        match self {
            Block::File(file) => file,
            Block::Empty(..) => panic!(),
        }
    }

    #[must_use]
    pub fn as_empty(self) -> EmptyBlock {
        match self {
            Block::File(..) => panic!(),
            Block::Empty(empty) => empty,
        }
    }
}

fn load() -> (Vec<Block>, usize) {
    let input = include_str!("input.txt");

    let mut id_counter = 0;

    (
        input
            .trim()
            .chars()
            .map(|ch| ch.to_digit(10).expect("failed to convert char") as usize)
            .enumerate()
            .map(|(idx, length)| {
                let id = id_counter;

                if idx % 2 == 0 {
                    id_counter += 1;
                    Block::new_file(id, length)
                } else {
                    Block::new_empty(length)
                }
            })
            .collect(),
        id_counter,
    )
}

fn main() {
    let load_start = Instant::now();

    let (disk_map, max_file_id) = load();

    let load_end = Instant::now();

    println!("Loading:");
    println!("   Time: {:?}", load_end - load_start);
    println!();

    let part_1_start = Instant::now();

    let mut part_1_disk_map = disk_map.clone();

    loop {
        let first_empty_index = part_1_disk_map.iter().position(Block::is_empty).unwrap();
        let last_filled_index = part_1_disk_map.len()
            - 1
            - part_1_disk_map
                .iter()
                .rev()
                .position(Block::is_file)
                .unwrap();

        if last_filled_index < first_empty_index {
            break;
        }

        let empty_block = part_1_disk_map[first_empty_index].as_empty();
        let file_block = part_1_disk_map[last_filled_index].as_file();

        if empty_block.length < file_block.length {
            let left_over_space = file_block.length - empty_block.length;

            part_1_disk_map[first_empty_index] = Block::new_file(file_block.id, empty_block.length);
            part_1_disk_map[last_filled_index] = Block::new_file(file_block.id, left_over_space);
        } else if empty_block.length == file_block.length {
            part_1_disk_map.swap(first_empty_index, last_filled_index);
        } else {
            let left_over_empty_space = empty_block.length - file_block.length;

            part_1_disk_map.swap(first_empty_index, last_filled_index);
            part_1_disk_map.insert(
                first_empty_index + 1,
                Block::new_empty(left_over_empty_space),
            );
        }
    }

    let mut idx = 0;

    let part_1_solution: usize = part_1_disk_map
        .iter()
        .copied()
        .filter(Block::is_file)
        .map(Block::as_file)
        .map(|file| {
            let current_idx = idx;
            idx += file.length;
            (0..file.length).map(move |off| (current_idx + off) * file.id)
        })
        .flatten()
        .sum();

    let part_1_end = Instant::now();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_end - part_1_start);
    println!();

    let part_2_start = Instant::now();

    let mut part_2_disk_map = disk_map;

    for file_id in (0..max_file_id).rev() {
        let file_index = part_2_disk_map
            .iter()
            .position(|block| block.is_file() && block.as_file().id == file_id)
            .unwrap();

        let file_block = part_2_disk_map[file_index].as_file();

        // try finding empty index that has enough space for our file block
        let Some(first_fitting_empty_index) = part_2_disk_map
            .iter()
            .position(|item| item.is_empty() && item.as_empty().length >= file_block.length)
        else {
            continue;
        };

        // if the first empty block is after the file block we dont move the block
        if first_fitting_empty_index > file_index {
            continue;
        }

        let empty_block = part_2_disk_map[first_fitting_empty_index].as_empty();

        // if we have more empty_space than space ensure that we add a padding free block otherwise just swap the blocks
        if empty_block.length > file_block.length {
            let left_over_empty_space = empty_block.length - file_block.length;

            part_2_disk_map[file_index] = Block::new_empty(file_block.length);
            part_2_disk_map[first_fitting_empty_index] =
                Block::new_file(file_id, file_block.length);

            part_2_disk_map.insert(
                first_fitting_empty_index + 1,
                Block::new_empty(left_over_empty_space),
            );
        } else {
            part_2_disk_map.swap(first_fitting_empty_index, file_index);
        }
    }

    let mut idx = 0;

    let part_2_solution: usize = part_2_disk_map
        .iter()
        .copied()
        .map(|block| match block {
            Block::File(file) => {
                let current_idx = idx;
                idx += file.length;
                (0..file.length)
                    .map(move |off| (current_idx + off) * file.id)
                    .sum()
            }
            Block::Empty(empty) => {
                idx += empty.length;
                0
            }
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
