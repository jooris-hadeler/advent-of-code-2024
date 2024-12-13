use std::time::Instant;

fn load() -> () {
    let input = include_str!("input.txt");

    todo!("Implement Loading");
}

fn main() {
    let load_start = Instant::now();

    let _ = load();

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution: usize = todo!("Implement Part 1");

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    let part_2_start = Instant::now();

    let part_2_solution: usize = todo!("Implement Part 2");

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
