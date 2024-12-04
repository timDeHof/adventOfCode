use std::error::Error;
use std::fs::read_to_string;

#[derive(Debug)]
struct PatternStats {
    count: usize,
    top_ms: usize,    // M's at top, S's at bottom
    bottom_ms: usize, // M's at bottom, S's at top
    left_ms: usize,   // M's at left, S's at right
    right_ms: usize,  // M's at right, S's at left
}

fn find_x_shaped_mas(grid: &[Vec<char>]) -> (Vec<(usize, usize, &'static str)>, PatternStats) {
    let height = grid.len();
    let width = grid[0].len();
    let mut locations = Vec::with_capacity((height - 2) * (width - 2));
    let mut stats = PatternStats { count: 0, top_ms: 0, bottom_ms: 0, left_ms: 0, right_ms: 0 };

    // Pre-calculate grid boundaries
    let row_range = 1..height-1;
    let col_range = 1..width-1;

    for row in row_range {
        for col in col_range.clone() {
            if grid[row][col] != 'A' {
                continue;
            }

            // Check each pattern using array indexing instead of multiple conditionals
            let patterns = [
                ([-1,-1, -1,1, 1,-1, 1,1], grid[row-1][col-1] == 'M' && grid[row-1][col+1] == 'M' &&
                                          grid[row+1][col-1] == 'S' && grid[row+1][col+1] == 'S', "top-ms"),
                ([-1,-1, -1,1, 1,-1, 1,1], grid[row+1][col-1] == 'M' && grid[row+1][col+1] == 'M' &&
                                          grid[row-1][col-1] == 'S' && grid[row-1][col+1] == 'S', "bottom-ms"),
                ([-1,-1, 1,-1, -1,1, 1,1], grid[row-1][col-1] == 'M' && grid[row+1][col-1] == 'M' &&
                                          grid[row-1][col+1] == 'S' && grid[row+1][col+1] == 'S', "left-ms"),
                ([-1,1, 1,1, -1,-1, 1,-1], grid[row-1][col+1] == 'M' && grid[row+1][col+1] == 'M' &&
                                          grid[row-1][col-1] == 'S' && grid[row+1][col-1] == 'S', "right-ms")
            ];

            for (_, valid, pattern_type) in patterns.iter() {
                if *valid {
                    locations.push((row, col, *pattern_type));
                    stats.count += 1;
                    match *pattern_type {
                        "top-ms" => stats.top_ms += 1,
                        "bottom-ms" => stats.bottom_ms += 1,
                        "left-ms" => stats.left_ms += 1,
                        "right-ms" => stats.right_ms += 1,
                        _ => {}
                    }
                }
            }
        }
    }
    (locations, stats)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = read_to_string("input.txt")?;
    let grid: Vec<Vec<char>> = content
        .lines()
        .map(|line| line.trim().chars().collect())
        .collect();

    let (locations, stats) = find_x_shaped_mas(&grid);
    println!("Found {} X-shaped MAS patterns:", locations.len());

    for (row, col, pattern_type) in &locations {
        println!("\nPattern centered at ({}, {}) - Type: {}", row, col, pattern_type);
        for r in *row-1..=row+1 {
            for c in *col-1..=col+1 {
                print!("{}", grid[r][c]);
            }
            println!();
        }
    }

    println!("\nPattern Statistics:");
    println!("Total patterns: {}", stats.count);
    println!("  M's at top: {}", stats.top_ms);
    println!("  M's at bottom: {}", stats.bottom_ms);
    println!("  M's at left: {}", stats.left_ms);
    println!("  M's at right: {}", stats.right_ms);

    Ok(())
}