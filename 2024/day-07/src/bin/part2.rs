use std::time::Instant;

#[derive(Debug)]
struct Equation {
    target: u64,
    numbers: Vec<u64>,
}

// Using u8 for compact operator representation
type Operator = u8;
const ADD: Operator = 0;
const MUL: Operator = 1;
const CONCAT: Operator = 2;

// Fast integer concatenation without string conversion
#[inline(always)]
fn concat_numbers(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a * 10;
    }
    // Calculate number of digits in b
    let b_digits = (b as f64).log10().floor() as u32 + 1;
    a * 10_u64.pow(b_digits) + b
}

// Evaluate expression strictly left to right using optimized operations
#[inline(always)]
fn evaluate(numbers: &[u64], operators: &[Operator]) -> Option<u64> {
    let mut result = numbers[0];

    for i in 0..operators.len() {
        match operators[i] {
            ADD => {
                result = result.checked_add(numbers[i + 1])?;
            }
            MUL => {
                result = result.checked_mul(numbers[i + 1])?;
            }
            CONCAT => {
                result = concat_numbers(result, numbers[i + 1]);
            }
            _ => return None,
        }
    }
    Some(result)
}

// Generate operator combinations using bit manipulation
fn generate_operator_combinations(len: usize) -> impl Iterator<Item = Vec<Operator>> {
    let total_combinations = 3_u32.pow(len as u32);

    (0..total_combinations).map(move |n| {
        let mut ops = Vec::with_capacity(len);
        let mut num = n;
        for _ in 0..len {
            ops.push((num % 3) as u8);
            num /= 3;
        }
        ops
    })
}

// Parse numbers quickly without nom
fn parse_equation(line: &str) -> Option<Equation> {
    let mut parts = line.split(':');
    let target = parts.next()?.trim().parse().ok()?;

    let numbers = parts
        .next()?
        .split_whitespace()
        .map(|n| n.parse().ok())
        .collect::<Option<Vec<_>>>()?;

    Some(Equation { target, numbers })
}

fn solve_equations(input: &str) -> (u64, Vec<(String, u64)>) {
    let mut sum = 0u64;
    let mut solutions = Vec::new();

    for line in input.lines() {
        if let Some(equation) = parse_equation(line) {
            let ops_needed = equation.numbers.len() - 1;

            // Try all operator combinations
            for ops in generate_operator_combinations(ops_needed) {
                if let Some(result) = evaluate(&equation.numbers, &ops) {
                    if result == equation.target {
                        // Format solution only if we found a match
                        let formatted = format_solution(&equation.numbers, &ops, equation.target);
                        solutions.push((formatted, equation.target));
                        sum = sum.checked_add(equation.target).unwrap_or(sum);
                        break; // Found a solution, no need to try more combinations
                    }
                }
            }
        }
    }

    (sum, solutions)
}

// Format solution only when needed
fn format_solution(numbers: &[u64], operators: &[Operator], target: u64) -> String {
    let mut result = format!("{}", numbers[0]);
    for i in 0..operators.len() {
        let op_str = match operators[i] {
            ADD => " + ",
            MUL => " * ",
            CONCAT => " || ",
            _ => unreachable!(),
        };
        result.push_str(op_str);
        result.push_str(&numbers[i + 1].to_string());
    }
    result.push_str(&format!(" = {}", target));
    result
}

fn main() {
    let input = include_str!("../../input.txt");

    // Benchmark the solution
    let start = Instant::now();
    let (sum, solutions) = solve_equations(input);
    let duration = start.elapsed();

    println!("Solutions found:");
    for (solution, _) in solutions {
        println!("{}", solution);
    }
    println!("\nTotal calibration value: {}", sum);
    println!("Time taken: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_concat_numbers() {
        assert_eq!(concat_numbers(15, 6), 156);
        assert_eq!(concat_numbers(17, 8), 178);
        assert_eq!(concat_numbers(1, 23), 123);
        assert_eq!(concat_numbers(12, 345), 12345);
    }

    #[test]
    fn test_evaluation() {
        // Test basic operations
        assert_eq!(evaluate(&[10, 19], &[MUL]), Some(190));
        assert_eq!(evaluate(&[15, 6], &[CONCAT]), Some(156));

        // Test compound operations
        assert_eq!(evaluate(&[81, 40, 27], &[ADD, MUL]), Some(3267));
        assert_eq!(evaluate(&[11, 6, 16, 20], &[ADD, MUL, ADD]), Some(292));
        assert_eq!(evaluate(&[17, 8, 14], &[CONCAT, ADD]), Some(192));
    }

    #[test]
    fn test_operator_combinations() {
        let ops: Vec<Vec<Operator>> = generate_operator_combinations(2).collect();
        assert_eq!(ops.len(), 9); // 3^2 combinations

        // Test first few combinations
        assert_eq!(ops[0], vec![ADD, ADD]);
        assert_eq!(ops[1], vec![MUL, ADD]);
        assert_eq!(ops[2], vec![CONCAT, ADD]);
    }

    #[test]
    fn test_overflow_handling() {
        // Test multiplication overflow
        assert_eq!(evaluate(&[u64::MAX, 2], &[MUL]), None);

        // Test addition overflow
        assert_eq!(evaluate(&[u64::MAX, 1], &[ADD]), None);

        // Test valid large numbers
        assert_eq!(evaluate(&[u64::MAX / 2, 2], &[MUL]), Some(u64::MAX - 1));
    }
}
