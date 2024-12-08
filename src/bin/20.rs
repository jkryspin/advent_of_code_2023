use std::cmp::PartialEq;
use std::collections::{HashMap, VecDeque};
use std::ops::{Deref, DerefMut};

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut system = System::new(input);

    for _ in 0..4 {
        system.simulate();
        // print low and high pulses sent
        println!("Low pulses sent: {}", system.low_pulses_sent);
        println!("High pulses sent: {}", system.high_pulses_sent);
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

    fn get_module_mut(&mut self, name: &str) -> &mut Module {
        self.modules.iter_mut().find(|m| m.name == name).unwrap()
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
        if module.queue.is_empty() {
            return None;
        }
        let wire = module.queue.pop_back().unwrap();
        Some(wire)
    }
    fn send_pulse(&mut self, source: &str, target: &str, pulse: Pulse) {
        let target_module = self.get_module_mut(target);
        target_module.queue.push_back(Wire {
            source: source.to_string(),
            pulse: pulse.clone(),
        });
        if pulse == Pulse::High {
            self.high_pulses_sent += 1;
        } else {
            self.low_pulses_sent += 1;
        }
        //button -low-> broadcaster
        println!("{} -{:?}-> {}", source, pulse, target);
    }
    fn simulate(&mut self) {
        self.send_pulse("button", "broadcaster", Pulse::Low);
        let mut curr_name: String = "button".to_string();
        let mut queue: Vec<(String, String, Pulse)> = Vec::new();
        loop {
            queue.drain(..).for_each(|(source, target, pulse)| {
                self.send_pulse(&source, &target, pulse);
            });

            // curr_name = first module with a queue
            if let Some(module) = self.get_first_module_with_queue() {
                curr_name = module.name.clone();
            } else {
                break;
            }
            if let Some(wire) = self.pop_pulse(&curr_name) {
                // keep popping
                let curr = self.get_module_mut(&curr_name);

                match &mut curr.module_type {
                    ModuleType::FlipFlop { ref mut on } => {
                        if wire.pulse == Pulse::Low {
                            if !*on {
                                *on = true;
                                curr.outputs.iter().for_each(|output| {
                                    queue.push((curr_name.clone(), output.clone(), Pulse::High));
                                });
                            } else {
                                *on = false;
                                curr.outputs.iter().for_each(|output| {
                                    queue.push((curr_name.clone(), output.clone(), Pulse::Low));
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
                            queue.push((curr_name.clone(), output.clone(), wire.pulse.clone()));
                        }
                    }
                    ModuleType::Conjunction {
                        ref mut last_pulse_by_name,
                    } => {
                        last_pulse_by_name.insert(wire.source, wire.pulse.clone());
                        if last_pulse_by_name.iter().all(|p| p.1 == &Pulse::High) {
                            queue.push((curr_name.clone(), curr.outputs[0].clone(), Pulse::Low));
                        } else {
                            queue.push((curr_name.clone(), curr.outputs[0].clone(), Pulse::High));
                        }
                    }
                    ModuleType::Output => {}
                }
            }
        }
    }
}

#[derive(Clone, Debug)]
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
