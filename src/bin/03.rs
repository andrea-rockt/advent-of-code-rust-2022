use itertools::Itertools;
use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::collections::HashSet;
use std::{collections::hash_map::RandomState, hash::Hash};

pub fn part_one(input: &str) -> Result<u32, String> {
    use parser::*;

    let res: Result<u32, RucksackParsingError> = input
        .lines()
        .map(|l| l.parse::<Rucksack>().map(|r| calculate_priorities(&r)))
        .fold_while(Ok(0), |acc, elem| match elem {
            Err(err) => itertools::FoldWhile::Done(Err(err)),
            Ok(i) => itertools::FoldWhile::Continue(acc.map(|a| a + i)),
        })
        .into_inner();

    res.map_err(|e| e.to_string())
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let res: Result<Vec<Rucksack>, parser::RucksackParsingError> =
        input.lines().map(|l| l.parse::<Rucksack>()).collect();

    res.map(|rucksacks| {
        rucksacks
            .iter()
            .batching(|sub| {
                let res: Vec<&Rucksack> = sub.take(3).collect();
                if res.is_empty() {
                    None
                } else {
                    let res2 = calculate_badge_priorities(&res);
                    Some(res2)
                }
            })
            .sum()
    })
    .map_err(|x| x.to_string())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 3);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, Hash)]
struct Item {
    id: char,
}

#[derive(PartialEq, Debug)]
struct Rucksack {
    first_compartment: Vec<Item>,
    second_compartment: Vec<Item>,
}

static PRIORITIES: Lazy<HashMap<char, u32, RandomState>> = Lazy::new(|| {
    ('a'..='z')
        .chain('A'..='Z')
        .enumerate()
        .map(|(idx, char)| (char, (idx + 1) as u32))
        .collect()
});

fn calculate_priorities(rucksack: &Rucksack) -> u32 {
    let unique_in_first: HashSet<&Item> = HashSet::from_iter(rucksack.first_compartment.iter());
    let unique_in_second: HashSet<&Item> = HashSet::from_iter(rucksack.second_compartment.iter());

    let unique: HashSet<&&Item> = unique_in_first.intersection(&unique_in_second).collect();

    unique
        .iter()
        .fold(0, |acc, e| acc + *PRIORITIES.get(&e.id).unwrap_or(&0))
}

fn calculate_badge_priorities(rucksack: &[&Rucksack]) -> u32 {
    let res: Option<HashSet<&Item>> = rucksack
        .iter()
        .map(|r| {
            HashSet::from_iter(
                r.first_compartment
                    .iter()
                    .chain(r.second_compartment.iter()),
            )
        })
        .reduce(|l: HashSet<&Item>, r: HashSet<&Item>| l.intersection(&r).copied().collect());

    res.map(|x| {
        x.iter()
            .fold(0, |acc, e| acc + *PRIORITIES.get(&e.id).unwrap_or(&0))
    })
    .unwrap_or(0)
}

mod parser {

    use crate::Rucksack;

    use super::Item;
    use itertools::Itertools;
    use once_cell::sync::Lazy;
    use std::collections::HashSet;
    use std::str::FromStr;

    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum ItemParsingError {
        MoreThanOneCharacter { original_input: String },
        CharacterNotAllowed { original_input: String },
        Empty,
    }
    #[derive(PartialEq, Eq, Debug, Clone)]
    pub enum RucksackParsingError {
        UnbalancedItems { original_input: String },
        InvalidItems { invalid: Vec<ItemParsingError> },
    }

    impl ToString for ItemParsingError {
        fn to_string(&self) -> String {
            match self {
                ItemParsingError::MoreThanOneCharacter { original_input } => format!(
                    "More than one character supplied while parsing item [{}]",
                    original_input
                ),
                ItemParsingError::CharacterNotAllowed { original_input } => format!(
                    "Found a character not allowed while parsing item [{}], allowed [{}]",
                    original_input,
                    ALLOWED_ITEM_CHARS.iter().join(",")
                ),
                ItemParsingError::Empty => "Cannot parse item from empty string".to_string(),
            }
        }
    }

    impl ToString for RucksackParsingError {
        fn to_string(&self) -> String {
            match self {
                RucksackParsingError::UnbalancedItems { original_input } => format!(
                    "Items per line should be even, supplied: {}",
                    original_input,
                ),
                RucksackParsingError::InvalidItems { invalid } => {
                    let inner_error = invalid.iter().map(|e| e.to_string()).join(",");
                    format!(
                        "Some items in rucksack where not parsable [{}]",
                        inner_error
                    )
                }
            }
        }
    }

    impl FromStr for Rucksack {
        type Err = RucksackParsingError;

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            if s.len() % 2 == 1 {
                return Err(RucksackParsingError::UnbalancedItems {
                    original_input: s.to_string(),
                });
            }

            let (success, failures): (Vec<Item>, Vec<ItemParsingError>) = s
                .chars()
                .map(|c| c.to_string().parse::<Item>())
                .partition_result();

            if !failures.is_empty() {
                return Err(RucksackParsingError::InvalidItems { invalid: failures });
            }

            Ok(Rucksack {
                first_compartment: success[..success.len() / 2].to_vec(),
                second_compartment: success[success.len() / 2..].to_vec(),
            })
        }
    }

    pub static ALLOWED_ITEM_CHARS: Lazy<HashSet<char>> =
        Lazy::new(|| ('a'..='z').chain('A'..='Z').collect());

    impl FromStr for Item {
        type Err = ItemParsingError;

        fn from_str(input: &str) -> Result<Item, ItemParsingError> {
            if input.len() > 1 {
                return Err(ItemParsingError::MoreThanOneCharacter {
                    original_input: input.to_string(),
                });
            }

            let maybe_head = input.chars().next();

            if let Some(head) = maybe_head {
                if ALLOWED_ITEM_CHARS.contains(&head) {
                    Ok(Item { id: head })
                } else {
                    Err(ItemParsingError::CharacterNotAllowed {
                        original_input: input.to_string(),
                    })
                }
            } else {
                Err(ItemParsingError::Empty)
            }
        }
    }

    #[test]
    fn test_parse_item() {
        ALLOWED_ITEM_CHARS.iter().for_each(|char| {
            let parsed = char.to_string().parse::<Item>();

            assert_eq!(parsed, Ok(Item { id: *char }));
        });

        ('1'..='9').for_each(|char| {
            let parsed = char.to_string().parse::<Item>();

            assert_eq!(
                parsed,
                Err(ItemParsingError::CharacterNotAllowed {
                    original_input: char.to_string()
                })
            )
        });

        let empty = "".parse::<Item>();
        assert_eq!(empty, Err(ItemParsingError::Empty));

        let more_than_one_char = "abc".parse::<Item>();
        assert_eq!(
            more_than_one_char,
            Err(ItemParsingError::MoreThanOneCharacter {
                original_input: "abc".to_string()
            })
        );
    }

    #[test]
    fn test_parse_rucksack() {
        let example = "vJrwpWtwJgWrhcsFMMfFFhFp";

        let result = example.parse::<Rucksack>();

        assert_eq!(
            result,
            Ok(Rucksack {
                first_compartment: vec![
                    Item { id: 'v' },
                    Item { id: 'J' },
                    Item { id: 'r' },
                    Item { id: 'w' },
                    Item { id: 'p' },
                    Item { id: 'W' },
                    Item { id: 't' },
                    Item { id: 'w' },
                    Item { id: 'J' },
                    Item { id: 'g' },
                    Item { id: 'W' },
                    Item { id: 'r' }
                ],
                second_compartment: vec![
                    Item { id: 'h' },
                    Item { id: 'c' },
                    Item { id: 's' },
                    Item { id: 'F' },
                    Item { id: 'M' },
                    Item { id: 'M' },
                    Item { id: 'f' },
                    Item { id: 'F' },
                    Item { id: 'F' },
                    Item { id: 'h' },
                    Item { id: 'F' },
                    Item { id: 'p' }
                ]
            })
        );

        let unbalanced_example = "vJrwpWtwJgWrhcsFMMfFFhF";

        let unbalanced_result = unbalanced_example.parse::<Rucksack>();

        assert_eq!(
            unbalanced_result,
            Err(RucksackParsingError::UnbalancedItems {
                original_input: unbalanced_example.to_string()
            })
        );

        let invalid_id_example = "v3rwpWtwJgWrhcsFMMfFFhFp";

        let invalid_id_result = invalid_id_example.parse::<Rucksack>();

        assert_eq!(
            invalid_id_result,
            Err(RucksackParsingError::InvalidItems {
                invalid: vec![ItemParsingError::CharacterNotAllowed {
                    original_input: "3".to_string()
                }]
            })
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_one(&input), Ok(157));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 3);
        assert_eq!(part_two(&input), Ok(70));
    }
}
