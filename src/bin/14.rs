use crate::Dir::{East, North, South, West};
use ndarray::{Array2, Axis};
use std::collections::HashSet;
use std::fmt;
use std::ops::Index;
use std::panic::panic_any;
advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<_>>();
    let mut grid = Array2::<char>::default((lines.len(), lines[0].len()));
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
    let diff = match dir {
        North => (0, -1),
        South => (0, 1),
        East => (1, 0),
        West => (-1, 0),
    };
    round_rocks.sort_by(|(ax, ay), (bx, by)| {
        return match dir {
            North => ay.cmp(by),
            South => by.cmp(ay),
            East => bx.cmp(ax),
            West => ax.cmp(bx),
        };
    });
    round_rocks.iter().for_each(|(x, y)| {
        let mut curr_y = *y as i32 + diff.1;
        let mut curr_x = *x as i32 + diff.0;
        while grid.get((curr_y as usize, curr_x as usize)).unwrap_or(&'#') == &'.' {
            curr_y += diff.1;
            curr_x += diff.0;
        }
        curr_y -= diff.1;
        curr_x -= diff.0;

        *grid.get_mut((y.clone(), x.clone())).unwrap() = '.';
        *grid.get_mut((curr_y as usize, curr_x as usize)).unwrap() = 'O';
    });
}

pub fn part_two(input: &str) -> Option<usize> {
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
    let mut seen = Vec::<String>::new();
    let mut ans_map = Vec::<usize>::new();
    let mut steps = 0;
    let (cycle_start, steps, p) = loop {
        let curr_dir = &dirs[i];

        simulate(curr_dir, &mut grid);
        if curr_dir == &East {
            ans_map.push(score(&grid));
            match seen
                .iter()
                .enumerate()
                .find(|(cycle_start, p)| p == &&(curr_dir.to_string() + &grid.stringify()))
            {
                None => {}
                Some((cycle_start, p)) => {
                    break (cycle_start, steps, p);
                }
            }

            seen.push(curr_dir.to_string() + &grid.stringify());
            steps += 1;
        }

        i += 1;
        if i > 3 {
            i = 0;
        }
    };
    let cycle_length = steps - cycle_start;
    let target = 1000000000usize;

    let target_pos_in_cycle = (target - cycle_start) % cycle_length;
    let answer_pos = cycle_start + target_pos_in_cycle - 1;

    Some(ans_map[answer_pos])
}

trait Stringify {
    fn stringify(&self) -> String;
}
impl Stringify for Array2<char> {
    fn stringify(&self) -> String {
        let mut s = Vec::<char>::new();
        for (i, mut row) in self.axis_iter(Axis(0)).enumerate() {
            for (j, col) in row.iter().enumerate() {
                s.push(*col);
            }
        }
        return s.iter().collect();
    }
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
        assert_eq!(result, Some(64));
    }
}
