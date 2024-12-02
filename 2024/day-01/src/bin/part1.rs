fn main() {

    let input = include_str!("../../input1.txt");

    let sorted_input = sort(input);

    println!("{:?}", sorted_input);
}

fn sort(input: &str) -> Result<Vec<Vec<i32>>, Box<dyn std::error::Error>> {
    let pairs: Vec<(i32, i32)> = input
        .lines()
        .filter_map(|line| {
            let nums: Vec<i32> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
            if nums.len() == 2 {
                Some((nums[0], nums[1]))
            } else {
                None
            }
        })
        .collect();

    let mut left_numbers: Vec<i32> = pairs.iter().map(|(a, _)| *a).collect();
    let mut right_numbers: Vec<i32> = pairs.iter().map(|(_, b)| *b).collect();

     // Sort the left numbers
     left_numbers.sort();
     right_numbers.sort();

    // Create a new vector of pairs by zipping the two sorted vectors
    let result: i32 = std::iter::zip(left_numbers, right_numbers)
        .map(|(a, b)| (a -b).abs())
        .sum();

    Ok(vec![vec![result]])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_functionality() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = sort(input).unwrap();
        assert_eq!(result, vec![vec![11]]);
    }

    #[test]
    fn test_empty_input() {
        let input = "";
        let result = sort(input).unwrap();
        assert_eq!(result, vec![vec![0]]);
    }

}
