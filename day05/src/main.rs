use std::{cmp::Ordering, time::Instant};

fn load() -> (Vec<(usize, usize)>, Vec<Vec<usize>>) {
    let input = include_str!("input.txt");

    let empty_line_index = input
        .lines()
        .map(str::trim)
        .position(|line| line.is_empty())
        .expect("failed to locate empty line");

    let rules = input
        .lines()
        .map(str::trim)
        .take(empty_line_index)
        .map(|line| {
            let (first, second) = line.split_once("|").expect("failed splitting rule");

            (
                first.parse().expect("failed parsing integer"),
                second.parse().expect("failed parsing integer"),
            )
        })
        .collect();

    let updates = input
        .lines()
        .map(str::trim)
        .skip(empty_line_index)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(",")
                .map(|num| num.parse().expect("failed to parse integer"))
                .collect()
        })
        .collect();

    (rules, updates)
}

fn check_rule(rule: (usize, usize), update: &Vec<usize>) -> bool {
    let (first, second) = rule;

    let Some(first_pos) = update.iter().position(|&num| num == first) else {
        return true;
    };

    let Some(second_pos) = update.iter().position(|&num| num == second) else {
        return true;
    };

    first_pos < second_pos
}

fn main() {
    let load_start = Instant::now();

    let (rules, updates) = load();

    let load_end = Instant::now();

    println!("  Loading:");
    println!("     Time: {:?}", load_end - load_start);
    println!();

    let partition_start = Instant::now();

    let (valid_updates, mut invalid_updates): (Vec<_>, Vec<_>) = updates
        .into_iter()
        .partition(|update| rules.iter().all(|&rule| check_rule(rule, update)));

    let partition_end = Instant::now();

    println!("Partition:");
    println!("     Time: {:?}", partition_end - partition_start);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution: usize = valid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    let part_1_end = Instant::now();

    println!("   Part 1: {}", part_1_solution);
    println!("     Time: {:?}", part_1_end - part_1_start);
    println!();

    let part_2_start = Instant::now();

    for update in &mut invalid_updates {
        update.sort_by(|&a, &b| {
            rules
                .iter()
                .find_map(|&(f, s)| match (a == f, b == s) {
                    (true, true) => Some(Ordering::Less), // if `a == f` and `b == s`, then `a` must come before `b` (`Ordering::Less`)
                    (false, false) => match (a == s, b == f) {
                        (true, true) => Some(Ordering::Greater), // if `a == s` and `b == f`, then `b` must come before `a` (`Ordering::Greater`)
                        _ => None,                               // continue checking other rules
                    },
                    _ => None, // continue checking other rules
                })
                .unwrap_or(Ordering::Equal) // default to `Ordering::Equal` if no violation.
        });
    }

    let part_2_solution: usize = invalid_updates
        .iter()
        .map(|update| update[update.len() / 2])
        .sum();

    let part_2_end = Instant::now();

    println!("   Part 2: {}", part_2_solution);
    println!("     Time: {:?}", part_2_end - part_2_start);
    println!();

    println!("    Total:");
    println!(
        "     Time: {:?}",
        (load_end - load_start) + (part_1_end - part_1_start) + (part_2_end - part_2_start)
    );
}
