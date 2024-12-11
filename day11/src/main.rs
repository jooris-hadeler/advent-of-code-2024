use cached::proc_macro::cached;
use std::time::Instant;

fn load() -> Vec<usize> {
    let input = include_str!("input.txt");

    input
        .trim()
        .split(" ")
        .map(|part| part.parse().expect("failed to parse number"))
        .collect()
}

#[cached]
fn simulate_stone(stone: usize, iterations: usize) -> usize {
    if iterations == 0 {
        return 1;
    }

    if stone == 0 {
        return simulate_stone(1, iterations - 1);
    }

    let digits = stone.checked_ilog10().unwrap_or(0) + 1;

    if digits & 1 == 0 {
        let divisor = 10usize.pow(digits / 2);

        let left = stone / divisor;
        let right = stone % divisor;

        return simulate_stone(left, iterations - 1) + simulate_stone(right, iterations - 1);
    }

    simulate_stone(stone * 2024, iterations - 1)
}

fn main() {
    let load_start = Instant::now();

    let stones = load();

    let load_end = Instant::now();

    println!("Loading:");
    println!("   Time: {:?}", load_end - load_start);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution: usize = stones
        .iter()
        .copied()
        .map(|stone| simulate_stone(stone, 25))
        .sum();

    let part_1_end = Instant::now();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_end - part_1_start);
    println!();

    let part_2_start = Instant::now();

    let part_2_solution: usize = stones
        .iter()
        .copied()
        .map(|stone| simulate_stone(stone, 75))
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
