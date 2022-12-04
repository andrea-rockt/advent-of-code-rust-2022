use std::collections::VecDeque;

use nom::{error::convert_error, Finish};

use crate::parser::parse_file;

pub fn part_one(input: &str) -> Result<String, String> {
    let (_, parsed_data) = parse_file(input)
        .finish()
        .map_err(|e| convert_error(input, e))?;

    let mut state: Vec<VecDeque<char>> = vec![VecDeque::new(); parsed_data.index.0.len()];

    for crate_definition in parsed_data.crates {
        for (idx, c) in crate_definition.0.iter().enumerate() {
            if let CrateDefinition::Full(f) = c {
                state[idx].push_front(*f);
            }
        }
    }

    for MoveDefinition { how_many, from, to } in parsed_data.moves {
        for _ in 0..how_many {
            let popped = state[(from - 1) as usize].pop_back().unwrap();
            state[(to - 1) as usize].push_back(popped)
        }
    }

    let mut sol = String::new();

    for s in state {
        sol.push(*s.back().unwrap_or(&' '));
    }

    Ok(sol)
}

pub fn part_two(input: &str) -> Result<String, String> {
    let (_, parsed_data) = parse_file(input)
        .finish()
        .map_err(|e| convert_error(input, e))?;

    let mut state: Vec<VecDeque<char>> = vec![VecDeque::new(); parsed_data.index.0.len()];

    for crate_definition in parsed_data.crates {
        for (idx, c) in crate_definition.0.iter().enumerate() {
            if let CrateDefinition::Full(f) = c {
                state[idx].push_front(*f);
            }
        }
    }

    for MoveDefinition { how_many, from, to } in parsed_data.moves {
        let mut tmp: VecDeque<char> = VecDeque::new();
        for _ in 0..how_many {
            let popped = state[(from - 1) as usize].pop_back().unwrap();
            tmp.push_back(popped);
        }

        for _ in 0..how_many {
            let popped = tmp.pop_back().unwrap();
            state[(to - 1) as usize].push_back(popped)
        }
    }

    let mut sol = String::new();

    for s in state {
        sol.push(*s.back().unwrap_or(&' '));
    }

    Ok(sol)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 5);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}
#[derive(Debug)]
pub enum CrateDefinition {
    Empty,
    Full(char),
}

#[derive(Debug)]
pub struct CratesDefinition(Vec<CrateDefinition>);

#[derive(Debug)]
pub struct IndexDefinition(Vec<u32>);

#[derive(Debug)]
pub struct MoveDefinition {
    how_many: u32,
    from: u32,
    to: u32,
}

#[derive(Debug)]
pub struct FileDefinition {
    crates: Vec<CratesDefinition>,
    index: IndexDefinition,
    moves: Vec<MoveDefinition>,
}

mod parser {
    use crate::{
        CrateDefinition, CratesDefinition, FileDefinition, IndexDefinition, MoveDefinition,
    };
    use nom::branch::alt;
    use nom::bytes::complete::tag;
    use nom::character::complete::{
        anychar, char, digit1, newline, space0, space1,
    };
    use nom::combinator::{map, map_res};
    use nom::error::VerboseError;
    use nom::multi::{count, many1, separated_list1};
    use nom::sequence::{preceded, terminated, tuple};
    use nom::IResult;

    type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

    pub fn parse_empty_crate(input: &str) -> Res<CrateDefinition> {
        map(count(char(' '), 3), |_| CrateDefinition::Empty)(input)
    }

    pub fn parse_crate(input: &str) -> Res<CrateDefinition> {
        map(terminated(preceded(char('['), anychar), char(']')), |c| {
            CrateDefinition::Full(c)
        })(input)
    }

    pub fn parse_crates_definition(input: &str) -> Res<CratesDefinition> {
        map(
            separated_list1(char(' '), alt((parse_empty_crate, parse_crate))),
            CratesDefinition,
        )(input)
    }

    pub fn unsigned_integer(input: &str) -> Res<u32> {
        map_res(digit1, |c: &str| c.parse::<u32>())(input)
    }

    pub fn parse_index_definition(input: &str) -> Res<IndexDefinition> {
        map(
            terminated(
                preceded(space0, separated_list1(space1, unsigned_integer)),
                space0,
            ),
            IndexDefinition,
        )(input)
    }

    pub fn parse_move_definition(input: &str) -> Res<MoveDefinition> {
        map(
            tuple((
                tag("move"),
                terminated(preceded(space1, unsigned_integer), space1),
                tag("from"),
                terminated(preceded(space1, unsigned_integer), space1),
                tag("to"),
                preceded(space1, unsigned_integer),
            )),
            |(_, how_many, _, from, _, to)| MoveDefinition { how_many, from, to },
        )(input)
    }

    pub fn parse_file(input: &str) -> Res<FileDefinition> {
        map(
            tuple((
                many1(terminated(parse_crates_definition, newline)),
                terminated(parse_index_definition, newline),
                newline,
                separated_list1(newline, parse_move_definition),
            )),
            |(crates, index, _, moves)| FileDefinition {
                crates,
                index,
                moves,
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_one(&input), Ok("CMZ".to_string()));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 5);
        assert_eq!(part_two(&input), Ok("MCD".to_string()));
    }
}
