fn main() {
    let input = include_str!("../../input.txt");

    let reports = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|num| num.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    println!("{:?}", reports);

    let mut safe_reports = 0; // Declare as mutable and add semicolon

    for report in reports {
        println!("{:?}", report);
        let mut is_safe = true;
        if report.len() < 2 {
            continue; // Skip lines with fewer than 2 numbers
        }
        let is_increasing = report[0] < report[1];
        for i in 0..report.len() - 1 {
            let diff = (report[i] - report[i + 1]).abs();

            if diff < 1 || diff > 3 {
                // Use correct operator
                is_safe = false;
                break;
            }
            if (report[i] < report[i + 1]) != is_increasing {
                is_safe = false;
                break;
            }
        }

        if is_safe {
            safe_reports += 1;
        }
    }

    println!("Number of safe reports: {}", safe_reports); // Print the result
}
