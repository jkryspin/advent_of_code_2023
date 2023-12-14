use crate::Dir::{East, North, South, West};
use ndarray::{Array2, Axis};
use std::collections::HashSet;
use std::fmt;
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut grid = ndarray::Array2::<char>::default((lines.len(), lines[0].len()));
    for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            let c = lines.get(i).unwrap().chars().nth(j).unwrap();
            *col = c;
        }
    }
    simulate(&North, &mut grid);
    Some(score(&grid))
}

#[derive(Eq, PartialOrd, PartialEq, Debug)]
enum Dir {
    North,
    South,
    East,
    West,
}
impl fmt::Display for Dir {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
fn score(grid: &Array2<char>) -> usize {
    let mut score = 0;
    for (i, row) in grid.axis_iter(Axis(0)).enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'O' {
                score += grid.len_of(Axis(0)) - i;
            }
        }
    }
    score
}

fn simulate(dir: &Dir, grid: &mut Array2<char>) {
    let mut round_rocks = vec![];
    let mut cube_rocks = vec![];
    for (i, row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, c) in row.iter().enumerate() {
            if c == &'O' {
                round_rocks.push((j, i))
            }

            if c == &'#' {
                cube_rocks.push((j, i))
            }
        }
    }
    round_rocks.iter().for_each(|(x, y)| {
        let mut curr_y = *y as i32 - 1i32;
        while grid.get((curr_y as usize, x.clone())).unwrap_or(&'#') == &'.' {
            curr_y -= 1;
        }
        curr_y += 1;

        *grid.get_mut((y.clone(), x.clone())).unwrap() = '.';
        *grid.get_mut((curr_y as usize, x.clone())).unwrap() = 'O';
    });
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut grid = ndarray::Array2::<char>::default((lines.len(), lines[0].len()));
    for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            let c = lines.get(i).unwrap().chars().nth(j).unwrap();
            *col = c;
        }
    }
    let dirs = [North, West, South, East];
    let mut i = 0;
    let seen = HashSet::<String>::new();
    let mut steps = 0;
    loop {
        let curr_dir = &dirs[i];
        curr_dir.to_string();
        if seen.contains((curr_dir.to_string() + &grid.to_string()).as_str()) {
            dbg!(curr_dir, steps);
            panic!("found cycle!");
        }

        simulate(curr_dir, &mut grid);
        i += 1;
        if i > 3 {
            i = 0;
        }
        steps += 1;
    }
    Some(score(&grid));
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
