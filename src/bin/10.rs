use nom::Finish;
use parser::parse_program;

struct Cpu<'a> {
    x: i32,
    program: &'a Vec<Instruction>,
    program_counter: usize,
    cycles_remaining: u32,
    buffered_instruction: Instruction,
}

impl<'a> Cpu<'a> {
    fn new(program: &'a Vec<Instruction>) -> Cpu {
        Cpu {
            x: 1,
            program,
            program_counter: 0,
            cycles_remaining: 0,
            buffered_instruction: Instruction::Noop,
        }
    }

    fn step(&mut self) -> bool {
        if self.cycles_remaining > 0 {
            self.cycles_remaining -= 1;

            if self.cycles_remaining == 0 {
                match self.buffered_instruction {
                    Instruction::Noop => {}
                    Instruction::Addx(number) => {
                        self.x += number;
                    }
                }
            }

            return true;
        }

        if self.program_counter == self.program.len() {
            return false;
        }

        self.buffered_instruction = self.program[self.program_counter];

        match self.buffered_instruction {
            Instruction::Noop => {
                self.cycles_remaining = 0;
            }
            Instruction::Addx(_) => {
                self.cycles_remaining = 1;
            }
        };

        self.program_counter += 1;

        true
    }
}

pub fn part_one(input: &str) -> Result<i32, String> {
    let (_, program) = parse_program(input).finish().map_err(|e| e.to_string())?;

    let mut cpu = Cpu::new(&program.buffer);

    let mut cycle = 1;

    let mut strength = 0;

    loop {
        if cycle == 20
            || cycle == 60
            || cycle == 100
            || cycle == 140
            || cycle == 180
            || cycle == 220
        {
            strength += cpu.x * cycle
        }

        cycle += 1;
        if !cpu.step() {
            break;
        }
    }

    Ok(strength)
}

pub fn part_two(input: &str) -> Result<String, String> {
    let (_, program) = parse_program(input).finish().map_err(|e| e.to_string())?;

    let mut cpu = Cpu::new(&program.buffer);

    let mut cycle = 1;

    let mut indexes: [bool; 240] = [false; 240];

    loop {
        let row = (cycle - 1) % 40;
        let column = (cycle - 1) / 40;

        let left_position = cpu.x - 1;
        let middle_position = cpu.x;
        let right_position = cpu.x + 1;

        let mut sprite_position: [bool; 40] = [false; 40];

        if (0..40).contains(&left_position) {
            sprite_position[left_position as usize] = true;
        }

        if (0..40).contains(&middle_position) {
            sprite_position[middle_position as usize] = true;
        }

        if (0..40).contains(&right_position) {
            sprite_position[right_position as usize] = true;
        }

        if left_position == row {
            indexes[(left_position + column * 40) as usize] = true;
        }

        if right_position == row {
            indexes[(right_position + column * 40) as usize] = true;
        }

        if middle_position == row {
            indexes[(middle_position + column * 40) as usize] = true;
        }

        cycle += 1;
        if !cpu.step() {
            break;
        }
    }

    let mut result = String::new();

    (0..240).for_each(|i| {
        if indexes[i] {
            result.push('#');
        } else {
            result.push('.');
        }

        if i != 0 && (i + 1) % 40 == 0 {
            result.push('\n')
        }
    });

    Ok(result)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 10);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Instruction {
    Noop,
    Addx(i32),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Program {
    buffer: Vec<Instruction>,
}

mod parser {

    use nom::branch::alt;

    use nom::bytes::complete::tag;
    use nom::character::complete::{i32 as number, line_ending, space1};
    use nom::combinator::map;
    use nom::error::VerboseError;
    use nom::multi::separated_list1;
    use nom::sequence::tuple;
    use nom::IResult;

    use crate::{Instruction, Program};

    type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

    fn parse_instruction(input: &str) -> Res<Instruction> {
        alt((parse_noop, parse_addx))(input)
    }

    fn parse_noop(input: &str) -> Res<Instruction> {
        map(tag("noop"), |_| Instruction::Noop)(input)
    }

    fn parse_addx(input: &str) -> Res<Instruction> {
        map(tuple((tag("addx"), space1, number)), |(_, _, argument)| {
            Instruction::Addx(argument)
        })(input)
    }

    pub fn parse_program(input: &str) -> Res<Program> {
        map(separated_list1(line_ending, parse_instruction), |buffer| {
            Program { buffer }
        })(input)
    }
}
#[cfg(test)]
mod tests {
    use nom::Finish;

    use crate::parser::parse_program;

    use super::*;

    #[test]
    fn test_load_program() {
        let input = advent_of_code::read_file("examples", 10);

        let result: Result<(&str, Program), String> =
            parse_program(&input).finish().map_err(|e| e.to_string());
        use Instruction::*;
        let expected: Result<(&str, Program), String> = Ok((
            "",
            Program {
                buffer: vec![
                    Addx(15),
                    Addx(-11),
                    Addx(6),
                    Addx(-3),
                    Addx(5),
                    Addx(-1),
                    Addx(-8),
                    Addx(13),
                    Addx(4),
                    Noop,
                    Addx(-1),
                    Addx(5),
                    Addx(-1),
                    Addx(5),
                    Addx(-1),
                    Addx(5),
                    Addx(-1),
                    Addx(5),
                    Addx(-1),
                    Addx(-35),
                    Addx(1),
                    Addx(24),
                    Addx(-19),
                    Addx(1),
                    Addx(16),
                    Addx(-11),
                    Noop,
                    Noop,
                    Addx(21),
                    Addx(-15),
                    Noop,
                    Noop,
                    Addx(-3),
                    Addx(9),
                    Addx(1),
                    Addx(-3),
                    Addx(8),
                    Addx(1),
                    Addx(5),
                    Noop,
                    Noop,
                    Noop,
                    Noop,
                    Noop,
                    Addx(-36),
                    Noop,
                    Addx(1),
                    Addx(7),
                    Noop,
                    Noop,
                    Noop,
                    Addx(2),
                    Addx(6),
                    Noop,
                    Noop,
                    Noop,
                    Noop,
                    Noop,
                    Addx(1),
                    Noop,
                    Noop,
                    Addx(7),
                    Addx(1),
                    Noop,
                    Addx(-13),
                    Addx(13),
                    Addx(7),
                    Noop,
                    Addx(1),
                    Addx(-33),
                    Noop,
                    Noop,
                    Noop,
                    Addx(2),
                    Noop,
                    Noop,
                    Noop,
                    Addx(8),
                    Noop,
                    Addx(-1),
                    Addx(2),
                    Addx(1),
                    Noop,
                    Addx(17),
                    Addx(-9),
                    Addx(1),
                    Addx(1),
                    Addx(-3),
                    Addx(11),
                    Noop,
                    Noop,
                    Addx(1),
                    Noop,
                    Addx(1),
                    Noop,
                    Noop,
                    Addx(-13),
                    Addx(-19),
                    Addx(1),
                    Addx(3),
                    Addx(26),
                    Addx(-30),
                    Addx(12),
                    Addx(-1),
                    Addx(3),
                    Addx(1),
                    Noop,
                    Noop,
                    Noop,
                    Addx(-9),
                    Addx(18),
                    Addx(1),
                    Addx(2),
                    Noop,
                    Noop,
                    Addx(9),
                    Noop,
                    Noop,
                    Noop,
                    Addx(-1),
                    Addx(2),
                    Addx(-37),
                    Addx(1),
                    Addx(3),
                    Noop,
                    Addx(15),
                    Addx(-21),
                    Addx(22),
                    Addx(-6),
                    Addx(1),
                    Noop,
                    Addx(2),
                    Addx(1),
                    Noop,
                    Addx(-10),
                    Noop,
                    Noop,
                    Addx(20),
                    Addx(1),
                    Addx(2),
                    Addx(2),
                    Addx(-6),
                    Addx(-11),
                    Noop,
                    Noop,
                    Noop,
                ],
            },
        ));
        assert_eq!(result, expected);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 10);

        assert_eq!(part_one(&input), Ok(13140));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 10);

        let outcome = part_two(&input);

        let result = outcome.clone().unwrap();

        println!("{}", result);

        let expected = "##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######.....
";

        assert_eq!(outcome, Ok(expected.to_string()));
    }
}
