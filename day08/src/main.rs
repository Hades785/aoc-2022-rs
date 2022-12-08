fn parse(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_string().parse::<u32>().unwrap())
                .collect()
        })
        .collect()
}

fn solve_a(input: &str) -> u32 {
    let grid = parse(input);

    let mut visible = grid.len() as u32 * 4 - 4;

    // Ignore borders.
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            let tree = grid[i][j];

            let mut up = 0;
            for y in 0..i {
                let t = grid[y][j];
                up = if t > up { t } else { up };
            }

            let mut down = 0;
            for y in i + 1..grid.len() {
                let t = grid[y][j];
                down = if t > down { t } else { down };
            }

            let mut left = 0;
            for x in 0..j {
                let t = grid[i][x];
                left = if t > left { t } else { left };
            }

            let mut right = 0;
            for x in j + 1..grid[i].len() {
                let t = grid[i][x];
                right = if t > right { t } else { right };
            }

            if up < tree || down < tree || left < tree || right < tree {
                visible += 1;
            }
        }
    }

    visible
}

fn solve_b(input: &str) -> u32 {
    let grid = parse(input);

    let mut high_score = 0;

    // Score would be 0 on the borders anyway.
    for i in 1..grid.len() - 1 {
        for j in 1..grid[i].len() - 1 {
            let tree = grid[i][j];

            let mut up = 0;
            for y in (0..i).rev() {
                up += 1;
                if grid[y][j] >= tree {
                    break;
                }
            }

            let mut down = 0;
            for y in i + 1..grid.len() {
                down += 1;
                if grid[y][j] >= tree {
                    break;
                }
            }

            let mut left = 0;
            for x in (0..j).rev() {
                left += 1;
                if grid[i][x] >= tree {
                    break;
                }
            }

            let mut right = 0;
            for x in j + 1..grid[i].len() {
                right += 1;
                if grid[i][x] >= tree {
                    break;
                }
            }

            let score = up * down * left * right;
            if score > high_score {
                high_score = score
            }
        }
    }

    high_score
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
    fn day08_a() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_a(input), 21);
    }

    #[test]
    fn day08_b() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_b(input), 8);
    }
}
