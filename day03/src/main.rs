#![feature(iter_array_chunks)]

use array_tool::vec::Intersect;
use tuple::Map;

// Only dealing with ASCII stuff, luckily, otherwise this would be completely wrong.
fn score(input: impl Iterator<Item = char>) -> impl Iterator<Item = u32> {
    input
        // Invert character case.
        .map(|c| {
            if c.is_lowercase() {
                c.to_uppercase().next()
            } else {
                c.to_lowercase().next()
            }
            .unwrap()
        })
        // Map characters to their unicode codepoint and remove offset to go on range [1..].
        .map(|a| a as u32 - 'A' as u32 + 1)
        // Correct offset error between lowercase and upercase in unicode, 6 characters in-between [ / ] ^ _ `.
        .map(|a| if a > 26 { a - 6 } else { a })
}

fn solve_a(input: &str) -> u32 {
    let pockets = input.lines().map(|sack| {
        sack.split_at(sack.len() / 2)
            .map(|pocket| pocket.chars().collect::<Vec<char>>())
    });

    let mismatches = pockets.map(|(a, b)| *a.intersect(b).first().unwrap());

    score(mismatches).sum()
}

fn solve_b(input: &str) -> u32 {
    let sacks = input
        .lines()
        .map(|sack| sack.chars().collect::<Vec<char>>());

    let badges = sacks
        .array_chunks::<3>()
        .map(|[a, b, c]| *a.intersect(b).intersect(c).first().unwrap());

    score(badges).sum()
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
    fn day03_a() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_a(input), 157);
    }

    #[test]
    fn day03_b() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_b(input), 70);
    }
}
