use advent_of_code::GridCreator;
use ndarray::Axis;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<i32> {
    min_distance(input, 1, 3)
}

pub fn part_two(input: &str) -> Option<i32> {
    min_distance(input, 4, 10)
}

fn min_distance(input: &str, min_steps: usize, max_steps: usize) -> Option<i32> {
    let grid = input.create_grid();
    let mut distances = HashMap::<(i32, i32, (i32, i32)), i32>::new();
    let mut q = BinaryHeap::<Item>::new();
    q.push(Item {
        cost: 0,
        row: 0,
        col: 0,
        dr: 0,
        dc: 0,
    });
    while let Some(item) = q.pop() {
        let row = item.row;
        let col = item.col;
        let cost = item.cost;
        let dir = (item.dr, item.dc);

        // Are we in the bottom right?
        if (row, col)
            == (
                (grid.len_of(Axis(0)) - 1) as i32,
                (grid.len_of(Axis(0)) - 1) as i32,
            )
        {
            return Some(cost);
        }

        // For each direction Up, Down, Left, Right
        for (dr, dc) in [(-1, 0), (1, 0), (0, -1), (0, 1)] {
            // If the previous direction is the same or backwards, skip
            // We've already incremnted X steps in a previous loop
            if dir == (dr, dc) || dir == (-dr, -dc) {
                continue;
            }
            let mut next_cost = cost;
            for dist in 1..=max_steps {
                // New r/c positions
                let rr = row + dr * dist as i32;
                let cc = col + dc * dist as i32;

                //Boundary checking
                if rr >= grid.len_of(Axis(0)) as i32
                    || cc >= grid.len_of(Axis(0)) as i32
                    || rr < 0
                    || cc < 0
                {
                    continue;
                }

                // Add the current character to the next cost
                next_cost += grid
                    .get((rr as usize, cc as usize))
                    .unwrap()
                    .to_digit(10)
                    .unwrap() as i32;

                // Skip steps that aren't allowed to be stopped on
                if dist < min_steps {
                    continue;
                }
                let key = (rr, cc, (dr, dc));
                // If the next cost is less then the current max, set it as the best cost
                if next_cost < distances.get(&key).unwrap_or(&i32::MAX).clone() {
                    // Overwrite the distances
                    distances.insert(key, next_cost);

                    // Push the current item onto the heap
                    q.push(Item {
                        cost: next_cost,
                        row: rr,
                        col: cc,
                        dr,
                        dc,
                    });
                }
            }
        }
    }
    None
}

#[derive(Debug, PartialEq, Eq)]
struct Item {
    cost: i32,
    row: i32,
    col: i32,
    dr: i32,
    dc: i32,
}

// Reverse ordering so we get min-heap behavior
impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost)
    }
}

// What the crap is this used for
impl PartialOrd for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
