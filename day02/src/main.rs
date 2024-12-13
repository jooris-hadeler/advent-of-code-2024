use std::time::Instant;

fn load() -> Vec<Vec<usize>> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(" ")
                .map(|num| num.parse().expect("failed to parse number"))
                .collect()
        })
        .collect()
}

fn main() {
    let load_start = Instant::now();

    let reports = load();

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution = reports
        .iter()
        .filter(|report| {
            let all_increasing = report.windows(2).all(|wnd| wnd[0] < wnd[1]);
            let all_decreasing = report.windows(2).all(|wnd| wnd[0] > wnd[1]);
            let all_valid_diff = report
                .windows(2)
                .all(|wnd| matches!(wnd[0].abs_diff(wnd[1]), 1..=3));

            (all_increasing || all_decreasing) && all_valid_diff
        })
        .count();

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    let part_2_start = Instant::now();

    let part_2_solution = reports
        .iter()
        .filter(|report| {
            let mut report_with_one_dropped = Vec::<usize>::new();

            (0..report.len()).any(|idx| {
                report_with_one_dropped.clone_from(report);
                report_with_one_dropped.remove(idx);

                let all_increasing = report_with_one_dropped
                    .windows(2)
                    .all(|wnd| wnd[0] < wnd[1]);
                let all_decreasing = report_with_one_dropped
                    .windows(2)
                    .all(|wnd| wnd[0] > wnd[1]);
                let all_valid_diff = report_with_one_dropped
                    .windows(2)
                    .all(|wnd| matches!(wnd[0].abs_diff(wnd[1]), 1..=3));

                report_with_one_dropped.clear();

                (all_increasing || all_decreasing) && all_valid_diff
            })
        })
        .count();

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
