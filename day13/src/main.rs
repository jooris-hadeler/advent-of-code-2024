use std::{time::Instant, usize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Coords(pub isize, pub isize);

#[derive(Debug, Clone, Copy)]
pub struct ClawMachine {
    button_a: Coords,
    button_b: Coords,
    prize: Coords,
}

fn load() -> Vec<ClawMachine> {
    let input = include_str!("input.txt");

    let mut current_a = None;
    let mut current_b = None;

    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| {
            let (pre_colon, post_colon) = line
                .split_once(": ")
                .expect("failed to split string at colon");

            let strip_and_parse_int = |prefix: &str, input: &str| {
                input
                    .strip_prefix(prefix)
                    .expect("failed to strip prefix")
                    .parse::<isize>()
                    .expect("failed to parse usize")
            };

            match pre_colon {
                "Button A" => {
                    let (x, y) = post_colon
                        .split_once(", ")
                        .expect("failed to split string at comma");

                    current_a = Some(Coords(
                        strip_and_parse_int("X+", x),
                        strip_and_parse_int("Y+", y),
                    ));

                    None
                }
                "Button B" => {
                    let (x, y) = post_colon
                        .split_once(", ")
                        .expect("failed to split string at comma");

                    current_b = Some(Coords(
                        strip_and_parse_int("X+", x),
                        strip_and_parse_int("Y+", y),
                    ));

                    None
                }
                "Prize" => {
                    let (x, y) = post_colon
                        .split_once(", ")
                        .expect("failed to split string at comma");

                    let prize = Coords(strip_and_parse_int("X=", x), strip_and_parse_int("Y=", y));

                    let button_a = current_a.take().unwrap();
                    let button_b = current_b.take().unwrap();

                    Some(ClawMachine {
                        button_a,
                        button_b,
                        prize,
                    })
                }
                _ => panic!("invalid input"),
            }
        })
        .filter(Option::is_some)
        .map(Option::unwrap)
        .collect()
}

fn score_claw_machine(machine: &ClawMachine) -> usize {
    let Coords(goal_x, goal_y) = machine.prize;
    let Coords(ax, ay) = machine.button_a;
    let Coords(bx, by) = machine.button_b;

    let determinante = ax * by - ay * bx;
    if determinante == 0 {
        return 0;
    }

    let number_a = goal_x * by - goal_y * bx;
    let number_b = goal_y * ax - goal_x * ay;

    if number_a % determinante != 0 || number_b % determinante != 0 {
        return 0;
    }

    let presses_a = number_a / determinante;
    let presses_b = number_b / determinante;

    if presses_a >= 0 && presses_b >= 0 {
        3 * presses_a as usize + presses_b as usize
    } else {
        0
    }
}

fn main() {
    let load_start = Instant::now();

    let mut claw_machines = load();

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution: usize = claw_machines.iter().map(score_claw_machine).sum();

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    let part_2_start = Instant::now();

    claw_machines.iter_mut().for_each(|machine| {
        machine.prize.0 += 10000000000000;
        machine.prize.1 += 10000000000000;
    });

    let part_2_solution: usize = claw_machines.iter().map(score_claw_machine).sum();

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
