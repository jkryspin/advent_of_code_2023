use std::collections::HashSet;
advent_of_code::solution!(4);

pub fn part_one(input: &str) -> Option<u64> {
    let lines = input.lines();
    let mut total = 0;
    lines.for_each(|l| {
        let (_, winning_numbers, my_numbers) = parse(l);
        let win_hash = HashSet::<u64>::from_iter(winning_numbers);
        let mut sum = 0;
        my_numbers.iter().for_each(|n| {
            if win_hash.contains(n) {
                if sum == 0 {
                    sum = 1;
                } else {
                    sum = sum * 2;
                }
            }
        });
        total += sum;
    });
    Some(total)
}

fn parse(input: &str) -> (u64, Vec<u64>, Vec<u64>) {
    let mut parts = input.split("|");
    let left = parts.next().unwrap();
    let right = parts.next().unwrap();
    let mut card_split = left.split(":");
    let id = card_split
        .next()
        .unwrap()
        .split_whitespace()
        .into_iter()
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let winning_numbers = parse_numbers(card_split.next().unwrap().trim());
    let my_numbers = parse_numbers(right);
    return (id, winning_numbers, my_numbers);
}
fn parse_numbers(input: &str) -> Vec<u64> {
    return input
        .split_whitespace()
        .map(|w| {
            return w.parse::<u64>().unwrap();
        })
        .collect::<Vec<u64>>();
}

pub fn part_two(input: &str) -> Option<u64> {
    let lines = input.lines();
    let cards = lines
        .map(|l| {
            let (id, winning_numbers, my_numbers) = parse(l);
            let win_hash = HashSet::<u64>::from_iter(winning_numbers);
            let my_hash = HashSet::<u64>::from_iter(my_numbers);
            let sum = my_hash.intersection(&win_hash).count();
            return (id as usize, sum);
        })
        .collect::<Vec<(usize, usize)>>();

    let mut totals = vec![1; cards.len()];
    for (card_number, sum) in cards.into_iter() {
        let location = card_number - 1;
        for index in (location)..(location + sum) {
            totals[index + 1] += totals[location];
        }
    }
    Some(totals.iter().sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
