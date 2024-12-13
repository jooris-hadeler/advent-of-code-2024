use std::{collections::HashSet, time::Instant};

fn load() -> (Vec<Vec<usize>>, usize) {
    let input = include_str!("input.txt");

    let map: Vec<Vec<_>> = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.chars()
                .map(|ch| ch.to_digit(10).unwrap() as usize)
                .collect()
        })
        .collect();

    let size = map.len();

    (map, size)
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];

fn score(map: &Vec<Vec<usize>>, x: usize, y: usize, size: usize, allow_revisit: bool) -> usize {
    if map[y][x] != 0 {
        return 0;
    }

    let mut answer = 0;

    let mut stack = vec![(x, y)];
    let mut visited = HashSet::new();

    // Depth first search
    while let Some((cur_x, cur_y)) = stack.pop() {
        let cur_elev = map[cur_y][cur_x];

        if !allow_revisit && visited.contains(&(cur_x, cur_y)) {
            continue;
        }

        visited.insert((cur_x, cur_y));

        if cur_elev == 9 {
            answer += 1;
            continue;
        }

        for (step_x, step_y) in DIRECTIONS {
            let next_x = cur_x as isize + step_x;
            let next_y = cur_y as isize + step_y;

            let is_in_grid =
                (0 <= next_x && next_x < size as isize) && (0 <= next_y && next_y < size as isize);

            if !is_in_grid {
                continue;
            }

            let next_x = next_x as usize;
            let next_y = next_y as usize;
            let next_elev = map[next_y][next_x];

            if next_elev != cur_elev + 1 {
                continue;
            }

            stack.push((next_x, next_y));
        }
    }

    answer
}

fn main() {
    let load_start = Instant::now();

    let (map, size) = load();

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution: usize = (0..size)
        .flat_map(|y| {
            (0..size)
                .map(move |x| (x, y))
                .map(|(x, y)| score(&map, x, y, size, false))
        })
        .sum();

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    let part_2_start = Instant::now();

    let part_2_solution: usize = (0..size)
        .flat_map(|y| {
            (0..size)
                .map(move |x| (x, y))
                .map(|(x, y)| score(&map, x, y, size, true))
        })
        .sum();

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
