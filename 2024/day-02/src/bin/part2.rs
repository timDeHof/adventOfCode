fn main() {
    let input = include_str!("../../input.txt");
//     let input = "7 6 4 2 1
// 1 2 7 8 9
// 9 7 6 2 1
// 1 3 2 4 5
// 8 6 4 4 1
// 1 3 6 7 9";

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
        let check_report = |report: &Vec<i32>| -> bool {
            let is_increasing = report[0] < report[1];
            for i in 0..report.len() - 1 {
                let diff = (report[i] - report[i + 1]).abs();

                if diff < 1 || diff > 3 {
                    // Use correct operator
                    return false;
                }
                if (report[i] < report[i + 1]) != is_increasing {
                    return false;
                }
            }
            true
        };
        // Check original report
        if !check_report(&report) {
            // Check by removing one element at a time
            is_safe = false;
            for i in 0..report.len() {
                let mut modified_report = report.clone();
                modified_report.remove(i);
                if check_report(&modified_report) {
                    is_safe = true;
                    break;
                }
            }
        }
        if is_safe {
            safe_reports += 1;
        }
    }

    println!("Number of safe reports: {}", safe_reports); // Print the result
}
