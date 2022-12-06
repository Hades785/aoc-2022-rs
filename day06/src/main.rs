use std::collections::HashSet;

fn solve<const WIN_SIZE: usize>(input: &str) -> usize {
    let characters = input.chars().collect::<Vec<_>>();
    let unique_counts = characters
        .windows(WIN_SIZE)
        .map(|window| -> HashSet<_> { HashSet::from_iter(window) })
        .map(|set| set.len());

    unique_counts.take_while(|c| c < &WIN_SIZE).count() + WIN_SIZE
}

fn main() {
    let input = include_str!("input_data/input.txt");
    println!("Part 1: {}", solve::<4>(input));
    println!("Part 2: {}", solve::<14>(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_a() {
        let input = include_str!("input_data/test.txt");
        let processed = input.lines().map(|msg| solve::<4>(msg)).collect::<Vec<_>>();
        assert_eq!(processed, vec![7, 5, 6, 10, 11]);
    }

    #[test]
    fn day06_b() {
        let input = include_str!("input_data/test.txt");
        let processed = input
            .lines()
            .map(|msg| solve::<14>(msg))
            .collect::<Vec<_>>();
        assert_eq!(processed, vec![19, 23, 23, 29, 26]);
    }
}
