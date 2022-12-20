use std::{
    fmt::Display,
    ops::{Add, Mul},
};

pub struct Grid {
    data: Vec<u32>,
    height: usize,
    width: usize,
}

#[derive(Clone, Copy)]
enum Direction {
    East,
    North,
    West,
    South,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
pub struct Coordinates((usize, usize));

pub enum GridError {
    AllocationFailed,
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = self.width;
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                f.write_str(" ")?;
                f.write_str(&self.get(Coordinates((x, y))).to_string())?;
                f.write_str(" ")?;
            }
            f.write_str("\n")?;
        }

        std::fmt::Result::Ok(())
    }
}

impl Grid {
    fn from(vec: Vec<u32>, width: usize, height: usize) -> Grid {
        Grid {
            data: vec,
            height,
            width,
        }
    }

    pub fn new(width: usize, height: usize) -> Result<Grid, GridError> {
        let length = width
            .checked_mul(height)
            .ok_or(GridError::AllocationFailed)?;

        let vector: Vec<u32> = vec![0; length];

        Ok(Grid {
            data: vector,
            height,
            width,
        })
    }

    #[inline(always)]
    pub fn set(&mut self, coords: Coordinates, value: u32) {
        let Coordinates((x, y)) = coords;
        let index = y.mul(self.height).add(x);
        self.data[index] = value;
    }

    #[inline(always)]
    fn get(&self, coords: Coordinates) -> u32 {
        let Coordinates((x, y)) = coords;
        let index = y.mul(self.height).add(x);
        self.data[index]
    }

    fn print_highlight(&self, subject: &Coordinates, evaluating: &Coordinates) {
        if true {
            return;
        }

        for y in 0..self.height {
            for x in 0..self.width {
                if *subject == Coordinates((x, y)) {
                    print!("\x1b[7m{}\x1b[0m", self.get(Coordinates((x, y))));
                } else if *evaluating == Coordinates((x, y)) {
                    print!("\x1b[4m{}\x1b[0m", self.get(Coordinates((x, y))));
                } else {
                    print!("{}", self.get(Coordinates((x, y))));
                }
            }
            println!()
        }
        println!()
    }

    fn scenic_score(&self, subject: Coordinates) -> u32 {
        let directions: Vec<&Direction> = vec![
            &Direction::North,
            &Direction::East,
            &Direction::West,
            &Direction::South,
        ];

        let mut scenic_score = 1;
        for direction in &directions {
            let (_, score) =
                self.cast_ray(subject, **direction, |subject, current| current >= subject);
            scenic_score *= score;
        }

        scenic_score
    }

    fn cast_ray(
        &self,
        subject: Coordinates,
        direction: Direction,
        stop: fn(u32, u32) -> bool,
    ) -> (bool, u32) {
        match direction {
            Direction::East => self.cast_east(subject, stop),
            Direction::North => self.cast_north(subject, stop),
            Direction::West => self.cast_west(subject, stop),
            Direction::South => self.cast_south(subject, stop),
        }
    }

    fn cast_west(&self, subject: Coordinates, stop: fn(u32, u32) -> bool) -> (bool, u32) {
        let Coordinates((x, y)) = subject;
        let subject_height = self.get(subject);

        let mut steps = 0;

        for u in 0..x {
            let evaluating = Coordinates((x - u - 1, y));

            self.print_highlight(&subject, &evaluating);

            let current = self.get(evaluating);

            if stop(subject_height, current) {
                return (false, steps + 1);
            }
            steps += 1;
        }

        (true, steps)
    }

    fn cast_north(&self, subject: Coordinates, stop: fn(u32, u32) -> bool) -> (bool, u32) {
        let Coordinates((x, y)) = subject;
        let subject_height = self.get(subject);

        let mut steps = 0;

        for v in 0..y {
            let evaluating = Coordinates((x, y - v - 1));

            self.print_highlight(&subject, &evaluating);

            let current = self.get(evaluating);

            if stop(subject_height, current) {
                return (false, steps + 1);
            }
            steps += 1;
        }
        (true, steps)
    }

    fn cast_east(&self, subject: Coordinates, stop: fn(u32, u32) -> bool) -> (bool, u32) {
        let Coordinates((x, y)) = subject;
        let subject_height = self.get(subject);

        let mut steps = 0;

        for u in (x + 1)..self.width {
            let evaluating = Coordinates((u, y));

            self.print_highlight(&subject, &evaluating);

            let current = self.get(evaluating);

            if stop(subject_height, current) {
                return (false, steps + 1);
            }
            steps += 1;
        }

        (true, steps)
    }

    fn cast_south(&self, subject: Coordinates, stop: fn(u32, u32) -> bool) -> (bool, u32) {
        let Coordinates((x, y)) = subject;
        let subject_height = self.get(subject);

        let mut steps = 0;

        for v in (y + 1)..self.height {
            let evaluating = Coordinates((x, v));

            self.print_highlight(&subject, &evaluating);

            let current = self.get(evaluating);

            if stop(subject_height, current) {
                return (false, steps + 1);
            }
            steps += 1;
        }

        (true, steps)
    }

    fn load(input: &str) -> Result<(Vec<u32>, (usize, usize)), String> {
        let mut data: Vec<u32> = Vec::new();
        let mut width: usize = 0;
        let mut height: usize = 0;

        let mut len = 0;
        for c in input.chars() {
            len += 1;
            if c.is_whitespace() {
                if width == 0 {
                    width = len;
                }
                height += 1;
                continue;
            }

            data.push(c.to_string().parse::<u32>().map_err(|x| x.to_string())?);
        }

        Ok((data, (width - 1, height + 1)))
    }
}

pub fn part_one(input: &str) -> Result<u32, String> {
    let (data, (width, height)) = Grid::load(input)?;
    let grid = Grid::from(data, width, height);

    //    println!("{}", grid);
    //    println!("{}, {}", width, height);

    let mut visible_count = 0;

    let directions: Vec<&Direction> = vec![
        &Direction::North,
        &Direction::East,
        &Direction::West,
        &Direction::South,
    ];

    for (x, y) in (0..width).flat_map(|x| (0..height).map(move |y| (x, y))) {
        for direction in &directions {
            let (visible, _steps) =
                grid.cast_ray(Coordinates((x, y)), **direction, |subject, current| {
                    current >= subject
                });

            if visible {
                visible_count += 1;
                break;
            }
        }
    }

    Ok(visible_count)
}

pub fn part_two(input: &str) -> Result<u32, String> {
    let (data, (width, height)) = Grid::load(input)?;
    let grid = Grid::from(data, width, height);

    let mut max_scenic_score = 0;
    //    println!("{}", grid);
    //    println!("{}, {}", width, height);
    for (x, y) in (0..width).flat_map(|x| (0..height).map(move |y| (x, y))) {
        let scenic_score = grid.scenic_score(Coordinates((x, y)));

        max_scenic_score = if scenic_score > max_scenic_score {
            scenic_score
        } else {
            max_scenic_score
        }
    }
    Ok(max_scenic_score)
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 8);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cast_east() {
        let input = "30373
25512
65332
33549
35390";
        let (data, (width, height)) = Grid::load(input).unwrap();

        let grid = Grid::from(data, width, height);

        let res = grid.cast_east(Coordinates((2, 3)), |subject, current| current >= subject);

        assert_eq!(res, (false, 2));

        let res = grid.cast_east(Coordinates((2, 2)), |subject, current| current >= subject);
        assert_eq!(res, (false, 1));

        let res = grid.cast_east(Coordinates((3, 4)), |subject, current| current >= subject);

        assert_eq!(res, (true, 1));
    }

    #[test]
    fn test_cast_west() {
        let input = "30373
25512
65332
33549
35390";
        let (data, (width, height)) = Grid::load(input).unwrap();

        let grid = Grid::from(data, width, height);

        let res = grid.cast_west(Coordinates((0, 0)), |subject, current| current >= subject);

        assert_eq!(res, (true, 0));
        let res = grid.cast_west(Coordinates((0, 2)), |subject, current| current >= subject);

        assert_eq!(res, (true, 0));
    }

    #[test]
    fn test_cast_north() {
        let input = "30373
25512
65332
33549
35390";
        let (data, (width, height)) = Grid::load(input).unwrap();

        let grid = Grid::from(data, width, height);

        let res = grid.cast_north(Coordinates((2, 1)), |subject, current| current >= subject);

        assert_eq!(res, (true, 1));
        let res = grid.cast_north(Coordinates((0, 2)), |subject, current| current >= subject);

        assert_eq!(res, (true, 2));
    }

    #[test]
    fn test_cast_south() {
        let input = "30373
25512
65332
33549
35390";
        let (data, (width, height)) = Grid::load(input).unwrap();

        let grid = Grid::from(data, width, height);

        let res = grid.cast_south(Coordinates((2, 1)), |subject, current| current >= subject);

        assert_eq!(res, (false, 2));
        let res = grid.cast_south(Coordinates((0, 2)), |subject, current| current >= subject);

        assert_eq!(res, (true, 2));
    }

    #[test]
    fn test_scenic_score() {
        let input = "30373
25512
65332
33549
35390";
        let (data, (width, height)) = Grid::load(input).unwrap();

        let grid = Grid::from(data, width, height);

        let res = grid.scenic_score(Coordinates((2, 3)));

        assert_eq!(res, 8);

        let res = grid.scenic_score(Coordinates((2, 1)));

        assert_eq!(res, 4);
    }

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_one(&input), Ok(21));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 8);
        assert_eq!(part_two(&input), Ok(8));
    }
}
