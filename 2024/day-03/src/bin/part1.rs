use regex::Regex;

fn main() {
    let input = include_str!("../../input.txt");
    // let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut total_sum = 0;

    for caps in re.captures_iter(&input) {
        let x: i32 = caps[1].parse().unwrap();
        let y: i32 = caps[2].parse().unwrap();
        total_sum += x * y;
    }

    println!("{}", total_sum); // Output the total sum
}
