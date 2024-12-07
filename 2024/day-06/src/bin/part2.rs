use std::collections::HashSet;

fn main() {
    let input = include_str!("../../input.txt");
    let grid: Vec<Vec<char>> = input
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| line.chars().collect())
        .collect();

    let (guard_row, guard_col) = find_guard_position(&grid).expect("Guard not found in grid");
    let mut loop_stuck_positions = HashSet::new();

    // Check for every empty position in the grid
    for row in 0..grid.len() {
        for col in 0..grid[0].len() {
            if grid[row][col] == '.' && !(row == guard_row && col == guard_col) {
                // Temporarily place an obstruction
                let mut temp_grid = grid.clone();
                temp_grid[row][col] = '#';

                // Check if this causes a loop
                if causes_loop(&temp_grid, guard_row, guard_col) {
                    loop_stuck_positions.insert((row, col));
                }
            }
        }
    }

    println!("Positions that can cause a loop: {:?}", loop_stuck_positions);
    println!("Count of different obstruction positions causing loop: {}", loop_stuck_positions.len());
}

// A function to determine if the guard gets stuck in a loop given the current grid
fn causes_loop(grid: &[Vec<char>], start_row: usize, start_col: usize) -> bool {
    let mut visited = HashSet::new();
    let mut row = start_row;
    let mut col = start_col;
    let directions = ['^', '>', 'v', '<'];
    let mut direction_index = 0; // Start facing '^'

    loop {
        let pos = (row, col, directions[direction_index]);
        if visited.contains(&pos) {
            return true; // Found a loop
        }
        visited.insert(pos);

        if is_at_edge(grid, row, col, directions[direction_index]) {
            return false; // Exited the grid
        }

        if can_move_forward(grid, row, col, directions[direction_index]) {
            (row, col) = get_next_position(row, col, directions[direction_index]);
        } else {
            direction_index = (direction_index + 1) % 4; // Turn right
        }
    }
}

fn find_guard_position(grid: &[Vec<char>]) -> Option<(usize, usize)> {
    for (row, line) in grid.iter().enumerate() {
        if let Some(col) = line.iter().position(|&c| c == '^') {
            return Some((row, col));
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
