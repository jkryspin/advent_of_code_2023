use std::collections::HashMap;
use std::ops::Index;
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
    let lines_new = lines.map(|l| {
        let mut res: String = l.to_string();
        map.iter().for_each(|c| {
            res = res.replace(c.0, c.1).as_str().parse().unwrap();
        });
        return res;
    });
    let mut sum = 0;
    lines_new.into_iter().for_each(|s| {
        let (first, last) = parse_line(s);
        sum += first * 10 + last
    });

    Some(sum)
}
fn parse_line(s: String) -> (u32, u32) {
    let mut first = None;
    let mut last = None;
    s.chars().for_each(|c| {
        let d = c.to_digit(10);
        if first.is_none() && d.is_some() {
            first = d;
        }
        if d.is_some() {
            last = d;
        }
    });
    return (first.unwrap(), last.unwrap());
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
