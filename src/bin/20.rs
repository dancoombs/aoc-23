use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

use itertools::Itertools;

advent_of_code::solution!(20);

pub fn part_one(input: &str) -> Option<u32> {
    let mut modules = setup_modules(input);

    let mut low_pulses = 0;
    let mut high_pluses = 0;

    for _ in 0..1000 {
        let mut round_low_pulses = 0;
        let mut round_high_pluses = 0;
        let mut pending = VecDeque::from(vec![(
            "button".to_string(),
            "broadcaster".to_string(),
            false,
        )]);
        while !pending.is_empty() {
            let (from, to, pulse) = pending.pop_front().unwrap();
            if pulse {
                round_high_pluses += 1;
            } else {
                round_low_pulses += 1;
            }

            let module = modules.get_mut(&to);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            let output = process_pulse(&from, pulse, module);
            if let Some(new_pulse) = output {
                for output in &module.outputs {
                    pending.push_back((module.name.clone(), output.clone(), new_pulse));
                }
            }
        }
        low_pulses += round_low_pulses;
        high_pluses += round_high_pluses;
    }

    Some(high_pluses * low_pulses)
}

// rx
// &dr
// mp, qt, qb, ng
const INPUTS: [&str; 4] = ["mp", "qt", "qb", "ng"];

pub fn part_two(input: &str) -> Option<u32> {
    let mut modules = setup_modules(input);

    let mut i = 1;
    loop {
        let mut pending = VecDeque::from(vec![(
            "button".to_string(),
            "broadcaster".to_string(),
            false,
        )]);

        while !pending.is_empty() {
            let (from, to, pulse) = pending.pop_front().unwrap();
            if !pulse && INPUTS.contains(&to.as_str()) {
                println!("{} {}", to, i);
            }

            let module = modules.get_mut(&to);
            if module.is_none() {
                continue;
            }
            let module = module.unwrap();
            let output = process_pulse(&from, pulse, module);
            if let Some(new_pulse) = output {
                for output in &module.outputs {
                    pending.push_back((module.name.clone(), output.clone(), new_pulse));
                }
            }
        }
        i += 1;

        if i > 1_000_00 {
            return None;
        }
    }
}

fn setup_modules(input: &str) -> HashMap<String, Module> {
    let mut modules = input
        .lines()
        .map(|line| {
            let module = line.parse::<Module>().unwrap();
            (module.name.clone(), module)
        })
        .collect::<HashMap<String, Module>>();

    for module in modules.clone().values() {
        for output in &module.outputs {
            let output = modules.get_mut(output);
            if output.is_none() {
                continue;
            }
            let output = output.unwrap();

            if let State::Conjunction(state) = &mut output.state {
                state.insert(module.name.clone(), false);
            }
        }
    }

    modules
}

fn process_pulse(from: &str, pulse: bool, module: &mut Module) -> Option<bool> {
    match &mut module.state {
        State::Broadcast => Some(false),
        State::FlipFlop(state) => {
            if !pulse {
                *state = !*state;
                Some(*state)
            } else {
                None
            }
        }
        State::Conjunction(state) => {
            state
                .insert(from.to_string(), pulse)
                .expect("unexpected input to conjunction");
            if state.values().all(|&v| v) {
                Some(false)
            } else {
                Some(true)
            }
        }
    }
}

#[derive(Debug, Clone)]
enum State {
    Broadcast,
    FlipFlop(bool),
    Conjunction(HashMap<String, bool>),
}

#[derive(Debug, Clone)]
struct Module {
    name: String,
    state: State,
    outputs: Vec<String>,
}

impl FromStr for Module {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (kind, outputs) = s.split(" -> ").collect_tuple().unwrap();
        let (state, name) = match kind.chars().next().unwrap() {
            '%' => {
                let name = kind[1..].to_string();
                let state = State::FlipFlop(false);
                (state, name)
            }
            '&' => {
                let name = kind[1..].to_string();
                let state = State::Conjunction(HashMap::new());
                (state, name)
            }
            'b' => {
                let name = kind.to_string();
                let state = State::Broadcast;
                (state, name)
            }
            _ => panic!("Unknown kind: {}", kind),
        };

        Ok(Module {
            name,
            state,
            outputs: outputs.split(", ").map(|s| s.to_string()).collect(),
        })
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
    fn test_part_two() {
        let result: u64 = 3917 * 3919 * 4007 * 4027;
        println!("{}", result);
    }
}
