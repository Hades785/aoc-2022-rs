use std::ops::RangeInclusive;

#[derive(Debug)]
struct RangeBoundsFormatError;

fn to_range(bounds: (i32, i32)) -> RangeInclusive<i32> {
    let (a, b) = bounds;
    a..=b
}

fn solve(
    input: &str,
    filter_fn: impl FnMut(&(RangeInclusive<i32>, RangeInclusive<i32>)) -> bool,
) -> i32 {
    let pairs = input
        .lines()
        .map(|line| {
            line.split(",")
                .map(|a| a.split("-").map(|bound| bound.parse::<i32>().unwrap()))
                .map(|a| {
                    let bounds: Vec<i32> = a.collect();
                    match &bounds[..] {
                        &[low, high] => Ok((low, high)),
                        _ => Err(RangeBoundsFormatError),
                    }
                    .unwrap()
                })
        })
        .map(|pair| {
            let pair: Vec<(i32, i32)> = pair.collect();

            let (first, second) = match &pair[..] {
                &[a, b] => Ok((to_range(a), to_range(b))),
                _ => Err(RangeBoundsFormatError),
            }
            .unwrap();

            (first, second)
        });
    pairs.filter(filter_fn).count() as i32
}

fn solve_a(input: &str) -> i32 {
    solve(input, |(first, second)| {
        let mut first_in_second = true;
        let mut second_in_first = true;

        for i in first.clone() {
            if !second.contains(&i) {
                first_in_second = false;
                break;
            }
        }

        for i in second.clone() {
            if !first.contains(&i) {
                second_in_first = false;
                break;
            }
        }

        first_in_second || second_in_first
    })
}

fn solve_b(input: &str) -> i32 {
    solve(input, |(first, second)| {
        let mut first_in_second = false;
        let mut second_in_first = false;

        for i in first.clone() {
            if second.contains(&i) {
                first_in_second = true;
                break;
            }
        }

        for i in second.clone() {
            if first.contains(&i) {
                second_in_first = true;
                break;
            }
        }

        first_in_second || second_in_first
    })
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
    fn day04_a() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_a(input), 2);
    }

    #[test]
    fn day04_b() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_b(input), 4);
    }
}
