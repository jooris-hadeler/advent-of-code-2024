use std::time::Instant;

fn load() -> Vec<Vec<char>> {
    let input = include_str!("input.txt");

    input
        .lines()
        .map(str::trim)
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect()
}

fn main() {
    let load_start = Instant::now();

    let grid = load();
    let height = grid.len();
    let width = grid[0].len();

    println!("{width} {height}");

    let load_end = Instant::now();

    println!("Loading:");
    println!("   Time: {:?}", load_end - load_start);
    println!();

    let part_1_start = Instant::now();

    let part_1_solution: usize = (0..height)
        .map(|y| {
            let can_be_vertical = y < height - 3;

            (0..width)
                .map(|x| {
                    let can_be_horizontal = x < width - 3;
                    let can_be_diagonal = can_be_horizontal && can_be_vertical;

                    let is_xmas = |pattern: [char; 4]| {
                        matches!(pattern, ['X', 'M', 'A', 'S'] | ['S', 'A', 'M', 'X'])
                    };

                    let mut count = 0;

                    // horizontal
                    if can_be_horizontal
                        && is_xmas([
                            grid[y + 0][x + 0],
                            grid[y + 0][x + 1],
                            grid[y + 0][x + 2],
                            grid[y + 0][x + 3],
                        ])
                    {
                        count += 1;
                    }

                    // vertical
                    if can_be_vertical
                        && is_xmas([
                            grid[y + 0][x + 0],
                            grid[y + 1][x + 0],
                            grid[y + 2][x + 0],
                            grid[y + 3][x + 0],
                        ])
                    {
                        count += 1;
                    }

                    // diagonal top left to bottom right
                    if can_be_diagonal
                        && is_xmas([
                            grid[y + 0][x + 0],
                            grid[y + 1][x + 1],
                            grid[y + 2][x + 2],
                            grid[y + 3][x + 3],
                        ])
                    {
                        count += 1;
                    }

                    // diagonal bottom left to top right
                    if can_be_diagonal
                        && is_xmas([
                            grid[y + 3][x + 0],
                            grid[y + 2][x + 1],
                            grid[y + 1][x + 2],
                            grid[y + 0][x + 3],
                        ])
                    {
                        count += 1;
                    }

                    count
                })
                .sum::<usize>()
        })
        .sum();

    let part_1_end = Instant::now();

    println!(" Part 1: {}", part_1_solution);
    println!("   Time: {:?}", part_1_end - part_1_start);
    println!();

    let part_2_start = Instant::now();

    let part_2_solution: usize = (0..height - 2)
        .map(|y| {
            (0..width - 2)
                .filter(|&x| {
                    let is_mas =
                        |pattern: [char; 3]| matches!(pattern, ['M', 'A', 'S'] | ['S', 'A', 'M']);

                    is_mas([grid[y + 0][x + 0], grid[y + 1][x + 1], grid[y + 2][x + 2]])
                        && is_mas([grid[y + 2][x + 0], grid[y + 1][x + 1], grid[y + 0][x + 2]])
                })
                .count()
        })
        .sum();

    let part_2_end = Instant::now();

    println!(" Part 2: {}", part_2_solution);
    println!("   Time: {:?}", part_2_end - part_2_start);
    println!();
}
