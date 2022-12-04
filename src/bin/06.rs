use std::usize;

#[derive(Debug, Clone)]
struct FoldState<S, const C: usize> {
    buffer: [S; C],
    position: usize,
}

impl<S, const C: usize> FoldState<S, C>
where
    S: Sized + Default + Copy + Eq + std::fmt::Debug,
{
    fn zero() -> Self {
        FoldState {
            buffer: [S::default(); C],
            position: 0,
        }
    }

    fn found(&mut self, c: S) -> bool {
        self.rotate_buffer(c);

        self.position += 1;

        if self.position >= C && self.distinct() {
            return true;
        }

        false
    }

    fn distinct(&self) -> bool {
        for i in 0.. C{
            for j in 0..C {
                if j != i && self.buffer[i] == self.buffer[j] {
                    return false;
                }
            }
        }
        true
    }

    fn rotate_buffer(&mut self, c: S) {
        for x in 1..C {
            self.buffer[x - 1] = self.buffer[x];
        }
        self.buffer[C-1] = c;
    }
}

pub fn part_one(input: &str) -> Result<u32, String> {
    let mut state: FoldState<char, 4> = FoldState::<char, 4>::zero();

    for c in input.chars() {
        if state.found(c) {
            println!();
            return Result::Ok(state.position as u32);
        }
    }

    Result::Err("Not found".to_string())
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let mut state: FoldState<char, 14> = FoldState::<char, 14>::zero();

    for c in input.chars() {
        if state.found(c) {
            println!();
            return Result::Ok(state.position as u32);
        }
    }

    Err("Not solved".to_string())
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 6);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 6);

        let result: Result<Vec<u32>, String> = input.lines().take(4).map(part_one).collect();
        let expected: Vec<u32> = vec![5, 6, 10, 11];

        assert_eq!(result, Ok(expected));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 6);

        let result: Result<Vec<u32>, String> = input.lines().skip(4).map(part_two).collect();
        let expected: Vec<u32> = vec![19, 23, 23, 29, 26];

        assert_eq!(result, Ok(expected));
    }
}
