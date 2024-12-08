use std::collections::{HashMap, HashSet};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }
}

struct AntennaMap {
    antennas: HashMap<char, Vec<Point>>,
    width: i32,
    height: i32,
}

impl AntennaMap {
    fn new(input: &str) -> Self {
        let mut antennas = HashMap::new();
        let mut width = 0;
        let mut height = 0;

        for (y, line) in input.lines().enumerate() {
            width = line.len() as i32;
            for (x, ch) in line.chars().enumerate() {
                if ch != '.' {
                    antennas
                        .entry(ch)
                        .or_insert_with(Vec::new)
                        .push(Point::new(x as i32, y as i32));
                }
            }
            height = (y + 1) as i32;
        }

        println!("Loaded antenna map with dimensions {}x{}", width, height);
        for (freq, points) in &antennas {
            println!("Frequency {}: {:?}", freq, points);
        }

        Self {
            antennas,
            width,
            height,
        }
    }

    fn calculate_antinode(&self, a1: &Point, a2: &Point) -> Vec<Point> {
        let mut antinodes = Vec::new();

        // Calculate vector between points
        let dx = a2.x - a1.x;
        let dy = a2.y - a1.y;

        // Calculate both possible antinodes:
        // For a1: Move away from a2 by the same vector
        // This creates a point that's twice as far from a1 as a2 is
        let p1 = Point::new(a2.x + dx, a2.y + dy);

        // For a2: Move away from a1 by the same vector
        // This creates a point that's twice as far from a2 as a1 is
        let p2 = Point::new(a1.x - dx, a1.y - dy);

        for p in [p1, p2] {
            if p.x >= 0 && p.x < self.width && p.y >= 0 && p.y < self.height {
                antinodes.push(p);
            }
        }

        antinodes
    }

    fn find_antinodes(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::new();

        for (freq, positions) in &self.antennas {
            println!("\nProcessing frequency {}", freq);
            let mut count = 0;

            for i in 0..positions.len() {
                for j in i + 1..positions.len() {
                    let a1 = &positions[i];
                    let a2 = &positions[j];
                    println!("Checking antennas at {:?} and {:?}", a1, a2);

                    let new_antinodes = self.calculate_antinode(a1, a2);
                    for p in &new_antinodes {
                        println!("Found antinode at {:?}", p);
                    }
                    count += new_antinodes.len();
                    antinodes.extend(new_antinodes);
                }
            }
            println!("Found {} antinodes for frequency {}", count, freq);
        }

        antinodes
    }

    fn display(&self, antinodes: &HashSet<Point>) {
        let mut grid = vec![vec!['.'; self.width as usize]; self.height as usize];

        // Place antinodes
        for &point in antinodes {
            grid[point.y as usize][point.x as usize] = '#';
        }

        // Place antennas (overwriting antinodes if necessary)
        for (freq, points) in &self.antennas {
            for &point in points {
                grid[point.y as usize][point.x as usize] = *freq;
            }
        }

        println!("\nFinal map:");
        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let map = AntennaMap::new(input);
    let antinodes = map.find_antinodes();
    println!("\nFound {} antinodes", antinodes.len());
    map.display(&antinodes);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_case() {
        let input = "....a.....\n.....a....\n";
        let map = AntennaMap::new(input);
        let antinodes = map.find_antinodes();
        assert!(!antinodes.is_empty());
    }

    #[test]
    fn test_example_case() {
        let input = r#"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............"#;

        let map = AntennaMap::new(input);
        let antinodes = map.find_antinodes();
        assert_eq!(antinodes.len(), 14);
    }
}
