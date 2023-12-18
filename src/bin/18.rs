use crate::Dir::{Down, Left, Right, Up};
use itertools::Itertools;
advent_of_code::solution!(18);

pub fn part_one(input: &str) -> Option<i64> {
    let mut positions = Vec::<(i64, i64)>::new();
    let mut curr = (0, 0);
    input.lines().for_each(|l| {
        let pogs = l.split_whitespace().collect::<Vec<_>>();
        let amount = pogs.get(1).unwrap().parse::<usize>().unwrap();
        let dir = match pogs.get(0).unwrap() {
            &"U" => Up,
            &"D" => Down,
            &"L" => Left,
            &"R" => Right,
            _ => panic!("unmapped"),
        };
        for _ in 0..amount {
            match dir {
                Left => curr.0 -= 1,
                Up => curr.1 -= 1,
                Down => curr.1 += 1,
                Right => curr.0 += 1,
            }
            positions.push(curr.clone())
        }
    });
    positions.reverse();
    Some(shoelace(&positions) + positions.len() as i64)
}

fn shoelace(positions: &Vec<(i64, i64)>) -> i64 {
    let mut a = 0i64;
    let mut b = 0i64;
    for x in 0..positions.len() {
        a += (positions[x].0 * positions[(x + 1) % positions.len()].1) as i64;
        b += (positions[x].1 * positions[(x + 1) % positions.len()].0) as i64;
    }
    return (a - b).abs() / 2 - positions.len() as i64 / 2 + 1;
}

#[derive(Debug)]
enum Dir {
    Left,
    Up,
    Down,
    Right,
}

pub fn part_two(input: &str) -> Option<i64> {
    let mut positions = Vec::<(i64, i64)>::new();
    let mut curr = (0, 0);
    input.lines().for_each(|l| {
        let pogs = l.split_whitespace().collect::<Vec<_>>();
        let hex_chars = pogs.get(2).unwrap().chars();
        let d_string = hex_chars.clone().rev().nth(1).unwrap();
        let mut amount = hex_chars.collect::<Vec<_>>();
        amount.pop();
        amount.pop();
        amount.remove(0);
        amount.remove(0);
        let amt = amount.iter().map(|s| s.to_string()).join("");
        let amt_2 = i64::from_str_radix(amt.to_string().as_str(), 16).unwrap();

        let dir = match d_string {
            '0' => Right,
            '1' => Down,
            '2' => Left,
            '3' => Up,
            _ => panic!("unmapped"),
        };
        for _ in 0..amt_2 {
            match dir {
                Left => curr.0 -= 1,
                Up => curr.1 -= 1,
                Down => curr.1 += 1,
                Right => curr.0 += 1,
            }
            positions.push(curr.clone())
        }
    });
    Some(shoelace(&positions) + positions.len() as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
