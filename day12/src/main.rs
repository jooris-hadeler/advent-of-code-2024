use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Cell {
    Visited(char),
    Unvisited(char),
}

fn load() -> Vec<Vec<Cell>> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().map(|ch| Cell::Unvisited(ch)).collect())
        .collect()
}

const DIRECTIONS: [(isize, isize); 4] = [(-1, 0), (0, -1), (1, 0), (0, 1)];

fn explore_garden(
    garden: &mut Vec<Vec<Cell>>,
    x: usize,
    y: usize,
) -> (usize, Vec<HashSet<(usize, usize)>>) {
    let size = garden.len();

    let mut area = 0;
    let mut perimeter =
        HashMap::<_, _>::from_iter(DIRECTIONS.map(|dir| (dir, HashSet::<(usize, usize)>::new())));

    let mut stack = Vec::new();

    stack.push((x, y));

    while let Some((cur_x, cur_y)) = stack.pop() {
        let Cell::Unvisited(cur_char) = garden[cur_y][cur_x] else {
            continue;
        };

        garden[cur_y][cur_x] = Cell::Visited(cur_char);
        area += 1;

        for (diff_x, diff_y) in DIRECTIONS {
            let mut record_perimeter = || {
                _ = perimeter
                    .get_mut(&(diff_x, diff_y))
                    .unwrap()
                    .insert((cur_x, cur_y))
            };

            let new_x = cur_x as isize + diff_x;
            let new_y = cur_y as isize + diff_y;

            let in_bounds =
                new_x >= 0 && new_y >= 0 && new_x < size as isize && new_y < size as isize;

            if !in_bounds {
                record_perimeter();

                continue;
            };

            let (new_x, new_y) = (new_x as usize, new_y as usize);

            match garden[new_y][new_x] {
                Cell::Unvisited(new_char) if new_char != cur_char => record_perimeter(),
                Cell::Visited(new_char) if new_char != cur_char => record_perimeter(),
                Cell::Visited(_) => (),
                _ => stack.push((new_x, new_y)),
            }
        }
    }

    let perimeter = perimeter.into_values().collect();

    (area, perimeter)
}

fn count_neighborhoods(coords: &HashSet<(usize, usize)>) -> usize {
    // Check if two points are neighbors
    fn is_neighbor(a: &(usize, usize), b: &(usize, usize)) -> bool {
        a.0.abs_diff(b.0) + a.1.abs_diff(b.1) == 1
    }

    // Perform DFS to mark all nodes in the same connected component
    fn dfs(
        node: &(usize, usize),
        coords: &HashSet<(usize, usize)>,
        visited: &mut HashSet<(usize, usize)>,
    ) {
        let mut stack = vec![*node];
        while let Some(current) = stack.pop() {
            for neighbor in coords.iter() {
                if !visited.contains(neighbor) && is_neighbor(&current, neighbor) {
                    visited.insert(*neighbor);
                    stack.push(*neighbor);
                }
            }
        }
    }

    let mut visited = HashSet::new();
    let mut count = 0;

    for coord in coords {
        if !visited.contains(coord) {
            // Start a new DFS for a new component
            count += 1;
            visited.insert(*coord);
            dfs(coord, coords, &mut visited);
        }
    }

    count
}

fn main() {
    let load_start = Instant::now();

    let mut garden = load();
    let size = garden.len();

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let part_1_start = Instant::now();

    let mut aps = Vec::new();

    for y in 0..size {
        for x in 0..size {
            if let Cell::Visited(_) = garden[y][x] {
                continue;
            }

            aps.push(explore_garden(&mut garden, x, y));
        }
    }

    let part_1_solution: usize = aps
        .iter()
        .map(|(area, perimeter_coords)| {
            let perimeter_length: usize = perimeter_coords.iter().map(|coords| coords.len()).sum();

            area * perimeter_length
        })
        .sum();

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    let part_2_start = Instant::now();

    let part_2_solution: usize = aps
        .iter()
        .map(|(area, perimeter_coords)| {
            let side_count: usize = perimeter_coords
                .iter()
                .map(|coords| count_neighborhoods(coords))
                .sum();

            area * side_count
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
