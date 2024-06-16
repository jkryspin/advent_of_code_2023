use std::collections::HashMap;
advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<usize> {
    let (workflow_map, vars) = parse(input);
    let goods = vars
        .iter()
        .filter(|v| {
            let mut w = workflow_map.get("in").unwrap();
            'outer: loop {
                assert!(w.rules.len() > 0);
                for r in w.rules.iter() {
                    // Match found, goto next
                    if let Some(m) = r.matched(v) {
                        if m == "R" {
                            return false;
                        }
                        if m == "A" {
                            return true;
                        }
                        w = workflow_map.get(m.as_str()).unwrap();
                        // This is needed, so we continue on the new rule immediately
                        continue 'outer;
                    }
                }
                // No rules match
                if w.all_fail == "R" {
                    return false;
                }
                if w.all_fail == "A" {
                    return true;
                }

                w = workflow_map.get(w.all_fail.as_str()).unwrap();
            }
        })
        .collect::<Vec<_>>();

    Some(
        goods
            .iter()
            .map(|i| i.values().sum::<usize>())
            .sum::<usize>(),
    )
}

fn parse(input: &str) -> (HashMap<String, RuleSet>, Vec<HashMap<char, usize>>) {
    let (left, right) = input.split_once("\n\n").unwrap();
    let mut workflow_map = HashMap::<String, RuleSet>::new();
    left.lines().for_each(|l| {
        let (label, more) = l.split_once('{').unwrap();
        let mut rules = more.split(',').collect::<Vec<_>>();
        let all_fail = rules.pop().unwrap().replace("}", "");
        rules.into_iter().for_each(|r| {
            let (v, val_s) = r.split_once(&['<', '>']).unwrap();
            let (val, target) = val_s.split_once(":").unwrap();
            assert_eq!(v.len(), 1);
            let rule = Rule {
                c: v.chars().next().unwrap(),
                operation: Op::new(r),
                comparitor: val.parse::<usize>().unwrap(),
                target: target.to_string(),
            };
            if let Some(m) = workflow_map.get_mut(label) {
                m.rules.push(rule);
            } else {
                let rs = RuleSet {
                    rules: vec![rule],
                    all_fail: all_fail.clone(),
                };
                assert!(workflow_map.insert(label.to_string(), rs).is_none());
            }
        })
    });

    let vs = right
        .lines()
        .map(|l| {
            let l1 = l.replace("{", "").replace("}", "");
            let mut m = HashMap::new();
            l1.split(',').for_each(|vars| {
                let (name, val) = vars.split_once('=').unwrap();
                assert_eq!(name.len(), 1);
                m.insert(name.chars().next().unwrap(), val.parse::<usize>().unwrap());
            });
            return m;
        })
        .collect::<Vec<_>>();
    return (workflow_map, vs);
}

#[derive(Debug)]
struct RuleSet {
    rules: Vec<Rule>,
    all_fail: String,
}

#[derive(Debug)]
struct Rule {
    c: char,
    operation: Op,
    comparitor: usize,
    target: String,
}

impl Rule {
    fn matched(&self, map: &HashMap<char, usize>) -> Option<String> {
        match self.operation {
            Op::Less => {
                if map.get(&self.c).unwrap() < &self.comparitor {
                    return Some(self.target.to_string());
                } else {
                    None
                }
            }
            Op::Greater => {
                if map.get(&self.c).unwrap() > &self.comparitor {
                    return Some(self.target.to_string());
                } else {
                    None
                }
            }
        }
    }
}

#[derive(Debug)]
enum Op {
    Less,
    Greater,
}

impl Op {
    fn new(input: &str) -> Self {
        return if input.contains(">") {
            Op::Greater
        } else {
            Op::Less
        };
    }
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
        assert_eq!(result, Some(19114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
