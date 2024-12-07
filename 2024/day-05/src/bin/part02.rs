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

    // Store invalid sections
    let mut invalid_sections = Vec::new();

    'section_loop: for section in sections {
        // Check each pair in the section against the ordering rules
        for i in 0..section.len() {
            for j in i + 1..section.len() {
                let first = section[i];
                let second = section[j];

                // Check if there's a rule saying second must come before first
                if let Some(rules) = ordering_rules.get(&second) {
                    if rules.contains(&first) {
                        invalid_sections.push(section.clone());
                        continue 'section_loop;
                    }
                }
            }
        }
    }

    // Function to check if x should come before y based on rules
    fn should_come_before(x: u32, y: u32, rules: &HashMap<u32, Vec<u32>>) -> bool {
        if let Some(after_x) = rules.get(&x) {
            if after_x.contains(&y) {
                return true;
            }
        }
        if let Some(after_y) = rules.get(&y) {
            if after_y.contains(&x) {
                return false;
            }
        }
        x < y // Default to numerical order if no rules apply
    }

    // Reorder invalid sections
    let mut reordered_sections = Vec::new();
    for mut section in invalid_sections {
        section.sort_by(|a, b| {
            if should_come_before(*a, *b, &ordering_rules) {
                std::cmp::Ordering::Less
            } else {
                std::cmp::Ordering::Greater
            }
        });
        reordered_sections.push(section);
    }

    // Calculate total of middle page numbers from reordered sections
    let mut total_middle_page_numbers = 0;
    for section in &reordered_sections {
        let middle_index = section.len() / 2;
        total_middle_page_numbers += section[middle_index];
    }
    println!("Total of middle page numbers from reordered sections: {}", total_middle_page_numbers);
}
