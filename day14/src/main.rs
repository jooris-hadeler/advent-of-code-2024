use std::{
    collections::{HashSet, VecDeque},
    time::Instant,
};

pub type Vec2 = (isize, isize);

fn load() -> (Vec<Vec2>, Vec<Vec2>, Vec2) {
    // let (input, size) = (include_str!("example.txt"), (11, 7));
    let (input, size) = (include_str!("input.txt"), (101, 103));

    let (positions, velocities) = input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (start, velocity) = line.split_once(" ").expect("failed to split at the space");

            let split_and_parse_coords = |input: &str| {
                let (_, coords) = input.split_once("=").expect("failed to split at =");
                let (x, y) = coords.split_once(",").expect("failed to split at ,");

                (
                    x.parse::<isize>().expect("failed to parse number"),
                    y.parse::<isize>().expect("failed to parse number"),
                )
            };

            (
                split_and_parse_coords(start),
                split_and_parse_coords(velocity),
            )
        })
        .unzip();

    (positions, velocities, size)
}

fn simulate_robots(robot_position: &mut Vec<Vec2>, robot_velocity: &Vec<Vec2>, size: Vec2) {
    let (width, height) = size;

    for (pos, vel) in robot_position.iter_mut().zip(robot_velocity.iter()) {
        let (x, y) = pos;
        let &(vx, vy) = vel;

        *x += vx;
        *y += vy;

        if *x < 0 {
            *x += width;
        } else if *x >= width {
            *x -= width;
        }

        if *y < 0 {
            *y += height;
        } else if *y >= height {
            *y -= height;
        }
    }
}

fn largest_connected_area(grid: &Vec<Vec2>) -> usize {
    let cells: HashSet<Vec2> = grid.into_iter().copied().collect();

    let mut visited: HashSet<Vec2> = HashSet::new();
    let mut max_area = 0;

    fn dfs(cell: Vec2, cells: &HashSet<Vec2>, visited: &mut HashSet<Vec2>) -> usize {
        let mut stack = VecDeque::new();
        let mut area = 0;

        stack.push_back(cell);

        while let Some(current) = stack.pop_back() {
            if visited.contains(&current) {
                continue;
            }

            visited.insert(current);
            area += 1;

            let (x, y) = current;
            for neighbor in [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)] {
                if cells.contains(&neighbor) && !visited.contains(&neighbor) {
                    stack.push_back(neighbor);
                }
            }
        }

        area
    }

    for cell in cells.iter().copied() {
        if !visited.contains(&cell) {
            let area = dfs(cell, &cells, &mut visited);
            max_area = max_area.max(area);
        }
    }

    max_area
}

fn main() {
    let load_start = Instant::now();

    let (mut positions, velocities, size) = load();
    let (width, height) = size;

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let part_1_start = Instant::now();

    for _ in 0..100 {
        simulate_robots(&mut positions, &velocities, size);
    }

    let top_left_quadrant = positions
        .iter()
        .filter(|(x, y)| *x < width / 2 && *y < height / 2)
        .count();

    let top_right_quadrant = positions
        .iter()
        .filter(|(x, y)| *x > width / 2 && *y < height / 2)
        .count();

    let bottom_left_quadrant = positions
        .iter()
        .filter(|(x, y)| *x < width / 2 && *y > height / 2)
        .count();

    let bottom_right_quadrant = positions
        .iter()
        .filter(|(x, y)| *x > width / 2 && *y > height / 2)
        .count();

    let part_1_solution: usize =
        top_left_quadrant * top_right_quadrant * bottom_left_quadrant * bottom_right_quadrant;

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    let part_2_start = Instant::now();

    let mut part_2_solution = 0;

    for idx in 100.. {
        // if there are more than 25 robots connected this must be a christmas tree
        if largest_connected_area(&positions) > 25 {
            part_2_solution = idx;
            break;
        }

        simulate_robots(&mut positions, &velocities, size);
    }

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
