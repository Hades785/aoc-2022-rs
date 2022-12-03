use std::{cmp::Ordering, str::FromStr};

#[derive(Debug)]
struct RpsFormatError;

#[derive(Clone, Copy, Debug)]
struct ParseMovesError;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Into<i32> for Move {
    fn into(self) -> i32 {
        match self {
            Self::Rock => 1,
            Self::Paper => 2,
            Self::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = ParseMovesError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_uppercase().as_str() {
            "A" => Ok(Self::Rock),
            "X" => Ok(Self::Rock),
            "B" => Ok(Self::Paper),
            "Y" => Ok(Self::Paper),
            "C" => Ok(Self::Scissors),
            "Z" => Ok(Self::Scissors),
            _ => Err(ParseMovesError),
        }
    }
}

impl PartialOrd for Move {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self == other {
            Some(Ordering::Equal)
        } else {
            if *self == Self::Rock && *other == Self::Scissors
                || *self == Self::Scissors && *other == Self::Paper
                || *self == Self::Paper && *other == Self::Rock
            {
                Some(Ordering::Greater)
            } else {
                Some(Ordering::Less)
            }
        }
    }
}

fn compute_scores(moves: &Vec<(Move, Move)>) -> Vec<i32> {
    moves
        .iter()
        .map(|m| {
            let (opponent_move, own_move) = m;
            let mut own_value: i32 = (*own_move).into();

            if own_move == opponent_move {
                own_value += 3;
            } else if own_move > opponent_move {
                own_value += 6;
            }

            own_value
        })
        .collect()
}

fn solve_a(input: &str) -> i32 {
    let lines = input.split("\n");
    let moves = lines
        .map(|a| {
            let moves: Vec<Move> = a
                .split_whitespace()
                .map(|b| b.parse::<Move>().unwrap())
                .collect();
            match &moves[..] {
                &[opponent, own] => Ok((opponent, own)),
                _ => Err(RpsFormatError),
            }
            .unwrap()
        })
        .collect::<Vec<(Move, Move)>>();

    let moves_scores = compute_scores(&moves);
    moves_scores.iter().sum()
}

#[derive(Debug)]
struct ParseCommandError;

#[derive(Debug)]
enum Command {
    Win,
    Lose,
    Tie,
}

impl Command {
    fn process(&self, m: Move) -> Move {
        match &self {
            Self::Tie => m.clone(),
            Self::Win => match m {
                Move::Rock => Move::Paper,
                Move::Paper => Move::Scissors,
                Move::Scissors => Move::Rock,
            },
            Self::Lose => match m {
                Move::Rock => Move::Scissors,
                Move::Paper => Move::Rock,
                Move::Scissors => Move::Paper,
            },
        }
    }
}

impl FromStr for Command {
    type Err = ParseCommandError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Self::Lose),
            "Y" => Ok(Self::Tie),
            "Z" => Ok(Self::Win),
            _ => Err(ParseCommandError),
        }
    }
}

fn solve_b(input: &str) -> i32 {
    let lines = input.lines();
    let moves = lines
        .map(|line| {
            let fields: Vec<&str> = line.split_whitespace().collect();
            match &fields[..] {
                &[opponent_move, command] => Ok((
                    opponent_move.parse::<Move>().unwrap(),
                    command.parse::<Command>().unwrap(),
                )),
                _ => Err(RpsFormatError),
            }
            .unwrap()
        })
        .map(|line| {
            let (opponent_move, command) = line;
            (opponent_move, command.process(opponent_move))
        })
        .collect::<Vec<(Move, Move)>>();

    let moves_scores = compute_scores(&moves);
    moves_scores.iter().sum()
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
    #[ignore]
    fn rps_rock_gt_scissors() {
        assert!(Move::Rock > Move::Scissors);
    }

    #[test]
    #[ignore]
    fn rps_paper_gt_rock() {
        assert!(Move::Paper > Move::Rock);
    }

    #[test]
    #[ignore]
    fn rps_scissors_gt_paper() {
        assert!(Move::Scissors > Move::Paper);
    }

    #[test]
    #[ignore]
    fn rps_rock_eq_rock() {
        assert!(Move::Rock == Move::Rock);
    }

    #[test]
    #[ignore]
    fn rps_paper_eq_paper() {
        assert!(Move::Paper == Move::Paper);
    }

    #[test]
    #[ignore]
    fn rps_scissors_eq_scissors() {
        assert!(Move::Scissors == Move::Scissors);
    }

    #[test]
    fn day02_a() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_a(input), 15);
    }

    #[test]
    fn day02_b() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_b(input), 12);
    }
}
