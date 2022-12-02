use itertools::{FoldWhile, Itertools};
/// \--- Day 1: Calorie Counting ---
/// ----------
/// 
/// Santa's reindeer typically eat regular reindeer food, but they need a lot of [magical energy](/2018/day/25) to deliver presents on Christmas. For that, their favorite snack is a special type of *star* fruit that only grows deep in the jungle. The Elves have brought you on their annual expedition to the grove where the fruit grows.
/// 
/// To supply enough magical energy, the expedition needs to retrieve a minimum of *fifty stars* by December 25th. Although the Elves assure you that the grove has plenty of fruit, you decide to grab any fruit you see along the way, just in case.
/// 
/// Collect stars by solving puzzles. Two puzzles will be made available on each day in the Advent calendar; the second puzzle is unlocked when you complete the first. Each puzzle grants *one star*. Good luck!
/// 
/// The jungle must be too overgrown and difficult to navigate in vehicles or access from the air; the Elves' expedition traditionally goes on foot. As your boats approach land, the Elves begin taking inventory of their supplies. One important consideration is food - in particular, the number of *Calories* each Elf is carrying (your puzzle input).
/// 
/// The Elves take turns writing down the number of Calories contained by the various meals, snacks, rations, etc. that they've brought with them, one item per line. Each Elf separates their own inventory from the previous Elf's inventory (if any) by a blank line.
/// 
/// For example, suppose the Elves finish writing their items' Calories and end up with the following list:
/// 
/// ```
/// 1000
/// 2000
/// 3000
/// 
/// 4000
/// 
/// 5000
/// 6000
/// 
/// 7000
/// 8000
/// 9000
/// 
/// 10000
/// 
/// ```
/// 
/// This list represents the Calories of the food carried by five Elves:
/// 
/// * The first Elf is carrying food with `1000`, `2000`, and `3000` Calories, a total of `*6000*` Calories.
/// * The second Elf is carrying one food item with `*4000*` Calories.
/// * The third Elf is carrying food with `5000` and `6000` Calories, a total of `*11000*` Calories.
/// * The fourth Elf is carrying food with `7000`, `8000`, and `9000` Calories, a total of `*24000*` Calories.
/// * The fifth Elf is carrying one food item with `*10000*` Calories.
/// 
/// In case the Elves get hungry and need extra snacks, they need to know which Elf to ask: they'd like to know how many Calories are being carried by the Elf carrying the *most* Calories. In the example above, this is *`24000`* (carried by the fourth Elf).
/// 
/// Find the Elf carrying the most Calories. *How many total Calories is that Elf carrying?*
/// 
/// Your puzzle answer was `69177`.
/// 
/// \--- Part Two ---
/// ----------
/// 
/// By the time you calculate the answer to the Elves' question, they've already realized that the Elf carrying the most Calories of food might eventually *run out of snacks*.
/// 
/// To avoid this unacceptable situation, the Elves would instead like to know the total Calories carried by the *top three* Elves carrying the most Calories. That way, even if one of those Elves runs out of snacks, they still have two backups.
/// 
/// In the example above, the top three Elves are the fourth Elf (with `24000` Calories), then the third Elf (with `11000` Calories), then the fifth Elf (with `10000` Calories). The sum of the Calories carried by these three elves is `*45000*`.
/// 
/// Find the top three Elves carrying the most Calories. *How many Calories are those Elves carrying in total?*
/// 
/// Your puzzle answer was `207456`.
/// 
/// Both parts of this puzzle are complete! They provide two gold stars: \*\*
/// 
/// At this point, you should [return to your Advent calendar](/2022) and try another puzzle.
/// 
/// If you still want to see it, you can [get your puzzle input](1/input).
/// 
/// You can also [Shareon [Twitter](https://twitter.com/intent/tweet?text=I%27ve+completed+%22Calorie+Counting%22+%2D+Day+1+%2D+Advent+of+Code+2022&url=https%3A%2F%2Fadventofcode%2Ecom%2F2022%2Fday%2F1&related=ericwastl&hashtags=AdventOfCode) [Mastodon](javascript:void(0);)] this puzzle.



/// Holds the folding state, the current count of calories and the max count of calories seen so far.
struct State {
    count_of_calories_for_current_elf: u32,
    count_of_calories_for_the_top_elf_so_far: u32,
}

impl State {
    /// Returns the maximum sum of calories among all elfs
    fn max_sum_of_calories(&self) -> u32 {
        self.count_of_calories_for_the_top_elf_so_far
    }

    /// Updates the calories count for the elf we are folding over
    fn update_calories_current_elf(&self, calories: u32) -> Self {
        State {
            count_of_calories_for_current_elf: self.count_of_calories_for_current_elf + calories,
            ..*self
        }
    }

    /// Signal that the current elf we were summing calories
    /// for has no more snacks and updates the max sum of calories seen so far
    fn finish_current_elf(&self) -> Self {
        State {
            count_of_calories_for_the_top_elf_so_far: std::cmp::max(
                self.count_of_calories_for_the_top_elf_so_far,
                self.count_of_calories_for_current_elf,
            ),
            count_of_calories_for_current_elf: 0,
        }
    }

    /// Initial state of the fold, the maximum sum of calories so far is 0 for the current elf,
    /// the global maximum amount of calories held by an elf starts at zero.
    fn zero() -> State {
        State {
            count_of_calories_for_current_elf: 0,
            count_of_calories_for_the_top_elf_so_far: 0,
        }
    }
}

/// Implements a somewhat naive fold, it iterates over all lines and
///  * if the line is not empty: updates the calories sum for snacks held by the current elf
///  * if the line is empty: updates the maximum value seen so far
///
/// Notes:
///
/// * if we encounter a parsing error (a line is not parsable as a u32) instead of panic we coerce the error
///   to zero amount of calories
pub fn part_one(input: &str) -> Result<u32, String> {
    Ok(
        input
            .lines()
            .fold(State::zero(), |acc, elem| {
                if elem.is_empty() {
                    acc.finish_current_elf()
                } else {
                    acc.update_calories_current_elf(elem.parse().unwrap_or(0))
                }
            })
            .max_sum_of_calories(),
    )
}

/// Generate a FoldWhile closure that:
///
/// * runs the supplied closure on *non_empty* lines and continue the FoldWhile
/// * stops the FoldWhile and returns the accumulator when an empty line is found
fn while_not_empty<G, A>(h: G) -> impl Fn(A, &str) -> FoldWhile<A>
where
    G: Fn(A, &str) -> A,
{
    // we need to move the supplied closure
    // inside the generated closure scope since we will capture `h`
    move |acc, str: &str| {
        if str.is_empty() {
            FoldWhile::Done(acc)
        } else {
            FoldWhile::Continue(h(acc, str))
        }
    }
}

/// Generate a FoldWhile closure that parses all values as i32 and sums them until an empty line is found
///
/// Notes:
///
/// * if we encounter a parsing error (a line is not parsable as a u32) instead of panic we coerce the error
///   to zero amount of calories
fn sum_all_values_until_empty_line() -> impl Fn(Option<i32>, &str) -> FoldWhile<Option<i32>> {
    while_not_empty(|acc: Option<i32>, string| {
        let parsed = string.parse::<i32>().unwrap_or(0);
        acc.or(Some(0)).map(|prev_value| prev_value + parsed)
    })
}

pub fn part_two(input: &str) -> Result<u32,String> {
    let sum_of_top_three: i32 = input
        // We iterate over lines
        .lines()
        // We lazyly compute batches, we get an iterator that we can pull elements from until the batch is finished
        .batching(|lines_to_batch_every_empty_line| {
            lines_to_batch_every_empty_line
                //We pull lines from the iterator until we find an empty line
                .fold_while(None, sum_all_values_until_empty_line())
                .into_inner()
            
            //the result of this batch of lines is the sum of the calories of each snack inside the current batch (elf)
        })
        // the number of calories is positive or zero, we invert the number since we can compute efficiently only the k-smallest elements, we need the k-greatest
        .map(|i| -i)
        // We compute the k smallest, this is guaranteed to take
        // This is guaranteed to use `k * sizeof(i32) + O(1)` memory
        // and `O(n log k)` time, with `n` the number of elements in the input.
        .k_smallest(3)
        // We invert the results again since we want positive numbers
        .map(|i| -i)
        // then we sum the calories held by the top three elves as required by the puzzle
        .sum();

        //we know that number of calories is zero or positive so we can safely cast to u32
    Ok(sum_of_top_three as u32)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 1);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_one(&input), Ok(24000));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 1);
        assert_eq!(part_two(&input), Ok(45000));
    }
}
