use advent_of_code::GridCreator;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet, VecDeque};
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.create_grid();
    let distances = HashMap::new();
    let mut q = BinaryHeap::<std::cmp::Reverse<_>>::new();
    while let Some(Reverse((cost, (row, col, dir)))) = q.pop() {
        if (row, col) == (grid.len() - 1, grid.row(0).len() - 1) {
            return cost;
        }
        if distances.get(&(row, col, dir)).is_some_and(|&c| cost < c) {
            continue;
        }
    }
    None
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
        assert_eq!(result, None);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
