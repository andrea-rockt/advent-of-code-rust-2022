use itertools::Itertools;
use nom::Finish;
use parser::parse_monkeys;

pub fn part_one(input: &str) -> Result<u64, String> {
    let (_, monkeys) = parse_monkeys(input).finish().map_err(|e| e.to_string())?;

    let mut mutable_monkeys = monkeys;

    let mut inspections = vec![0; mutable_monkeys.len()];

    for _ in 0..20 {
        for i in 0..mutable_monkeys.len() {
            for (removed_indx, item_index) in (0..mutable_monkeys[i].items.len()).enumerate() {
                inspections[i] += 1;

                let mut alert_level =
                    mutable_monkeys[i].items[item_index - removed_indx].alert_level;

                let Expression {
                    lhs: _,
                    operation,
                    rhs,
                } = mutable_monkeys[i].operation;

                match operation {
                    Operation::Add => match rhs {
                        Element::Old => alert_level += alert_level,
                        Element::Literal(operand) => alert_level += operand,
                    },
                    Operation::Multiply => match rhs {
                        Element::Old => alert_level *= alert_level,
                        Element::Literal(operand) => alert_level *= operand,
                    },
                }

                alert_level /= 3;

                if alert_level % mutable_monkeys[i].test.divisor == 0 {
                    let mut removed = mutable_monkeys[i].items.remove(item_index - removed_indx);
                    let idx = mutable_monkeys[i].test.throw_true as usize;
                    removed.alert_level = alert_level;
                    mutable_monkeys[idx].items.push(removed);
                } else {
                    let mut removed = mutable_monkeys[i].items.remove(item_index - removed_indx);
                    let idx = mutable_monkeys[i].test.throw_false as usize;
                    removed.alert_level = alert_level;
                    mutable_monkeys[idx].items.push(removed);
                }
            }
        }
    }

    Ok(inspections.iter().sorted().rev().take(2).product())
}

pub fn part_two(input: &str) -> Result<u64, String> {
    let (_, monkeys) = parse_monkeys(input).finish().map_err(|e| e.to_string())?;

    let mut mutable_monkeys = monkeys;

    let mut inspections = vec![0; mutable_monkeys.len()];

    let relief: u64 = mutable_monkeys.iter().map(|m| m.test.divisor).product();

    for _ in 0..10000 {
        for i in 0..mutable_monkeys.len() {
            for (removed_indx, item_index) in (0..mutable_monkeys[i].items.len()).enumerate() {
                inspections[i] += 1;

                let mut alert_level =
                    mutable_monkeys[i].items[item_index - removed_indx].alert_level;

                let Expression {
                    lhs: _,
                    operation,
                    rhs,
                } = mutable_monkeys[i].operation;

                match operation {
                    Operation::Add => match rhs {
                        Element::Old => alert_level += alert_level,
                        Element::Literal(operand) => alert_level += operand,
                    },
                    Operation::Multiply => match rhs {
                        Element::Old => alert_level *= alert_level,
                        Element::Literal(operand) => alert_level *= operand,
                    },
                }

                alert_level %= relief;

                if alert_level % mutable_monkeys[i].test.divisor == 0 {
                    let mut removed = mutable_monkeys[i].items.remove(item_index - removed_indx);
                    let idx = mutable_monkeys[i].test.throw_true as usize;
                    removed.alert_level = alert_level;
                    mutable_monkeys[idx].items.push(removed);
                } else {
                    let mut removed = mutable_monkeys[i].items.remove(item_index - removed_indx);
                    let idx = mutable_monkeys[i].test.throw_false as usize;
                    removed.alert_level = alert_level;
                    mutable_monkeys[idx].items.push(removed);
                }
            }
        }
    }

    Ok(inspections.iter().sorted().rev().take(2).product())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Monkey {
    idx: u64,
    items: Vec<Item>,
    operation: Expression,
    test: Test,
}
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Item {
    alert_level: u64,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Test {
    divisor: u64,
    throw_true: u64,
    throw_false: u64,
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct Expression {
    lhs: Element,
    operation: Operation,
    rhs: Element,
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Element {
    Old,
    Literal(u64),
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
pub enum Operation {
    Add,
    Multiply,
}

mod parser {

    use itertools::Itertools;
    use nom::branch::alt;

    use nom::bytes::complete::tag;
    use nom::character::complete::{line_ending, space1, u64 as number};
    use nom::combinator::map;
    use nom::error::VerboseError;
    use nom::multi::separated_list1;
    use nom::sequence::{preceded, terminated, tuple};
    use nom::IResult;

    use crate::{Element, Expression, Item, Monkey, Operation, Test};

    type Res<'a, U> = IResult<&'a str, U, VerboseError<&'a str>>;

    struct MonkeyLine(u64);
    struct StartingItemsLine(Vec<u64>);

    struct OperationLine(Element, Operation, Element);

    struct TestLine(u64);

    struct TrueLine(u64);

    struct FalseLine(u64);

    fn parse_monkey_line(input: &str) -> Res<MonkeyLine> {
        map(
            terminated(preceded(tag("Monkey "), number), tag(":")),
            MonkeyLine,
        )(input)
    }

    fn parse_starting_items_line(input: &str) -> Res<StartingItemsLine> {
        map(
            preceded(
                tag("  Starting items: "),
                separated_list1(tag(", "), number),
            ),
            StartingItemsLine,
        )(input)
    }

    fn parse_element(input: &str) -> Res<Element> {
        alt((parse_old, parse_literal))(input)
    }

    fn parse_old(input: &str) -> Res<Element> {
        map(tag("old"), |_| Element::Old)(input)
    }

    fn parse_literal(input: &str) -> Res<Element> {
        map(number, Element::Literal)(input)
    }

    fn parse_add(input: &str) -> Res<Operation> {
        map(tag("+"), |_| Operation::Add)(input)
    }

    fn parse_multiply(input: &str) -> Res<Operation> {
        map(tag("*"), |_| Operation::Multiply)(input)
    }

    fn parse_operation(input: &str) -> Res<Operation> {
        alt((parse_add, parse_multiply))(input)
    }

    fn parse_test_line(input: &str) -> Res<TestLine> {
        map(preceded(tag("  Test: divisible by "), number), TestLine)(input)
    }

    fn parse_true_line(input: &str) -> Res<TrueLine> {
        map(
            preceded(tag("    If true: throw to monkey "), number),
            TrueLine,
        )(input)
    }

    fn parse_false_line(input: &str) -> Res<FalseLine> {
        map(
            preceded(tag("    If false: throw to monkey "), number),
            FalseLine,
        )(input)
    }

    fn parse_monkey(input: &str) -> Res<Monkey> {
        map(
            tuple((
                terminated(parse_monkey_line, line_ending),
                terminated(parse_starting_items_line, line_ending),
                terminated(parse_operation_line, line_ending),
                terminated(parse_test_line, line_ending),
                terminated(parse_true_line, line_ending),
                terminated(parse_false_line, line_ending),
            )),
            |(
                MonkeyLine(m),
                StartingItemsLine(items),
                OperationLine(lhs, operation, rhs),
                TestLine(divisor),
                TrueLine(throw_true),
                FalseLine(throw_false),
            )| Monkey {
                idx: m,
                operation: Expression {
                    lhs,
                    operation,
                    rhs,
                },
                test: Test {
                    divisor,
                    throw_true,
                    throw_false,
                },
                items: items
                    .iter()
                    .map(|alert_level| Item {
                        alert_level: *alert_level,
                    })
                    .collect_vec(),
            },
        )(input)
    }

    fn parse_operation_line(input: &str) -> Res<OperationLine> {
        map(
            preceded(
                tag("  Operation: new = "),
                tuple((
                    parse_element,
                    space1,
                    parse_operation,
                    space1,
                    parse_element,
                )),
            ),
            |(lhs, _, operation, _, rhs)| OperationLine(lhs, operation, rhs),
        )(input)
    }

    pub fn parse_monkeys(input: &str) -> Res<Vec<Monkey>> {
        separated_list1(line_ending, parse_monkey)(input)
    }
}

#[cfg(test)]
mod tests {
    use nom::Finish;

    use super::*;
    #[test]
    fn test_parse_monkeys() {
        let input = advent_of_code::read_file("examples", 11);
        use Element::*;
        use Operation::*;
        let expected = Ok((
            "",
            vec![
                Monkey {
                    idx: 0,
                    items: vec![Item { alert_level: 79 }, Item { alert_level: 98 }],
                    operation: Expression {
                        lhs: Old,
                        operation: Multiply,
                        rhs: Literal(19),
                    },
                    test: Test {
                        divisor: 23,
                        throw_true: 2,
                        throw_false: 3,
                    },
                },
                Monkey {
                    idx: 1,
                    items: vec![
                        Item { alert_level: 54 },
                        Item { alert_level: 65 },
                        Item { alert_level: 75 },
                        Item { alert_level: 74 },
                    ],
                    operation: Expression {
                        lhs: Old,
                        operation: Add,
                        rhs: Literal(6),
                    },
                    test: Test {
                        divisor: 19,
                        throw_true: 2,
                        throw_false: 0,
                    },
                },
                Monkey {
                    idx: 2,
                    items: vec![
                        Item { alert_level: 79 },
                        Item { alert_level: 60 },
                        Item { alert_level: 97 },
                    ],
                    operation: Expression {
                        lhs: Old,
                        operation: Multiply,
                        rhs: Old,
                    },
                    test: Test {
                        divisor: 13,
                        throw_true: 1,
                        throw_false: 3,
                    },
                },
                Monkey {
                    idx: 3,
                    items: vec![Item { alert_level: 74 }],
                    operation: Expression {
                        lhs: Old,
                        operation: Add,
                        rhs: Literal(3),
                    },
                    test: Test {
                        divisor: 17,
                        throw_true: 0,
                        throw_false: 1,
                    },
                },
            ],
        ));
        let res = parser::parse_monkeys(&input)
            .finish()
            .map_err(|e| e.to_string());

        assert_eq!(res, expected);
    }
    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Ok(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Ok(2713310158));
    }
}
