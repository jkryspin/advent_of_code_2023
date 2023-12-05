use std::cmp::Ordering;
use std::ops::RangeInclusive;
use std::str::SplitWhitespace;
advent_of_code::solution!(5);

struct Map {
    source: u64,
    destination: u64,
    length: u64,
}

impl Map {
    fn get_source(&self, dest: &u64) -> Option<u64> {
        return if (self.destination..(self.destination + self.length)).contains(dest) {
            Some(dest - self.destination + self.source)
        } else {
            None
        };
    }
    fn get_dest(&self, source: u64) -> Option<u64> {
        if (self.source..(self.source + self.length)).contains(&source) {
            return Some(source - self.source + self.destination);
        }
        return None;
    }
}

fn parse_map(s: &str) -> Vec<Map> {
    let mut lines = s.lines();
    let name = lines.next().unwrap();
    let s_name = name.split('-').collect::<Vec<&str>>();

    let v = lines
        .into_iter()
        .map(|l| {
            let mut split = l.split_whitespace();
            let destination = split.next().unwrap().parse::<u64>().unwrap();
            let source = split.next().unwrap().parse::<u64>().unwrap();
            let length = split.next().unwrap().parse::<u64>().unwrap();
            Map {
                source,
                destination,
                length,
            }
        })
        .collect::<Vec<Map>>();
    return v;
}
pub fn part_one(input: &str) -> Option<u64> {
    let mut maps_s = input.split("\n\n");
    let mut init_seeds = maps_s.next().unwrap().split_whitespace();
    init_seeds.next();

    let seeds: Vec<u64> = init_seeds.map(|s| s.parse::<u64>().unwrap()).collect();
    let maps: Vec<Vec<Map>> = maps_s
        .map(|m| {
            return parse_map(m);
        })
        .collect();

    let locations = seeds
        .iter()
        .map(|seed| {
            let mut curr_seed = seed.to_owned();
            maps.iter().for_each(|map| {
                for m in map.iter() {
                    match m.get_dest(curr_seed) {
                        None => {}
                        Some(seed) => {
                            curr_seed = seed;
                            break;
                        }
                    }
                }
            });
            curr_seed
        })
        .collect::<Vec<u64>>();

    Some(locations.iter().min().unwrap().to_owned().to_owned())
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut maps_s = input.split("\n\n");
    let mut init_seeds = maps_s.next().unwrap().split_whitespace();
    init_seeds.next();

    let valid_ranges = get_range(init_seeds.clone());
    let maps: Vec<Vec<Map>> = maps_s.clone()
        .map(|m| {
            return parse_map(m);
        })
        .collect();

    let rev = maps.iter().rev().collect::<Vec<&Vec<Map>>>();
    let mut valid_ranges_locations = maps.last().unwrap().iter().map(|c|{
        return c.destination..=(c.destination + c.length);
    }).collect::<Vec<_>>();
    valid_ranges_locations.sort_by(|a,b|{
        if a.start() > b.start() {
            return Ordering::Greater;
        }
        return Ordering::Less;
    });

    for seed_range in valid_ranges_locations {
        for seed in seed_range{
        let source = source(seed, &rev);
        for r in valid_ranges.iter() {
            if r.contains(&source) {
                return Some(seed);
            }
        }
        }
    }
    return None
}

fn get_range(init_seeds: SplitWhitespace) -> Vec<RangeInclusive<u64>> {
    let mut x = 0;
    let v_seeds = init_seeds.collect::<Vec<&str>>();
    let mut valid_ranges = vec![];
    while x + 1 < v_seeds.len() {
        println!("{}", &v_seeds[x]);
        let left = v_seeds[x].parse::<u64>().unwrap();
        let right = v_seeds[x + 1].parse::<u64>().unwrap();
        valid_ranges.push(left..=(right + left));
        x += 2;
    }
    return valid_ranges;
}

fn source(seed: u64, maps: &Vec<&Vec<Map>>) -> u64 {
    let mut s = seed;
    &maps.iter().for_each(|v_maps| {
        for m in v_maps.iter() {
            match m.get_source(&s) {
                None => {}
                Some(src) => {
                    s = src;
                    break;
                }
            }
        }
    });
    return s;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(35));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }
}
