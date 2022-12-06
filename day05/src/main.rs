use regex::Regex;
use std::str::FromStr;

#[derive(Debug)]
struct InstructionParseError;

#[derive(Debug)]
struct Instruction {
    source: usize,
    target: usize,
    count: usize,
}

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let regex = Regex::new(r"move (\d+) from (\d+) to (\d+)").unwrap();

        let captures = regex.captures(s).map(|captures| {
            captures
                .iter()
                .skip(1)
                .flat_map(|c| c)
                .map(|c| c.as_str().parse::<usize>().unwrap())
                .collect::<Vec<_>>()
        });
        match captures.as_ref().map(|c| c.as_slice()) {
            Some(&[count, source, target]) => Ok(Instruction {
                source: source,
                target: target,
                count: count,
            }),
            _ => Err(InstructionParseError),
        }
    }
}

// https://stackoverflow.com/a/64499219
fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> {
    let len = v[0].len();

    let mut iterators = v.into_iter().map(|a| a.into_iter()).collect::<Vec<_>>();
    (0..len)
        .map(|_| {
            iterators
                .iter_mut()
                .map(|a| a.next().unwrap())
                .collect::<Vec<T>>()
        })
        .collect()
}

fn init(input: &str) -> Vec<Vec<String>> {
    let stack_lines = input.lines().rev();
    let stack_count = stack_lines
        .clone()
        .take(1)
        .last()
        .unwrap()
        .split_whitespace()
        .count();

    let regex = Regex::new(r"(?:\[\w\]| {3}) ?").unwrap();
    let unprocessed_stacks = stack_lines
        .skip(1)
        .map(|line| {
            format!(
                "{:<width$}",
                line,
                width = stack_count * 3 + (stack_count - 1)
            )
        })
        .map(|line| {
            let captures = regex
                .captures_iter(line.as_str())
                .map(|c| {
                    c.iter()
                        .map(|s| s.unwrap().as_str())
                        .map(|s| {
                            match s
                                .replace("[", "]")
                                .replace("]", "")
                                .replace(" ", "")
                                .split_whitespace()
                                .collect::<Vec<_>>()
                                .as_slice()
                            {
                                [element] => Some(element.to_string()),
                                _ => None,
                            }
                        })
                        .collect::<Vec<_>>()
                })
                .flatten();

            captures.collect()
        });

    let tmp_stacks = transpose(unprocessed_stacks.collect());
    tmp_stacks
        .iter()
        .map(|a| {
            a.iter()
                .map(|b| b.as_ref().unwrap_or(&"".to_string()).to_string())
                .collect::<Vec<_>>()
                .join("")
                .chars()
                .map(|c| c.to_string())
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
}

fn solve_a(input: &str) -> String {
    let split: Vec<&str> = input.split("\n\n").collect();
    let initial_stacks = split.first().unwrap();
    let instructions = split
        .last()
        .unwrap()
        .lines()
        .map(|i| i.parse::<Instruction>().unwrap());

    let mut stacks = init(*initial_stacks);

    for instruction in instructions {
        for _ in 0..instruction.count {
            let item = &stacks[instruction.source - 1].pop().unwrap();
            let _ = &stacks[instruction.target - 1].push(item.to_string());
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&"".to_string()).to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn solve_b(input: &str) -> String {
    let split: Vec<&str> = input.split("\n\n").collect();
    let initial_stacks = split.first().unwrap();
    let instructions = split
        .last()
        .unwrap()
        .lines()
        .map(|i| i.parse::<Instruction>().unwrap());

    let mut stacks = init(*initial_stacks);

    for instruction in instructions {
        // Pop multiple at once!
        let source = &mut stacks[instruction.source - 1];
        let mut items = source.split_off(source.len() - instruction.count);
        let _ = &stacks[instruction.target - 1].append(&mut items);
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap_or(&"".to_string()).to_string())
        .collect::<Vec<_>>()
        .join("")
}

fn main() {
    let input = include_str!("input_data/input.txt");
    println!("Part 1: {}", solve_a(input));
    println!("Part 2: {}", solve_b(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_a() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_a(input), "CMZ".to_string());
    }

    #[test]
    fn day05_b() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_b(input), "MCD".to_string());
    }
}
