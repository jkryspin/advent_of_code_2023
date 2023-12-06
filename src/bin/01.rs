use std::collections::HashMap;
advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut sum = 0;
    lines.for_each(|s| {
        let (first, last) = parse_line(s.to_string());
        sum += first * 10 + last;
    });
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let lines = input.lines();
    let mut map: HashMap<&str, &str> = HashMap::new();
    map.insert("one", "o1e");
    map.insert("two", "t2o");
    map.insert("three", "t3e");
    map.insert("four", "f4r");
    map.insert("five", "f5e");
    map.insert("six", "s6x");
    map.insert("seven", "s7n");
    map.insert("eight", "e8t");
    map.insert("nine", "n9e");
    return Some(lines.fold(0, |acc, l| {
        let mut res: String = l.to_string();
        map.iter().for_each(|c| {
            res = res.replace(c.0, c.1);
        });
        let (first, last) = parse_line(res);
        acc + first * 10 + last
    }));
}
fn parse_line(s: String) -> (u32, u32) {
    let first = get_first_digit(s.chars().collect::<Vec<char>>());
    let last = get_first_digit(s.chars().rev().collect::<Vec<char>>());
    return (first, last);
}

fn get_first_digit(s: Vec<char>) -> u32 {
    for c in s {
        let d = c.to_digit(10);
        match d {
            None => {}
            Some(c) => {
                return c;
            }
        }
    }
    // This shouldn't happen
    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(
            "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet",
        );
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
