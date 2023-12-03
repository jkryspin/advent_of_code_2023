use std::collections::{HashMap, HashSet};
use std::ops::{Add, DerefMut};
advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = create_grid(input);
    let mut sum = 0;
    grid.iter().enumerate().for_each(|(y, l)| {
        let mut i = 0;
        while i < l.len() {
            let mut number_s = "".to_string();
            let mut c = l[i];
            let start = (i, y);
            let mut end = (i, y);
            while c.is_ascii_digit() {
                end = (i, y);
                number_s.push_str(&c.to_string());
                if let Some(&x) = l.get(i + 1) {
                    i += 1;
                    c = x;
                } else {
                    break;
                }
            }
            i += 1;
            if number_s.len() > 0 {
                let positions = positions(start, end);
                if adjacent_to_symbol(positions, &grid) {
                    sum += number_s.parse::<u32>().unwrap();
                }
            }
        }
    });

    Some(sum)
}
fn positions(start: (usize, usize), end: (usize, usize)) -> Vec<(usize, usize)> {
    return (start.0..=end.0)
        .into_iter()
        .map(|x| (x, start.1))
        .collect();
}

fn create_grid(input: &str) -> Vec<Vec<char>> {
    let lines = input.lines().collect::<Vec<_>>();
    let width = lines.clone().first().unwrap().len();
    let height = lines.len();
    let mut grid_raw = vec!['.'; width * height];

    // Vector of 'width' elements slices
    let grid_base: Vec<_> = grid_raw
        .chunks_mut(width)
        .into_iter()
        .map(|x| x.to_vec())
        .collect();

    // Final 2d array `&mut [&mut [_]]`
    let mut grid = grid_base;

    // Accessing data
    for (y, l) in lines.iter().enumerate() {
        for (x, c) in l.chars().enumerate() {
            grid[y][x] = c;
        }
    }
    return grid;
}
fn adjacent_to_symbol(positions: Vec<(usize, usize)>, grid: &Vec<Vec<char>>) -> bool {
    for p in positions.iter() {
        for a in -1i32..=1 {
            for b in -1i32..=1 {
                if a == 0 && b == 0 {
                    continue;
                }
                if let Some(y) = grid.get((p.1 as i32 + a) as usize) {
                    if let Some(x) = y.get((p.0 as i32 + b) as usize) {
                        if x != &'.' && !x.is_ascii_digit() {
                            return true;
                        }
                    }
                }
            }
        }
    }
    return false;
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = create_grid(input);
    let mut cog_map = HashMap::<(usize, usize), Vec<u32>>::new();
    grid.iter().enumerate().for_each(|(y, l)| {
        let mut i = 0;
        while i < l.len() {
            let mut number_s = "".to_string();
            let mut c = l[i];
            let start = (i, y);
            let mut end = (i, y);
            while c.is_ascii_digit() {
                end = (i, y);
                number_s.push_str(&c.to_string());
                if let Some(&x) = l.get(i + 1) {
                    i += 1;
                    c = x;
                } else {
                    break;
                }
            }
            i += 1;
            if number_s.len() > 0 {
                let positions = positions(start, end);
                let cog_positions = cog_positions(positions, &grid);
                cog_positions.iter().for_each(|(x_i, y_i)| {
                    if let Some(m) = cog_map.get_mut(&(x_i.clone(), y_i.clone())) {
                        m.push(number_s.parse().unwrap());
                    } else {
                        cog_map.insert((x_i.clone(), y_i.clone()), vec![number_s.parse().unwrap()]);
                    }
                })
            }
        }
    });

    let mut sum = 0;
    cog_map.iter().for_each(|(key, val)| {
        if val.len() == 2 {
            sum += (val[0] * val[1]);
        }
    });

    Some(sum)
}
fn cog_positions(positions: Vec<(usize, usize)>, grid: &Vec<Vec<char>>) -> HashSet<(usize, usize)> {
    let mut found_positions = HashSet::<(usize, usize)>::new();
    for p in positions.iter() {
        for a in -1i32..=1 {
            for b in -1i32..=1 {
                if a == 0 && b == 0 {
                    continue;
                }
                let y_pos = p.1 as i32 + a;
                let x_pos = p.0 as i32 + b;
                if let Some(y) = grid.get((y_pos) as usize) {
                    if let Some(x) = y.get(x_pos as usize) {
                        if x == &'*' {
                            found_positions.insert((x_pos as usize, y_pos as usize));
                        }
                    }
                }
            }
        }
    }
    return found_positions;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_one_help() {
        let result = part_one(
            "12.......*..
+.........34
.......-12..
..78........
..*....60...
78..........
.......23...
....90*12...
............
2.2......12.
.*.........*
1.1.......56",
        );
        assert_eq!(result, Some(413));
    }

    #[test]
    fn test_part_one_help_2() {
        let result = part_one(
            "12.......*..
+.........34
.......-12..
..78........
..*....60...
78.........9
.5.....23..$
8...90*12...
............
2.2......12.
.*.........*
1.1..503+.56",
        );
        assert_eq!(result, Some(925));
    }

    #[test]
    fn test_part_one_help_3() {
        let result = part_one(
            "100
200",
        );
        assert_eq!(result, Some(0));
    }

    #[test]
    fn test_part_one_help_4() {
        let result = part_one("503+");
        assert_eq!(result, Some(503));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
