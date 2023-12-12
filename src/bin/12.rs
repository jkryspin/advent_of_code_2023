use cached::proc_macro::cached;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<usize> {
    let lines = input.lines().collect::<Vec<_>>();
    Some(
        lines
            .iter()
            .map(|line| {
                let (row, pattern) = line.split_once(" ").unwrap();
                let r = row.to_string() + ".";
                let x = arrangements(
                    r.chars().collect(),
                    pattern
                        .split(",")
                        .map(|s| s.parse::<usize>().unwrap())
                        .collect(),
                );
                x
            })
            .sum::<usize>(),
    )
}

#[cached]
fn arrangements(chars: Vec<char>, groups: Vec<usize>) -> usize {
    let cs = chars.clone();
    if cs.len() == 0 {
        return if groups.len() > 0 { 0 } else { 1 };
    }
    if cs[0] == '.' {
        return arrangements(cs[1..].to_vec(), groups);
    }
    if cs[0] == '?' {
        let mut one = cs.clone();
        one[0] = '.';
        let mut two = cs.clone();
        two[0] = '#';
        return arrangements(one, groups.clone()) + arrangements(two, groups.clone());
    }

    if cs[0] == '#' {
        if groups.len() == 0 {
            return 0;
        }
        if cs.len() < groups[0] {
            return 0;
        }
        for c in 0..groups[0] {
            if cs[c] == '.' {
                return 0;
            }
        }
        // Group doesn't end in #
        if cs[groups[0]] == '#' {
            return 0;
        }
        // are there more groups to process
        if groups.len() >= 1 {
            // Move Forward
            return arrangements(cs[(groups[0] + 1)..].to_vec(), groups[1..].to_vec());
        }
        // Match found!
        return 1;
    }

    panic!("Parsing issue");
}
pub fn part_two(input: &str) -> Option<usize> {
    let lines = input.lines();
    let mut sum = 0;
    lines.for_each(|line| {
        let (row, pattern) = line.split_once(" ").unwrap();
        let mut r = (row.to_string() + "?").repeat(5);
        let mut p = (pattern.to_string() + ",").repeat(5);
        r = r[0..r.len() - 1].to_string();
        r = r + ".";
        p = p[0..p.len() - 1].to_string();
        let x = arrangements(
            r.chars().collect(),
            p.split(",").map(|s| s.parse::<usize>().unwrap()).collect(),
        );
        sum += x;
    });
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
