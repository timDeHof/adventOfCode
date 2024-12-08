use std::collections::{HashMap, HashSet};
use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    #[inline(always)]
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

        // Pre-allocate vectors for better performance
        let lines: Vec<&str> = input.lines().collect();
        height = lines.len() as i32;
        if height > 0 {
            width = lines[0].len() as i32;
        }

        for (y, &line) in lines.iter().enumerate() {
            for (x, ch) in line.bytes().enumerate() {
                if ch != b'.' {
                    antennas.entry(ch as char)
                        .or_insert_with(|| Vec::with_capacity(4))
                        .push(Point::new(x as i32, y as i32));
                }
            }
        }

        Self {
            antennas,
            width,
            height,
        }
    }

    // Fast GCD calculation for line calculations
    #[inline(always)]
    fn gcd(mut a: i32, mut b: i32) -> i32 {
        a = a.abs();
        b = b.abs();
        while b != 0 {
            let t = b;
            b = a % b;
            a = t;
        }
        a
    }

    // Efficiently find all points on a line between two points
    #[inline(always)]
    fn line_points(&self, a1: &Point, a2: &Point) -> Vec<Point> {
        let mut points = Vec::new();

        let dx = a2.x - a1.x;
        let dy = a2.y - a1.y;

        if dx == 0 && dy == 0 {
            points.push(*a1);
            return points;
        }

        // Calculate the GCD to find the smallest step
        let step = Self::gcd(dx, dy);
        let step_x = dx / step;
        let step_y = dy / step;

        // Start from the leftmost/topmost point
        let (start, end) = if a1.x < a2.x || (a1.x == a2.x && a1.y < a2.y) {
            (*a1, *a2)
        } else {
            (*a2, *a1)
        };

        // Extend line in both directions
        let mut x = start.x;
        let mut y = start.y;

        // Extend backward
        while x >= 0 && y >= 0 && x < self.width && y < self.height {
            points.push(Point::new(x, y));
            x -= step_x;
            y -= step_y;
        }

        x = start.x + step_x;
        y = start.y + step_y;

        // Extend forward
        while x < self.width && y < self.height && x >= 0 && y >= 0 {
            points.push(Point::new(x, y));
            x += step_x;
            y += step_y;
        }

        points
    }

    fn find_antinodes(&self) -> HashSet<Point> {
        let mut antinodes = HashSet::with_capacity((self.width * self.height) as usize / 4);

        for (_, positions) in self.antennas.iter() {
            // Only process if there's more than one antenna of this frequency
            if positions.len() > 1 {
                // Pre-calculate all pairs to avoid nested loop
                let pairs: Vec<_> = (0..positions.len())
                    .flat_map(|i| ((i + 1)..positions.len()).map(move |j| (i, j)))
                    .collect();

                // Process all pairs in parallel if available
                for (i, j) in pairs {
                    let points = self.line_points(&positions[i], &positions[j]);
                    antinodes.extend(points);
                }
            }
        }

        antinodes
    }
}

fn main() {
    let input = include_str!("../../input.txt");

    let start = Instant::now();
    let map = AntennaMap::new(input);
    let antinodes = map.find_antinodes();
    let duration = start.elapsed();

    println!("Found {} antinodes in {:?}", antinodes.len(), duration);

    // Only generate visualization if needed
    #[cfg(debug_assertions)]
    {
        let mut grid = vec![vec!['.'; map.width as usize]; map.height as usize];
        for &point in &antinodes {
            grid[point.y as usize][point.x as usize] = '#';
        }
        for (freq, points) in &map.antennas {
            for &point in points {
                grid[point.y as usize][point.x as usize] = *freq;
            }
        }
        for row in grid {
            println!("{}", row.iter().collect::<String>());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_line_points() {
        let map = AntennaMap::new(".....\n.....\n");
        let p1 = Point::new(0, 0);
        let p2 = Point::new(2, 2);
        let points = map.line_points(&p1, &p2);
        assert!(points.contains(&Point::new(0, 0)));
        assert!(points.contains(&Point::new(1, 1)));
        assert!(points.contains(&Point::new(2, 2)));
    }

    #[test]
    fn test_example() {
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
        assert_eq!(antinodes.len(), 34);
    }
}