advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let times = parse_part_one(&mut lines);
    let records = parse_part_one(&mut lines);
    let total = (0..times.len())
        .into_iter()
        .map(|i| Race {
            time: times[i],
            record: records[i],
        })
        .map(|r| r.ways_to_win())
        .reduce(|acc, e| acc * e)
        .unwrap();

    Some(total)
}

struct Race {
    time: u64,
    record: u64,
}

impl Race {
    fn ways_to_win(&self) -> u64 {
        (0..self.time)
            .into_iter()
            .reduce(|acc, held_time| {
                let traveled = held_time * (self.time - held_time);
                if traveled > self.record {
                    acc + 1
                } else {
                    acc
                }
            })
            .unwrap()
    }
}

fn parse_part_one(lines: &mut core::str::Lines) -> Vec<u64> {
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .map(|x| x.parse().unwrap())
        .collect::<Vec<u64>>();
    return times;
}

fn parse_part_two(lines: &mut core::str::Lines) -> u64 {
    let times = lines
        .next()
        .unwrap()
        .split_whitespace()
        .skip(1)
        .collect::<Vec<&str>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    return times;
}
pub fn part_two(input: &str) -> Option<u64> {
    let mut lines = input.lines();
    let time = parse_part_two(&mut lines);
    let distance = parse_part_two(&mut lines);
    let r = Race {
        time,
        record: distance,
    };
    Some(r.ways_to_win())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
