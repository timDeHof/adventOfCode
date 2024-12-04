

fn parse_grid(content: &str) -> Vec<Vec<char>> {
    content.lines()
        .map(|line| line.trim().chars().collect())
        .collect()
}

fn find_xmas(grid: &[Vec<char>]) -> Vec<(usize, usize, &'static str)> {
    let height = grid.len();
    let width = grid[0].len();
    let mut locations = Vec::new();

    // Eight possible directions
    let directions = [
        (0, 1, "right"),   // →
        (1, 0, "down"),    // ↓
        (1, 1, "down-right"), // ↘
        (-1, 1, "up-right"),  // ↗
        (0, -1, "left"),   // ←
        (-1, 0, "up"),     // ↑
        (-1, -1, "up-left"), // ↖
        (1, -1, "down-left")  // ↙
    ];

    for row in 0..height {
        for col in 0..width {
            for &(dy, dx, direction) in &directions {
                let mut valid = true;
                let word = "XMAS";

                for (i, expected_char) in word.chars().enumerate() {
                    let new_row = row as i32 + dy * i as i32;
                    let new_col = col as i32 + dx * i as i32;

                    if new_row < 0 || new_row >= height as i32 ||
                       new_col < 0 || new_col >= width as i32 || grid[new_row as usize][new_col as usize] != expected_char {
                        valid = false;
                        break;
                    }


                }

                if valid {
                    locations.push((row, col, direction));
                }
            }
        }
    }

    locations
}

fn main() {
    let content = include_str!("../../input.txt");
    let grid = parse_grid(&content);

    let locations = find_xmas(&grid);
    println!("Found {} XMAS occurrences:", locations.len());

    // for (row, col, direction) in locations {
    //     println!("XMAS starts at ({}, {}) going {}", row, col, direction);
    // }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_xmas() {
        let content = include_str!("../../example.txt");
        let grid = parse_grid(&content);

        let locations = find_xmas(&grid);
        assert!(locations.len() > 0, "Should find at least one XMAS");
        assert_eq!(locations.len(), 18, "Should find 18 XMAS");
    }
}