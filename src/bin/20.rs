use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut system = System::new(input);
    // find source of all conjuctions
    let (mut conjuctions, rest): (Vec<_>, Vec<_>) = system.modules.iter_mut().partition(|m| {
        return matches!(m.module_type, ModuleType::Conjunction { .. });
    });
    // for each conjuction, set all inputs on the conjuction to low
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

    for _ in 0..1000 {
        system.simulate();
        // print low and high pulses sent
        // println!("Low pulses sent: {}", system.low_pulses_sent);
        // println!("High pulses sent: {}", system.high_pulses_sent);
    }
    // print pulses sent

    Some(system.low_pulses_sent * system.high_pulses_sent)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

struct System {
    modules: Vec<Module>,
    high_pulses_sent: u32,
    low_pulses_sent: u32,
}

impl System {
    fn new(input: &str) -> Self {
        let mut modules: Vec<_> = input.lines().map(Module::from).collect();
        modules.push(Module {
            module_type: ModuleType::Button,
            queue: Vec::new().into(),
            outputs: vec!["broadcaster".to_string()],
            name: "button".to_string(),
        });
        // add output module
        modules.push(Module {
            module_type: ModuleType::Output,
            queue: Vec::new().into(),
            outputs: vec![],
            name: "output".to_string(),
        });

        Self {
            modules,
            high_pulses_sent: 0,
            low_pulses_sent: 0,
        }
    }

    fn get_module_mut(&mut self, name: &str) -> Option<&mut Module> {
        self.modules.iter_mut().find(|m| m.name == name)
    }

    fn get_module(&self, name: &str) -> Module {
        self.modules
            .iter()
            .find(|m| m.name == name)
            .unwrap()
            .clone()
    }
    fn get_first_module_with_queue(&self) -> Option<&Module> {
        self.modules.iter().find(|m| !m.queue.is_empty())
    }

    fn pop_pulse(&mut self, name: &str) -> Option<Wire> {
        let mut module = self.get_module_mut(name);
        match module {
            None => None,
            Some(m) => {
                if m.queue.is_empty() {
                    return None;
                }
                let wire = m.queue.pop_back().unwrap();
                Some(wire)
            }
        }
    }
    fn send_pulse(&mut self, source: &str, target: &str, pulse: Pulse) {
        let target_module = self.get_module_mut(target);
        match target_module {
            None => {}
            Some(target_module) => {
                target_module.queue.push_back(Wire {
                    source: source.to_string(),
                    pulse: pulse.clone(),
                });
            }
        }
        println!("{} -> {:?} {:?}", source, pulse, target);

        if pulse == Pulse::High {
            self.high_pulses_sent += 1;
        } else {
            self.low_pulses_sent += 1;
        }
    }
    fn simulate(&mut self) {
        // self.send_pulse("button", "broadcaster", Pulse::Low);
        let mut curr_name: String = "button".to_string();
        let mut queue: VecDeque<(String, String, Pulse)> = VecDeque::new();
        queue.push_back(("button".to_string(), "broadcaster".to_string(), Pulse::Low));
        loop {
            if let Some((source, target, pulse)) = queue.pop_front() {
                self.send_pulse(&source, &target, pulse);
            } 

            if let Some(module) = self.get_first_module_with_queue() {
                curr_name = module.name.clone();
            } else {
                break;
            }
            if let Some(wire) = self.pop_pulse(&curr_name) {
                let curr = self.get_module_mut(&curr_name).unwrap();

                match &mut curr.module_type {
                    ModuleType::FlipFlop { ref mut on } => {
                        if wire.pulse == Pulse::Low {
                            if !*on {
                                *on = true;
                                curr.outputs.iter().for_each(|output| {
                                    queue.push_back((
                                        curr_name.clone(),
                                        output.clone(),
                                        Pulse::High,
                                    ));
                                });
                            } else {
                                *on = false;
                                curr.outputs.iter().for_each(|output| {
                                    queue.push_back((
                                        curr_name.clone(),
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
                                curr_name.clone(),
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
                                    curr_name.clone(),
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
    queue: VecDeque<Wire>,
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
                queue: Vec::new().into(),
                outputs: right.split(", ").map(|s| s.to_string()).collect(),
                name: "broadcaster".to_string(),
            };
            return x;
        }

        if left.starts_with("%") {
            let x = Self {
                module_type: ModuleType::FlipFlop { on: false },
                queue: Vec::new().into(),
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
                queue: Vec::new().into(),
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
