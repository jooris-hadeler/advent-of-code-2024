use std::time::Instant;

#[derive(Debug)]
enum Command {
    Do,
    Dont,
    Mul(usize, usize),
}

fn load() -> Vec<Command> {
    let input = include_str!("input.txt");

    let mut chars = input.chars().peekable();

    macro_rules! peek {
        () => {
            chars.peek().copied()
        };
    }

    macro_rules! next {
        () => {
            chars.next()
        };
    }

    macro_rules! expect {
        ($expected_ch:expr) => {
            if peek!().is_some_and(|nch| nch == $expected_ch) {
                next!();
                true
            } else {
                false
            }
        };
    }

    let mut commands = Vec::new();

    while let Some(ch) = next!() {
        match ch {
            // check fo mul
            'm' => {
                if !expect!('u') || !expect!('l') || !expect!('(') {
                    continue;
                }

                let mut first_number = String::new();

                while peek!().is_some_and(|ch| ch.is_ascii_digit()) {
                    first_number.push(next!().unwrap());
                }

                if first_number.is_empty() || !expect!(',') {
                    continue;
                }

                let mut second_number = String::new();

                while peek!().is_some_and(|ch| ch.is_ascii_digit()) {
                    second_number.push(next!().unwrap());
                }

                if second_number.is_empty() || !expect!(')') {
                    continue;
                }

                let left = first_number.parse().expect("failed to parse number");
                let right = second_number.parse().expect("failed to parse number");

                commands.push(Command::Mul(left, right));
            }
            // check fo do or don't
            'd' => {
                if !expect!('o') {
                    continue;
                }

                if expect!('(') {
                    if !expect!(')') {
                        continue;
                    }
                    commands.push(Command::Do);
                    continue;
                }

                if !expect!('n') || !expect!('\'') || !expect!('t') {
                    continue;
                }

                if expect!('(') {
                    if !expect!(')') {
                        continue;
                    }
                    commands.push(Command::Dont);
                    continue;
                }
            }
            _ => (),
        }
    }

    commands
}

fn main() {
    let load_start = Instant::now();

    let commands = load();

    let load_elapsed = load_start.elapsed();

    println!("Loading:");
    println!("   Time: {:?}", load_elapsed);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution: usize = commands
        .iter()
        .map(|cmd| match cmd {
            &Command::Mul(l, r) => l * r,
            _ => 0,
        })
        .sum();

    let part_1_elapsed = part_1_start.elapsed();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_elapsed);
    println!();

    let part_2_start = Instant::now();

    let mut active = true;
    let part_2_solution: usize = commands
        .iter()
        .filter(|cmd| {
            active = match cmd {
                Command::Do => true,
                Command::Dont => false,
                _ => active,
            };

            active
        })
        .map(|cmd| match cmd {
            &Command::Mul(l, r) => l * r,
            _ => 0,
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
