use std::collections::HashMap;

fn main() {
    // Read input
    let input = include_str!("../../input.txt");

    // Split input into rules and sections
    let parts: Vec<&str> = input.split("\n\n").collect();

    // Parse ordering rules into a HashMap
    let mut ordering_rules: HashMap<u32, Vec<u32>> = HashMap::new();

    parts[0].lines()
        .filter(|line| !line.is_empty())
        .for_each(|line| {
            let nums: Vec<u32> = line
                .split('|')
                .map(|n| n.parse().unwrap())
                .collect();

            // If we see X|Y, store X -> Y because X must come before Y
            ordering_rules
                .entry(nums[0])  // X
                .or_insert_with(Vec::new)
                .push(nums[1]);   // Y
        });

    // Parse sections
    let sections: Vec<Vec<u32>> = parts[1]
        .lines()
        .filter(|line| !line.is_empty())
        .map(|line| {
            line.split(',')
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect();

    // Store valid sections
    let mut valid_sections = Vec::new();

    for section in sections {
        let mut valid = true;

        // Check each pair in the section against the ordering rules
        'outer: for i in 0..section.len() {
            for j in i + 1..section.len() {
                let first = section[i];
                let second = section[j];

                // Check if there's a rule saying first must come before second
                if let Some(rules) = ordering_rules.get(&first) {
                    if rules.contains(&second) {
                        continue;
                    }
                }

                // Check if there's a rule saying second must come before first
                if let Some(rules) = ordering_rules.get(&second) {
                    if rules.contains(&first) {
                        valid = false;
                        break 'outer;
                    }
                }
            }
        }

        if valid {
            valid_sections.push(section);
        }
    }

    // // Print all valid sections
    // println!("Valid sections:");
    // for section in &valid_sections {
    //     println!(
    //         "{}",
    //         section
    //             .iter()
    //             .map(|n| n.to_string())
    //             .collect::<Vec<String>>()
    //             .join(",")
    //     );
    // }

    // Calculate total of middle page numbers
    let mut total_middle_page_numbers = 0;
    for section in &valid_sections {
        let middle_index = section.len() / 2;
        total_middle_page_numbers += section[middle_index];
    }
    println!("Total of middle page numbers: {}", total_middle_page_numbers);
}
