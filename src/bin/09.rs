use std::{
    collections::HashSet,
    ops::{Add, Sub, AddAssign},
};

use nom::Finish;
use parser::parse_file;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub enum Direction {
    Left,
    Up,
    Right,
    Down,
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Move(Direction, u32);

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
pub struct Position(i32, i32);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct Rope {
    knots: Vec<Position>,
}

impl Sub for Position {
    type Output = Position;

    fn sub(self, rhs: Self) -> Self::Output {
        Position(self.0 - rhs.0, self.1 - rhs.1)
    }
}

impl Add for Position {
    type Output = Position;

    fn add(self, rhs: Self) -> Self::Output {
        Position(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl AddAssign for Position {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
        self.1 += rhs.1;
    }
}

impl Rope {
    fn update(&mut self, dir: &Direction) -> Position {
        let movement = match dir {
            Direction::Left => Position(-1, 0),
            Direction::Up => Position(0, 1),
            Direction::Right => Position(1, 0),
            Direction::Down => Position(0, -1),
        };

        self.knots[0] += movement;

        for i in 0..self.knots.len() - 1 {
            let h = self.knots[i];
            let t = self.knots[i + 1];

            self.knots[i + 1] = Self::calculate_new_tail_position(h, t);
        }

        *self.knots.last().unwrap()
    }

    fn simulate(&mut self, parsed: &Vec<Move>) -> u32 {
        let mut tail_positions: HashSet<Position> = HashSet::new();
        tail_positions.insert(Position(0, 0));
        for Move(dir, steps) in parsed {
            for _ in 0..*steps {
                let new_tail_position = self.update(dir);
                tail_positions.insert(new_tail_position);
            }
        }
        tail_positions.len() as u32
    }

    fn calculate_new_tail_position(head: Position, tail: Position) -> Position {
        let Position(distance_x, distance_y) = head - tail;
        let distance = i32::max(i32::abs(distance_x), i32::abs(distance_y));

        let movement = if distance > 1 {
            Position(i32::signum(distance_x), i32::signum(distance_y))
        } else {
            Position(0, 0)
        };

        tail + movement
    }
}

pub fn part_one(input: &str) -> Result<u32, String> {
    let parsed = parse_file(input).finish().map_err(|x| x.to_string())?.1;

    let mut rope = Rope {
        knots: vec![Position(0, 0); 2],
    };

    let tail_positions = rope.simulate(&parsed);

    Ok(tail_positions as u32)
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let parsed = parse_file(input).finish().map_err(|x| x.to_string())?.1;

    let mut rope = Rope {
        knots: vec![Position(0, 0); 10],
    };

    let tail_positions = rope.simulate(&parsed);

    Ok(tail_positions as u32)
}

mod parser {

    use nom::branch::alt;

    use nom::character::complete::{char, digit1, line_ending, space1};
    use nom::combinator::{map, map_res};
    use nom::error::VerboseError;
    use nom::multi::separated_list1;
    use nom::sequence::{terminated, tuple};
    use nom::IResult;

    use crate::{Direction, Move};

    type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

    fn parse_up(input: &str) -> Res<Direction> {
        map(char('U'), |_| Direction::Up)(input)
    }

    fn parse_down(input: &str) -> Res<Direction> {
        map(char('D'), |_| Direction::Down)(input)
    }

    fn parse_left(input: &str) -> Res<Direction> {
        map(char('L'), |_| Direction::Left)(input)
    }

    fn parse_right(input: &str) -> Res<Direction> {
        map(char('R'), |_| Direction::Right)(input)
    }

    fn parse_direction(input: &str) -> Res<Direction> {
        alt((parse_left, parse_right, parse_up, parse_down))(input)
    }

    pub fn parse_unsigned_integer(input: &str) -> Res<u32> {
        map_res(digit1, |c: &str| c.parse::<u32>())(input)
    }

    fn parse_move(input: &str) -> Res<Move> {
        map(
            tuple((terminated(parse_direction, space1), parse_unsigned_integer)),
            |(direction, steps)| Move(direction, steps),
        )(input)
    }

    pub fn parse_file(input: &str) -> Res<Vec<Move>> {
        separated_list1(line_ending, parse_move)(input)
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 9);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;

    #[test]
    fn test_part_parse_file() {
        let input = advent_of_code::read_file("examples", 9);
        let data = input.split("\n---\n").next().unwrap();
        let res = super::parser::parse_file(data)
            .finish()
            .map_err(|e| e.to_string());

        let expected: Vec<Move> = vec![
            Move(Direction::Right, 4),
            Move(Direction::Up, 4),
            Move(Direction::Left, 3),
            Move(Direction::Down, 1),
            Move(Direction::Right, 4),
            Move(Direction::Down, 1),
            Move(Direction::Left, 5),
            Move(Direction::Right, 2),
        ];

        assert_eq!(res, Ok(("", expected)));
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 9);
        let data = input.split("---\n").next().expect("first chunk");
        assert_eq!(part_one(data), Ok(13));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 9);
        let data = input.split("---\n").last().expect("second chunk");
        assert_eq!(part_two(data), Ok(36));
    }
}
