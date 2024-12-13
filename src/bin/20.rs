use std::cmp::PartialEq;
use std::collections::{HashMap, HashSet, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut system = System::new(input);

    for _ in 0..1000 {
        system.simulate(true);
    }

    Some(system.low_pulses_sent * system.high_pulses_sent)
}

pub fn part_two(input: &str) -> Option<u64> {
    let mut system = System::new(input);

    let mut i: u64 = 1;
    let mut ans = vec![];
    loop {
        if system.simulate(false) {
            ans.push(i);
        }
        i += 1;
        if ans.len() == 4 {
            break;
        }
    }
    Some(ans.iter().product())
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

struct System {
    modules: Vec<Module>,
    high_pulses_sent: u32,
    low_pulses_sent: u32,
    cs_source: HashSet<String>,
}

impl System {
    fn new(input: &str) -> Self {
        let mut modules: Vec<_> = input.lines().map(Module::from).collect();
        modules.push(Module {
            module_type: ModuleType::Button,
            outputs: vec!["broadcaster".to_string()],
            name: "button".to_string(),
        });
        // add output module
        modules.push(Module {
            module_type: ModuleType::Output,
            outputs: vec![],
            name: "output".to_string(),
        });

        // find source of all conjuctions
        let (mut conjuctions, rest): (Vec<_>, Vec<_>) = modules.iter_mut().partition(|m| {
            return matches!(m.module_type, ModuleType::Conjunction { .. });
        });
        for con in conjuctions.iter_mut() {
            match &mut con.module_type {
                ModuleType::Conjunction { last_pulse_by_name } => {
                    for module in rest
                        .iter()
                        .filter(|m| m.outputs.contains(&con.name) && m.name != "output")
                    {
                        last_pulse_by_name.insert(module.name.clone(), Pulse::Low);
                    }
                }
                _ => unreachable!(),
            }
        }

        Self {
            modules,
            high_pulses_sent: 0,
            low_pulses_sent: 0,
            cs_source: HashSet::new(),
        }
    }

    fn get_module_mut(&mut self, name: &str) -> Option<&mut Module> {
        self.modules.iter_mut().find(|m| m.name == name)
    }

    fn simulate(&mut self, part_one: bool) -> bool {
        let mut t = false;
        // self.send_pulse("button", "broadcaster", Pulse::Low);
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        while let Some((source, target, pulse)) = queue.pop_front() {
            match pulse {
                Pulse::High => self.high_pulses_sent += 1,
                Pulse::Low => self.low_pulses_sent += 1,
            }
            if (source == "tg" || source == "hn" || source == "lz" || source == "kh")
                && pulse == Pulse::High
                && !self.cs_source.contains(&source)
            {
                self.cs_source.insert(source.clone());
                return true;
            }

            let wire = Wire {
                source: source.clone(),
                pulse: pulse.clone(),
            };
            if let Some(curr) = self.get_module_mut(&target) {
                match &mut curr.module_type {
                    ModuleType::FlipFlop { ref mut on } => {
                        if wire.pulse == Pulse::Low {
                            if !*on {
                                *on = true;
                                curr.outputs.iter().for_each(|output| {
                                    queue.push_back((
                                        curr.name.clone(),
                                        output.clone(),
                                        Pulse::High,
                                    ));
                                });
                            } else {
                                *on = false;
                                curr.outputs.iter().for_each(|output| {
                                    queue.push_back((
                                        curr.name.clone(),
                                        output.clone(),
                                        Pulse::Low,
                                    ));
                                });
                            }
                        }
                    }
                    ModuleType::Button => {
                        unreachable!("Button should not be in the queue");
                    }
                    ModuleType::Broadcast => {
                        let outputs = curr.outputs.clone();
                        for output in outputs.iter() {
                            queue.push_back((
                                curr.name.clone(),
                                output.clone(),
                                wire.pulse.clone(),
                            ));
                        }
                    }
                    ModuleType::Conjunction {
                        ref mut last_pulse_by_name,
                    } => {
                        last_pulse_by_name.insert(wire.source, wire.pulse.clone());

                        let send_low = last_pulse_by_name.iter().all(|p| p.1 == &Pulse::High)
                            || last_pulse_by_name.len() == 0;
                        {
                            for output in curr.outputs.iter() {
                                queue.push_back((
                                    curr.name.clone(),
                                    output.clone(),
                                    if send_low { Pulse::Low } else { Pulse::High },
                                ));
                            }
                        }
                    }
                    ModuleType::Output => {}
                }
            }
        }
        t
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum ModuleType {
    FlipFlop {
        on: bool,
    },
    Button,
    Broadcast,
    Conjunction {
        last_pulse_by_name: HashMap<String, Pulse>,
    },
    Output,
}

#[derive(Clone, Debug)]
struct Module {
    module_type: ModuleType,
    outputs: Vec<String>,
    name: String,
}

#[derive(Clone, Eq, PartialEq, Debug)]
enum Pulse {
    High,
    Low,
}

#[derive(Clone, Debug)]
struct Wire {
    source: String,
    pulse: Pulse,
}

impl From<&str> for Module {
    fn from(value: &str) -> Self {
        let (left, right) = value.split_once(" -> ").unwrap();
        if left == "broadcaster" {
            let x = Self {
                module_type: ModuleType::Broadcast,
                outputs: right.split(", ").map(|s| s.to_string()).collect(),
                name: "broadcaster".to_string(),
            };
            return x;
        }

        if left.starts_with("%") {
            let x = Self {
                module_type: ModuleType::FlipFlop { on: false },
                outputs: right.split(", ").map(|s| s.to_string()).collect(),
                name: left.to_string().trim_matches('%').to_string(),
            };
            return x;
        }
        if left.starts_with("&") {
            let x = Self {
                module_type: ModuleType::Conjunction {
                    last_pulse_by_name: HashMap::new(),
                },
                outputs: right.split(", ").map(|s| s.to_string()).collect(),
                name: left.to_string().trim_matches('&').to_string(),
            };
            return x;
        }
        unreachable!("Invalid module type");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(11687500));
    }

    #[test]
    fn test_part_one_a() {
        let result = part_one(
            r#"broadcaster -> a, b, c
%a -> b
%b -> c
%c -> inv
&inv -> a"#,
        );
        assert_eq!(result, Some(32000000));
    }

    #[test]
    fn test_part_one_b() {
        let result = part_one(
            r#"broadcaster -> a, b
%a -> con
%b -> con
&con -> output"#,
        );
        // assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
