use std::str::FromStr;

#[derive(Clone, Copy, Debug)]
struct State {
    x: i32,
    tick: u32,
    power: i32,
}

impl State {
    fn new() -> Self {
        Self {
            x: 1,
            tick: 0,
            power: 0,
        }
    }

    fn tick(&mut self) {
        self.tick += 1;

        if self.tick % 40 == 20 {
            self.power += self.tick as i32 * self.x;
        }
    }
}

#[derive(Clone, Copy, Debug)]
enum Instruction {
    NoOp,
    AddX { val: i32 },
    RawAddX { val: i32 },
}

impl Instruction {
    fn process(self, state: &mut State) -> Option<Instruction> {
        state.tick();

        match self {
            Self::NoOp => None,
            Self::AddX { val } => Some(Self::RawAddX { val }),
            Self::RawAddX { val } => {
                state.x += val;
                None
            }
        }
    }
}

#[derive(Debug)]
struct InstructionParseError;

impl FromStr for Instruction {
    type Err = InstructionParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut fragments = s.split_whitespace();
        let i = fragments.next();
        let args: Vec<_> = fragments.collect();

        match i {
            Some("noop") => Ok(Instruction::NoOp),
            Some("addx") => Ok(Instruction::AddX {
                val: args[0].parse::<i32>().unwrap(),
            }),
            _ => Err(InstructionParseError),
        }
    }
}

fn solve_a(input: &str) -> i32 {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap());

    let mut state = State::new();

    for instruction in instructions {
        let mut r = instruction.process(&mut state);

        while let Some(instruction) = r {
            r = instruction.process(&mut state);
        }
    }

    state.power
}

fn solve_b(input: &str) -> String {
    let instructions = input
        .lines()
        .map(|line| line.parse::<Instruction>().unwrap());

    let mut state = State::new();
    let mut output = String::new();

    fn draw(state: &mut State, output: &mut String) {
        let carret = state.tick as i32 % 40;
        let pos = state.x;

        if carret == 0 {
            *output += "\n";
        }

        *output += if carret >= pos - 1 && carret <= pos + 1 {
            "#"
        } else {
            "."
        };
    }

    for instruction in instructions {
        draw(&mut state, &mut output);
        let mut r = instruction.process(&mut state);

        while let Some(instruction) = r {
            draw(&mut state, &mut output);
            r = instruction.process(&mut state);
        }
    }

    output.trim().to_string()
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
    fn day10_a() {
        let input = include_str!("input_data/test.txt");
        assert_eq!(solve_a(input), 13140);
    }

    #[test]
    fn day10_b() {
        let input = include_str!("input_data/test.txt");
        let output = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
        "#
        .trim()
        .to_string();
        assert_eq!(solve_b(input), output);
    }
}
