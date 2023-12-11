use ndarray::Axis;
use std::collections::HashSet;
advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<usize> {
    Some(solve(input, 1))
}

pub fn part_two(input: &str) -> Option<usize> {
    Some(solve(input, 1000000 - 1))
}

fn solve(input: &str, spacing: u32) -> usize {
    let lines = input.lines().collect::<Vec<_>>();
    let x_length = lines.first().unwrap().len();
    let y_length = lines.len();
    let mut grid = ndarray::Array2::<char>::default((y_length, x_length));
    let mut galaxies = vec![];
    for (i, mut row) in grid.axis_iter_mut(Axis(0)).enumerate() {
        for (j, col) in row.iter_mut().enumerate() {
            let c = lines.get(i).unwrap().chars().nth(j).unwrap();
            if c == '#' {
                galaxies.push((j, i))
            }
            *col = c;
        }
    }

    let mut row_has_galaxy = HashSet::<usize>::new();
    let mut col_has_galaxy = HashSet::<usize>::new();
    for (i, row) in grid.axis_iter(Axis(0)).enumerate() {
        for (j, col) in row.iter().enumerate() {
            if col == &'#' {
                row_has_galaxy.insert(i);
                col_has_galaxy.insert(j);
            }
        }
    }
    let mut sum = 0;
    let mut seen = HashSet::<((usize, usize), (usize, usize))>::new();
    galaxies.iter().for_each(|g| {
        galaxies.iter().for_each(|g2| {
            if &g != &g2 && !seen.contains(&(g.clone(), g2.clone())) {
                let d = distance(g, g2, &row_has_galaxy, &col_has_galaxy, spacing as usize);
                seen.insert((g.clone(), g2.clone()));
                seen.insert((g2.clone(), g.clone()));
                sum += d;
            }
        })
    });
    sum
}

fn distance(
    one: &(usize, usize),
    two: &(usize, usize),
    row_has_galaxy: &HashSet<usize>,
    col_has_galaxy: &HashSet<usize>,
    spacing: usize,
) -> usize {
    let rows = one.1.min(two.1)..=two.1.max(one.1);
    let mut rows_augment = 0;
    rows.for_each(|r| {
        if !row_has_galaxy.contains(&r) {
            rows_augment += spacing;
        }
    });
    let cols = one.0.min(two.0)..=two.0.max(one.0);
    let mut cols_augment = 0;
    cols.for_each(|c| {
        if !col_has_galaxy.contains(&c) {
            cols_augment += spacing;
        }
    });
    return (one.0).abs_diff(two.0) + (one.1).abs_diff(two.1) + (rows_augment + cols_augment);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_one_a() {
        let row_has_g = HashSet::<usize>::from_iter(vec![2, 5, 4, 6, 1, 9, 8, 0]);
        let col_has_g = HashSet::<usize>::from_iter(vec![3, 0, 6, 1, 9, 7, 4]);

        // let d = distance((1, 5), (4, 9), &row_has_g, &col_has_g);
        // assert_eq!(d, 9);

        let d = distance(&(3, 0), &(7, 8), &row_has_g, &col_has_g, 1);
        assert_eq!(d, 15);
        // assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = solve(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            9,
        );
        assert_eq!(result, 1030usize);
    }

    #[test]
    fn test_part_two_b() {
        let result = solve(
            "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....",
            99,
        );
        assert_eq!(result, 8410usize);
    }
}
