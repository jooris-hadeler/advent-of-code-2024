use std::{mem, thread, time::Instant};

fn load() -> Vec<(usize, Vec<usize>)> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (goal, numbers_str) = line.split_once(":").expect("failed to parse input");

            let goal = goal.parse::<usize>().expect("failed to parse number");

            let numbers = numbers_str
                .trim()
                .split(" ")
                .map(|num| num.parse::<usize>().expect("failed to parse number"))
                .collect();

            (goal, numbers)
        })
        .collect()
}

fn is_solvable(goal: usize, numbers: &Vec<usize>, use_concat_operator: bool) -> bool {
    let mut numbers = numbers.iter().copied().peekable();
    let mut queue = Vec::new();

    queue.push(numbers.next().expect("failed to unwrap first number"));

    while let Some(num) = numbers.next() {
        let old_queue = mem::take(&mut queue);
        let is_last = numbers.peek().is_none();

        for prev in old_queue {
            let add = prev + num;
            let mul = prev * num;

            if is_last && (add == goal || mul == goal) {
                return true;
            }

            if use_concat_operator {
                let num_digits = num.ilog10() + 1;
                let concat = prev * 10usize.pow(num_digits) + num;

                if is_last && concat == goal {
                    return true;
                }

                queue.push(concat);
            }

            queue.push(prev + num);
            queue.push(prev * num);
        }
    }

    false
}

fn main() {
    let load_start = Instant::now();

    let equations = load();

    // calculate chunk size for n threads
    let thread_count = 16;
    let chunk_size = match equations.len() % thread_count {
        0 => equations.len() / thread_count,
        left_over => (equations.len() + thread_count - left_over) / thread_count,
    };

    let load_end = Instant::now();

    println!("Loading:");
    println!("   Time: {:?}", load_end - load_start);
    println!();

    let part_1_start = Instant::now();

    let handles: Vec<_> = equations
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.into_iter().cloned().collect::<Vec<_>>();

            thread::spawn(move || {
                chunk
                    .iter()
                    .filter(|(goal, numbers)| is_solvable(*goal, numbers, false))
                    .map(|(goal, _)| goal)
                    .sum::<usize>()
            })
        })
        .collect();

    let part_1_sum: usize = handles
        .into_iter()
        .map(|handle| handle.join().expect("failed to join handle"))
        .sum();

    let part_1_end = Instant::now();

    println!(" Part 1: {}", part_1_sum);
    println!("   Time: {:?}", part_1_end - part_1_start);
    println!();

    let part_2_start = Instant::now();

    let handles: Vec<_> = equations
        .chunks(chunk_size)
        .map(|chunk| {
            let chunk = chunk.into_iter().cloned().collect::<Vec<_>>();

            thread::spawn(move || {
                chunk
                    .iter()
                    .filter(|(goal, numbers)| is_solvable(*goal, numbers, true))
                    .map(|(goal, _)| goal)
                    .sum::<usize>()
            })
        })
        .collect();

    let part_2_sum: usize = handles
        .into_iter()
        .map(|handle| handle.join().expect("failed to join handle"))
        .sum();

    let part_2_end = Instant::now();

    println!(" Part 2: {}", part_2_sum);
    println!("   Time: {:?}", part_2_end - part_2_start);
    println!();

    println!(
        "  Total: {:?}",
        (part_2_end - part_2_start) + (part_1_end - part_1_start) + (load_end - load_start)
    );
}
