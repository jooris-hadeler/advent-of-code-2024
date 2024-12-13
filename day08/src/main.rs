use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

fn load() -> (HashMap<char, Vec<(isize, isize)>>, (isize, isize)) {
    let input = include_str!("input.txt");

    let mut positions = HashMap::new();

    let mut height: isize = 0;
    let mut width: isize = 0;

    for (y, line) in input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .enumerate()
    {
        for (x, ch) in line
            .chars()
            .enumerate()
            .filter(|(_, ch)| matches!(ch, 'a'..='z' | 'A'..='Z' | '0'..='9'))
        {
            if !positions.contains_key(&ch) {
                positions.insert(ch, Vec::new());
            }

            positions
                .get_mut(&ch)
                .unwrap()
                .push((x as isize, y as isize));
        }

        width = line.chars().count() as isize;
        height = y as isize + 1;
    }

    (positions, (width, height))
}

fn unique_pairs<T: Clone + Eq + std::hash::Hash, I: Iterator<Item = T>>(set: I) -> Vec<(T, T)> {
    let mut pairs = Vec::new();
    let items: Vec<T> = set.collect(); // Collect items into a Vec for indexed access

    for i in 0..items.len() {
        for j in i + 1..items.len() {
            pairs.push((items[i].clone(), items[j].clone())); // Add the pair
        }
    }

    pairs
}

fn main() {
    let load_start = Instant::now();

    let (positions, (width, height)) = load();

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let mut antinode_positions = HashSet::new();

    let part_1_start = Instant::now();

    for (_, positions) in positions.iter() {
        for ((ax, ay), (bx, by)) in unique_pairs(positions.iter().copied()) {
            let diffx = ax - bx;
            let diffy = ay - by;

            let cx = ax + diffx;
            let cy = ay + diffy;

            if cx >= 0 && cx < width && cy >= 0 && cy < height {
                antinode_positions.insert((cx, cy));
            }

            let dx = bx - diffx;
            let dy = by - diffy;

            if dx >= 0 && dx < width && dy >= 0 && dy < height {
                antinode_positions.insert((dx, dy));
            }
        }
    }

    let part_1_solution: usize = antinode_positions.len();

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    antinode_positions.clear();

    let part_2_start = Instant::now();

    for (_, positions) in positions.iter() {
        for ((ax, ay), (bx, by)) in unique_pairs(positions.iter().copied()) {
            let diffx = ax - bx;
            let diffy = ay - by;

            let mut cx = ax + diffx;
            let mut cy = ay + diffy;

            while cx >= 0 && cx < width && cy >= 0 && cy < height {
                antinode_positions.insert((cx, cy));

                cy += diffy;
                cx += diffx;
            }

            let mut dx = bx - diffx;
            let mut dy = by - diffy;

            while dx >= 0 && dx < width && dy >= 0 && dy < height {
                antinode_positions.insert((dx, dy));

                dy -= diffy;
                dx -= diffx;
            }

            antinode_positions.insert((ax, ay));
            antinode_positions.insert((bx, by));
        }
    }

    let part_2_solution = antinode_positions.len();

    let part_2_elapsed = part_2_start.elapsed();

    println!(" Part 2: {}", part_2_solution);
    println!("   Time: {:?}", part_2_elapsed);
    println!();

    println!("  Total:");
    println!(
        "   Time: {:?}",
        (load_elapsed) + (part_1_elapsed) + (part_2_elapsed)
    );
    println!();
}
