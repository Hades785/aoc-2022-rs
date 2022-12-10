use std::{
    collections::HashSet,
    ops::{Add, AddAssign, Sub},
    str::FromStr,
};

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    fn add(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x + rhs.x,
            y: self.y + rhs.x,
        }
    }
}

impl AddAssign for Point {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Point {
    type Output = Point;
    fn sub(self, rhs: Self) -> Self::Output {
        Point {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(Debug)]
struct PointParseError;

impl FromStr for Point {
    type Err = PointParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s: Vec<&str> = s.split_whitespace().collect();
        match s.as_slice() {
            [direction, value] => {
                let value: i32 = value.parse().unwrap();
                match direction.to_uppercase().as_str() {
                    "U" => Ok(Point { x: 0, y: value }),
                    "D" => Ok(Point { x: 0, y: -value }),
                    "L" => Ok(Point { x: -value, y: 0 }),
                    "R" => Ok(Point { x: value, y: 0 }),
                    _ => Err(PointParseError),
                }
            }
            _ => Err(PointParseError),
        }
    }
}

impl Point {
    fn norm(self) -> u32 {
        let sum: f64 = (self.x.pow(2) + self.y.pow(2)).into();
        let sqrt = sum.sqrt();
        sqrt.floor() as u32
    }

    fn approach(&mut self, other: Self) -> Vec<Self> {
        let mut diff = other - *self;
        let mut track = vec![];

        while diff.norm() > 1 {
            let unit = Point {
                x: diff.x.clamp(-1, 1),
                y: diff.y.clamp(-1, 1),
            };

            *self += unit;
            track.push(*self);

            // Update diff.
            diff = other - *self;
        }

        track
    }
}

fn solve_a(input: &str) -> u32 {
    let mut head = Point { x: 0, y: 0 };
    let mut tail = Point { x: 0, y: 0 };
    let mut tracks = vec![tail];

    let commands = input.lines().map(|line| line.parse::<Point>().unwrap());
    for command in commands {
        head += command;
        let track = tail.approach(head);
        tracks.extend(track);
    }

    HashSet::<_>::from_iter(tracks.iter()).len() as u32
}

fn main() {
    let input = include_str!("input_data/input.txt");
    println!("Part 1: {}", solve_a(input));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day09_a() {
        let input = include_str!("input_data/test_a.txt");
        assert_eq!(solve_a(input), 13);
    }
}
