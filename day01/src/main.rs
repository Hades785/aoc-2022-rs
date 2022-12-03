fn solve_for_n<const N: usize>(input: &str) -> i32 {
    let mut sums = input
        .split("\n\n")
        .map(|a| {
            a.split_whitespace()
                .map(|b| b.parse::<i32>().unwrap())
                .sum::<i32>()
        })
        .collect::<Vec<i32>>();

    sums.sort();
    sums.reverse();

    sums.iter().take(N).sum()
}

fn main() {
    let input = include_str!("input_data/input.txt");

    println!("Part 1: {}", solve_for_n::<1>(input));
    println!("Part 2: {}", solve_for_n::<3>(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day01_a() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_for_n::<1>(input.into()), 24000);
    }

    #[test]
    fn day01_b() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_for_n::<3>(input.into()), 45000);
    }
}
