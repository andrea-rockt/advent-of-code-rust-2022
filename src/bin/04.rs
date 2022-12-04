pub fn part_one(input: &str) -> Result<u32, String> {
    let x: Result<Vec<CampSectionAssignment>, String> = input
        .lines()
        .map(|l| l.parse::<CampSectionAssignment>())
        .collect();

    match x {
        Result::Ok(r) => Ok(r.iter().filter(|x| x.fully_contains()).map(|_| 1).sum()),
        Result::Err(x) => Result::Err(x),
    }
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let x: Result<Vec<CampSectionAssignment>, String> = input
        .lines()
        .map(|l| l.parse::<CampSectionAssignment>())
        .collect();

    match x {
        Result::Ok(r) => Ok(r.iter().filter(|x| x.overlap()).map(|_| 1).sum()),
        Result::Err(x) => Result::Err(x),
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 4);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CampSection {
    index: u32,
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]

pub struct CampSectionRange {
    start: CampSection,
    end: CampSection,
}

impl CampSectionRange {
    fn overlap(&self, other: &CampSectionRange) -> bool {
        self.end.index >= other.start.index && self.start.index <= other.end.index
    }

    fn fully_contains(&self, other: &CampSectionRange) -> bool {
        self.end.index >= other.end.index && self.start.index <= other.start.index
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CampSectionAssignment {
    left: CampSectionRange,
    right: CampSectionRange,
}

impl CampSectionAssignment {
    fn fully_contains(&self) -> bool {
        self.left.fully_contains(&self.right) || self.right.fully_contains(&self.left)
    }
    fn overlap(&self) -> bool {
        self.left.overlap(&self.right) || self.right.overlap(&self.left)
    }
}

mod parser {
    use crate::{CampSection, CampSectionAssignment, CampSectionRange};
    use nom::error::convert_error;
    use nom::sequence::separated_pair;
    use nom::Finish;
    use nom::{
        character::complete::{char, digit1},
        combinator::map_res,
        error::VerboseError,
        IResult,
    };
    use std::str::FromStr;

    type Res<T, U> = IResult<T, U, VerboseError<T>>;

    impl FromStr for CampSectionRange {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match parse_camp_section_range(s).finish() {
                Ok((_remaining, name)) => Ok(name),
                Err(e) => Err(convert_error(s, e)),
            }
        }
    }

    impl FromStr for CampSectionAssignment {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match parse_camp_section_assignment(s).finish() {
                Ok((_remaining, res)) => Ok(res),
                Err(e) => Err(convert_error(s, e)),
            }
        }
    }

    impl FromStr for CampSection {
        type Err = String;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match parse_camp_section(s).finish() {
                Ok((_remaining, res)) => Ok(res),
                Err(e) => Err(convert_error(s, e)),
            }
        }
    }

    pub fn parse_camp_section(input: &str) -> Res<&str, CampSection> {
        map_res(map_res(digit1, |r: &str| r.parse::<u32>()), |index: u32| {
            Ok::<CampSection, VerboseError<&str>>(CampSection { index })
        })(input)
    }

    pub fn parse_camp_section_range(input: &str) -> Res<&str, CampSectionRange> {
        map_res(
            separated_pair(parse_camp_section, char('-'), parse_camp_section),
            |(start, end)| {
                Ok::<CampSectionRange, VerboseError<&str>>(CampSectionRange { start, end })
            },
        )(input)
    }

    pub fn parse_camp_section_assignment(input: &str) -> Res<&str, CampSectionAssignment> {
        map_res(
            separated_pair(
                parse_camp_section_range,
                char(','),
                parse_camp_section_range,
            ),
            |res| {
                Ok::<CampSectionAssignment, VerboseError<&str>>(CampSectionAssignment {
                    left: res.0,
                    right: res.1,
                })
            },
        )(input)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_one(&input), Ok(2));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 4);
        assert_eq!(part_two(&input), Ok(4));
    }
}
