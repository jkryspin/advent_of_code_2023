use std::ops::{Range, RangeInclusive};

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (rules, xmases) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(|l| Rule::from(l)).collect::<Vec<_>>();
    let xmases = xmases.lines().map(|l| xmas::from(l)).collect::<Vec<_>>();

    let mut sum = 0;
    xmases.iter().for_each(|xmas| {
        if simulate(&rules, xmas) {
            sum += xmas.get_score();
        } else {
            println!("No match: {:?}", xmas);
        }
    });

    Some(sum as usize)
}

fn simulate(rules: &[Rule], xmas: &xmas) -> bool {
    let mut input = "in".to_string();
    let mut history = vec![];
    let mut rules_applied = vec![];
    'l: loop {
        history.push(input.clone());
        for r in rules.iter() {
            let result = rule_matches(r, xmas, &input);
            if let Some(output) = result {
                rules_applied.push(r);
                input = output;
                if input == "A" {
                    return true;
                } else if input == "R" {
                    println!("Rejected: {:?}", history);
                    return false;
                }
                continue 'l;
            }
        }
        unreachable!("No rule found for input: {}", input);
    }
}

fn rule_matches(rule: &Rule, xmas: &xmas, input: &str) -> Option<String> {
    if rule.input != input {
        return None;
    }
    for rule_option in rule.rule_options.iter() {
        let found = match rule_option.xmas {
            'x' => {
                rule_option.greater_than == (xmas.x > rule_option.value)
                    && xmas.x != rule_option.value
            }
            'm' => {
                rule_option.greater_than == (xmas.m > rule_option.value)
                    && xmas.m != rule_option.value
            }
            'a' => {
                rule_option.greater_than == (xmas.a > rule_option.value)
                    && xmas.a != rule_option.value
            }
            's' => {
                rule_option.greater_than == (xmas.s > rule_option.value)
                    && xmas.s != rule_option.value
            }
            _ => unreachable!(),
        };

        if found {
            return Some(rule_option.result.clone());
        }
    }
    Some(rule.last.clone())
}

#[derive(Debug)]
struct xmas {
    x: u32,
    m: u32,
    a: u32,
    s: u32,
}

impl xmas {
    fn get_score(&self) -> u32 {
        self.x + self.m + self.a + self.s
    }
}
impl From<&str> for xmas {
    fn from(value: &str) -> Self {
        let value = value.trim_matches(|c| c == '{' || c == '}');
        let mut x = None;
        let mut m = None;
        let mut a = None;
        let mut s = None;

        for part in value.split(',') {
            // println!("{}", part);
            let (key, val) = part.split_once('=').unwrap();
            match key {
                "x" => x = Some(val.parse().unwrap()),
                "m" => m = Some(val.parse().unwrap()),
                "a" => a = Some(val.parse().unwrap()),
                "s" => s = Some(val.parse().unwrap()),
                _ => unreachable!("Unknown key: {}", key),
            }
        }

        Self {
            x: x.unwrap(),
            m: m.unwrap(),
            a: a.unwrap(),
            s: s.unwrap(),
        }
    }
}

#[derive(Debug)]
struct Rule {
    input: String,
    last: String,
    rule_options: Vec<RuleOption>,
}

// impl from string
impl From<&str> for Rule {
    fn from(value: &str) -> Self {
        let (input, rest) = value.split_once("{").unwrap();
        let options = rest.split(",").collect::<Vec<&str>>();
        let mut iter = options.iter().rev();
        let catch_all = iter.next().unwrap().trim_matches(|c| c == '{' || c == '}');
        assert!(iter.len() > 0);
        let rules = iter
            .rev()
            .map(|s| {
                let s_clone = s.clone();
                let (xmas, value) = s.split_once(&['<', '>']).unwrap();
                let (value, result) = value.split_once(":").unwrap();
                let result = result.trim_end_matches('}');
                assert_eq!(xmas.len(), 1);
                assert!(result.len() > 0);
                println!("{}", s_clone);
                RuleOption {
                    value: value.parse().unwrap(),
                    result: result.to_string(),
                    xmas: xmas.chars().next().unwrap(),
                    greater_than: s_clone.contains(">"),
                }
            })
            .collect::<Vec<_>>();
        // print all rules
        // println!("{} -> {:?}", input, rules);

        assert!(input.len() <= 3 && input.len() > 1);

        Self {
            rule_options: rules,
            input: input.to_string(),
            last: catch_all.to_string(),
        }
    }
}
#[derive(Debug)]
struct RuleOption {
    value: u32,
    result: String,
    xmas: char,
    greater_than: bool,
}

pub fn part_two(input: &str) -> Option<u128> {
    let (rules, _) = input.split_once("\n\n").unwrap();
    let rules = rules.lines().map(|l| Rule::from(l)).collect::<Vec<_>>();

    let input = "in".to_string();

    Some(simulate_two(
        input,
        &rules,
        xmas_range {
            x: 1..=4000,
            m: 1..=4000,
            a: 1..=4000,
            s: 1..=4000,
        },
    ))
}

fn simulate_two(input: String, rules: &[Rule], mut xmas_range: xmas_range) -> u128 {
    if input == "A" {
        return xmas_range.get_combinations_count();
    }
    if input == "R" {
        return 0;
    }

    let rule = rules
        .iter()
        .find(|&r| r.input == input)
        .expect("Count not find rule for input");

    let mut sum = 0;
    let mut range_left = xmas_range.clone();
    for r in rule.rule_options.iter() {
        let mut good_range = range_left.clone();

        match r.xmas {
            'x' => {
                if r.greater_than {
                    good_range.x = (r.value + 1)..=*xmas_range.x.end();
                    range_left.x = *xmas_range.x.start()..=r.value;
                } else {
                    good_range.x = *xmas_range.x.start()..=(r.value - 1);
                    range_left.x = r.value..=*xmas_range.x.end();
                }
            }
            'm' => {
                if r.greater_than {
                    good_range.m = (r.value + 1)..=*xmas_range.m.end();
                    range_left.m = *xmas_range.m.start()..=r.value;
                } else {
                    good_range.m = *xmas_range.m.start()..=(r.value - 1);
                    range_left.m = r.value..=*xmas_range.m.end();
                }
            }
            'a' => {
                if r.greater_than {
                    good_range.a = (r.value + 1)..=*xmas_range.a.end();
                    range_left.a = *xmas_range.a.start()..=r.value;
                } else {
                    good_range.a = *xmas_range.a.start()..=(r.value - 1);
                    range_left.a = r.value..=*xmas_range.a.end();
                }
            }
            's' => {
                if r.greater_than {
                    good_range.s = (r.value + 1)..=*xmas_range.s.end();
                    range_left.s = *xmas_range.s.start()..=r.value;
                } else {
                    good_range.s = *xmas_range.s.start()..=(r.value - 1);
                    range_left.s = r.value..=*xmas_range.s.end();
                }
            }
            _ => unreachable!(),
        }
        xmas_range = range_left.clone();

        sum += simulate_two(r.result.clone(), rules, good_range);
    }
    sum += simulate_two(rule.last.clone(), rules, range_left);

    sum
}

#[derive(Debug, Clone)]
struct xmas_range {
    x: RangeInclusive<u32>,
    m: RangeInclusive<u32>,
    a: RangeInclusive<u32>,
    s: RangeInclusive<u32>,
}

impl xmas_range {
    fn get_combinations_count(&self) -> u128 {
        // get length of each range
        println!("{:?}", self);
        let x = self.x.end() - (*self.x.start()) + 1;
        let m = self.m.end() - (*self.m.start()) + 1;
        let a = self.a.end() - (*self.a.start()) + 1;
        let s = self.s.end() - (*self.s.start()) + 1;

        let ans = x as u128 * m as u128 * a as u128 * s as u128;
        println!("{} * {} * {} * {} = {}", x, m, a, s, ans);

        ans
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_xmas() {
        let xmas = xmas::from("{x=820,m=149,a=558,s=29}");
        assert_eq!(xmas.x, 820);
        assert_eq!(xmas.m, 149);
        assert_eq!(xmas.a, 558);
        assert_eq!(xmas.s, 29);
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        // print how far off result is from 167409079868000
        println!("{}", result.unwrap() as i128 - 167409079868000i128);

        assert_eq!(result, Some(167409079868000));
    }
}
