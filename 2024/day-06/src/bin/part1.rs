use std::collections::HashSet;
fn main() {
    // Read input

    let input = include_str!("../../example.txt");
    let mut grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let (mut row, mut col) = find_guard_position(&grid).expect("Guard not found in grid");
    let mut direction = '^';

    println!("Initial guard position: ({}, {})", row, col);
    //print_grid(&grid);
    let mut turns = 0;
    let mut visited_positions = HashSet::new();
    visited_positions.insert((row, col)); // Mark initial position as visited
    while !is_at_edge(&grid, row, col, direction) {
        if can_move_forward(&grid, row, col, direction) {
            // Move forward
            grid[row][col] = '.';
            (row, col) = get_next_position(row, col, direction);
            grid[row][col] = direction;
            turns += 1; // Only increment turns when moving forward
            visited_positions.insert((row, col)); // Add new position to visited
            println!(
                "\nMoving {} to: ({}, {})",
                direction_name(direction),
                row,
                col
            );
        } else {
            // Turn right
            direction = turn_right(direction);
            grid[row][col] = direction;
            println!("\nTurning right at: ({}, {})", row, col);
        }
        // print_grid(&grid);
        println!("turns: {}", turns);
    }
    // Count of unique positions visited
    println!(
        "The guard visited {} unique positions.",
        visited_positions.len()
    );
    // println!("\nGuard is at the edge after {} turns.", turns);
    // Clear final position when leaving grid
    grid[row][col] = '.';
    println!("\nGuard moves off the grid at: ({}, {})", row, col);
    // print_grid(&grid);
}

fn find_guard_position(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (row, line) in grid.iter().enumerate() {
        for (col, &c) in line.iter().enumerate() {
            if c == '^' {
                return Some((row, col));
            }
        }
    }
    None
}

fn is_at_edge(grid: &[Vec<char>], row: usize, col: usize, direction: char) -> bool {
    match direction {
        '^' => row == 0,
        '>' => col == grid[0].len() - 1,
        'v' => row == grid.len() - 1,
        '<' => col == 0,
        _ => unreachable!(),
    }
}

fn can_move_forward(grid: &[Vec<char>], row: usize, col: usize, direction: char) -> bool {
    let (next_row, next_col) = get_next_position(row, col, direction);
    next_row < grid.len() && next_col < grid[0].len() && grid[next_row][next_col] != '#'
}

fn get_next_position(row: usize, col: usize, direction: char) -> (usize, usize) {
    match direction {
        '^' => (row - 1, col),
        '>' => (row, col + 1),
        'v' => (row + 1, col),
        '<' => (row, col - 1),
        _ => unreachable!(),
    }
}

fn turn_right(direction: char) -> char {
    match direction {
        '^' => '>',
        '>' => 'v',
        'v' => '<',
        '<' => '^',
        _ => unreachable!(),
    }
}

fn direction_name(direction: char) -> &'static str {
    match direction {
        '^' => "up",
        '>' => "right",
        'v' => "down",
        '<' => "left",
        _ => unreachable!(),
    }
}

fn print_grid(grid: &[Vec<char>]) {
    for row in grid {
        println!("{}", row.iter().collect::<String>());
    }
}
