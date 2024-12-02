
fn main() {
    let input = include_str!("../../input1.txt");
      let pairs: Vec<(usize, usize)> = input
        .lines()
        .filter_map(|line| {
            let nums: Vec<usize> = line.split_whitespace().filter_map(|s| s.parse().ok()).collect();
            if nums.len() == 2 {
                Some((nums[0], nums[1]))
            } else {
                None
            }
        })
        .collect();
       let left_numbers: Vec<usize>  = pairs.iter().map(|(a, _)| *a).collect();
       let right_numbers: Vec<usize> = pairs.iter().map(|(_, b)| *b).collect();

       let result: usize = left_numbers.iter().map(|&number|{
             number * right_numbers.iter().filter(|&&number2| number == number2).count()
       }).sum();


       println!("{:?}", result);

}