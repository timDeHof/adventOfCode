use nom::{
    bytes::complete::tag,
    character::complete::{digit1, multispace0, multispace1},
    combinator::map_res,
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

#[derive(Debug)]
struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

// Parse a single number
fn parse_number(input: &str) -> IResult<&str, u64> {
    map_res(digit1, str::parse)(input)
}

// Parse an equation line (e.g., "190: 10 19")
fn parse_equation(input: &str) -> IResult<&str, Equation> {
    let (input, target) =
        terminated(parse_number, tuple((multispace0, tag(":"), multispace0)))(input)?;
    let (input, numbers) = separated_list1(multispace1, parse_number)(input)?;
    Ok((input, Equation { target, numbers }))
}

// Evaluate expression strictly left to right
fn evaluate(numbers: &[u64], operators: &[char]) -> u64 {
    let mut result = numbers[0];
    for i in 0..operators.len() {
        match operators[i] {
            '+' => result += numbers[i + 1],
            '*' => result *= numbers[i + 1],
            _ => panic!("Invalid operator"),
        }
    }
    result
}

// Generate all possible operator combinations
fn generate_operator_combinations(len: usize) -> Vec<Vec<char>> {
    let mut combinations = Vec::new();
    let ops = ['+', '*'];

    fn generate(current: Vec<char>, len: usize, ops: &[char], combinations: &mut Vec<Vec<char>>) {
        if current.len() == len {
            combinations.push(current);
            return;
        }
        for &op in ops {
            let mut new_current = current.clone();
            new_current.push(op);
            generate(new_current, len, ops, combinations);
        }
    }

    generate(Vec::new(), len, &ops, &mut combinations);
    combinations
}

fn solve_equation(eq: &Equation) -> bool {
    let required_ops = eq.numbers.len() - 1;
    let operator_combinations = generate_operator_combinations(required_ops);

    for ops in operator_combinations {
        if evaluate(&eq.numbers, &ops) == eq.target {
            return true;
        }
    }
    false
}

fn main() {
    let input = include_str!("../../input.txt");

    let mut sum = 0;

    for line in input.lines() {
        if let Ok((_, equation)) = parse_equation(line) {
            if solve_equation(&equation) {
                println!(
                    "Valid equation: {} with numbers {:?}",
                    equation.target, equation.numbers
                );
                sum += equation.target;
            }
        }
    }

    println!("\nTotal calibration result: {}", sum);
}

#[test]
fn test_evaluation() {
    // Test case 1: 190 = 10 * 19
    assert_eq!(evaluate(&[10, 19], &['*']), 190);

    // Test case 2a: 3267 = 81 + 40 * 27
    assert_eq!(evaluate(&[81, 40, 27], &['+', '*']), 3267);

    // Test case 2b: 3267 = 81 * 40 + 27
    assert_eq!(evaluate(&[81, 40, 27], &['*', '+']), 3267);

    // Test case 3: 292 = 11 + 6 * 16 + 20
    assert_eq!(evaluate(&[11, 6, 16, 20], &['+', '*', '+']), 292);
}

#[test]
fn test_parser() {
    let test = "190: 10 19";
    let (_, equation) = parse_equation(test).unwrap();
    assert_eq!(equation.target, 190);
    assert_eq!(equation.numbers, vec![10, 19]);
}
