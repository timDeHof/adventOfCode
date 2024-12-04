use std::error::Error;
use std::fs::read_to_string;
#[derive(Debug, PartialEq)]
enum Instruction {
    Multiply(i32, i32),
    Do,
    Dont,
}

fn parse_number(chars: &[char], start: usize) -> Option<(i32, usize)> {
    let mut end = start;
    while end < chars.len() && chars[end].is_ascii_digit() && end < start + 3 {
        end += 1;
    }
    if end > start {
        let num_str: String = chars[start..end].iter().collect();
        Some((num_str.parse().ok()?, end))
    } else {
        None
    }
}

fn parse_instruction(s: &str) -> Vec<Instruction> {
    let chars: Vec<char> = s.chars().collect();
    let mut instructions = Vec::new();
    let mut i = 0;

    while i < chars.len() {
        match chars.get(i..i + 4).map(|w| w.iter().collect::<String>()) {
            Some(w) if w == "mul(" => {
                if let Some((x, next_pos)) = parse_number(&chars, i + 4) {
                    if chars.get(next_pos) == Some(&',') {
                        if let Some((y, end_pos)) = parse_number(&chars, next_pos + 1) {
                            if chars.get(end_pos) == Some(&')') {
                                instructions.push(Instruction::Multiply(x, y));
                                i = end_pos + 1;
                                continue;
                            }
                        }
                    }
                }
            }
            Some(w)
                if w.starts_with("don'")
                    && chars.get(i + 4..i + 6) == Some(&['t', '('])
                    && chars.get(i + 6) == Some(&')') =>
            {
                instructions.push(Instruction::Dont);
                i += 7;
                continue;
            }
            Some(w) if w == "do()" => {
                instructions.push(Instruction::Do);
                i += 4;
                continue;
            }
            _ => {}
        }
        i += 1;
    }
    instructions
}

fn process_instructions(instructions: Vec<Instruction>) -> (Vec<Option<i32>>, i32) {
    let (results, _, sum) = instructions.iter().fold(
        (Vec::new(), true, 0),
        |(mut acc, enabled, sum), instr| match instr {
            Instruction::Multiply(x, y) => {
                let result = enabled.then_some(x * y);
                acc.push(result);
                (acc, enabled, sum + result.unwrap_or(0))
            }
            Instruction::Do => return (acc, true, sum),
            Instruction::Dont => return (acc, false, sum),
        },
    );
    (results, sum)
}

fn main() -> Result<(), Box<dyn Error>> {
    let content = include_str!("../../input.txt");
    let instructions = parse_instruction(&content);
    let (results, sum) = process_instructions(instructions);

    for (i, result) in results.iter().enumerate() {
        match result {
            Some(value) => println!("Multiplication {}: {}", i + 1, value),
            None => println!("Multiplication {}: disabled", i + 1),
        }
    }
    println!("Total sum: {}", sum);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_messy_input() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let instructions = parse_instruction(input);
        let (results, sum) = process_instructions(instructions);
        assert_eq!(results, vec![Some(8), Some(21), None, None, Some(2048), Some(88), Some(40)]);
        assert_eq!(sum, 2205);
    }
}
