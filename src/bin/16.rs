use crate::EnteredFrom::{East, North, South, West};
use ndarray::{Array2, Axis};
use std::collections::{HashMap, HashSet, VecDeque};
advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<usize> {
    let grid = create_grid(input.lines().collect());
    println!("{}", grid);
    let mut seen_count = HashMap::<(i32, i32), usize>::new();
    let mut queue = VecDeque::<(i32, i32, EnteredFrom)>::new();
    let mut already_hit = HashSet::<(i32, i32, EnteredFrom)>::new();
    queue.push_back((0, 0, West));
    while let Some((x, y, dir)) = queue.pop_back() {
        if y < 0
            || x < 0
            || grid.get((y as usize, x as usize)).is_none()
            || already_hit.contains(&(x, y, dir.clone()))
        {
            continue;
        }
        already_hit.insert((x, y, dir.clone()));
        if let Some(seen) = seen_count.get_mut(&(x, y)) {
            *seen += 1;
        } else {
            seen_count.insert((x, y), 1);
        }

        // Check for if none
        if let Some(c) = grid.get((y as usize, x as usize)) {
            match c {
                '.' => {
                    queue.push_back(match dir {
                        EnteredFrom::North => (x, y + 1, dir),
                        EnteredFrom::South => (x, y - 1, dir),
                        EnteredFrom::East => (x - 1, y, dir),
                        EnteredFrom::West => (x + 1, y, dir),
                    });
                }
                '\\' => queue.push_back(match dir {
                    EnteredFrom::North => (x + 1, y, West),
                    EnteredFrom::South => (x - 1, y, East),
                    EnteredFrom::East => (x, y - 1, South),
                    EnteredFrom::West => (x, y + 1, North),
                }),
                '/' => {
                    queue.push_back(match dir {
                        North => (x - 1, y, East),
                        South => (x + 1, y, West),
                        East => (x, y + 1, North),
                        West => (x, y - 1, South),
                    });
                }
                '-' => match dir {
                    North => {
                        queue.push_back((x + 1, y, West));
                        queue.push_back((x - 1, y, East));
                    }
                    South => {
                        queue.push_back((x + 1, y, West));
                        queue.push_back((x - 1, y, East));
                    }
                    East => {
                        queue.push_back((x - 1, y, dir));
                    }
                    West => {
                        queue.push_back((x + 1, y, dir));
                    }
                },
                '|' => match dir {
                    North => {
                        queue.push_back((x, y + 1, dir));
                    }
                    South => queue.push_back((x, y - 1, dir)),
                    East => {
                        queue.push_back((x, y - 1, South));
                        queue.push_back((x, y + 1, North));
                    }
                    West => {
                        queue.push_back((x, y - 1, South));
                        queue.push_back((x, y + 1, North));
                    }
                },
                x => {
                    panic!("{} Not mapped!!!", x)
                }
            }
        }
    }

    Some(seen_count.len())
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
enum EnteredFrom {
    North,
    South,
    East,
    West,
}

fn create_grid(lines: Vec<&str>) -> Array2<char> {
    let mut grid = ndarray::Array2::<char>::default((lines.len(), lines[0].len()));
    for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            let c = lines.get(i).unwrap().chars().nth(j).unwrap();
            *col = c;
        }
    }
    return grid;
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
