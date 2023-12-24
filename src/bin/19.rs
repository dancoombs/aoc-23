use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

advent_of_code::solution!(19);

pub fn part_one(input: &str) -> Option<u32> {
    let (workflows, inputs) = input.split("\n\n").collect_tuple().unwrap();
    let workflows = workflows
        .lines()
        .map(|l| l.parse::<Workflow>().unwrap())
        .map(|w| (w.name.clone(), w))
        .collect::<HashMap<String, Workflow>>();

    let re = regex::Regex::new(r"\{x=(\d+),m=(\d+),a=(\d+),s=(\d+)\}").unwrap();
    let inputs = inputs.lines().map(|i| {
        re.captures(i)
            .unwrap()
            .iter()
            .skip(1)
            .map(|c| c.unwrap().as_str().parse::<u32>().unwrap())
            .collect::<Vec<u32>>()
    });

    let mut sum = 0;
    for input in inputs {
        let mut cur = "in".to_string();
        while cur != "R" && cur != "A" {
            let workflow = workflows.get(&cur).unwrap();
            cur = workflow.evaluate(&input);
        }
        if cur == "A" {
            sum += input.iter().sum::<u32>();
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u64> {
    None
}

#[derive(Debug)]
struct Rule {
    category: u8,
    is_less_than: bool,
    value: u32,
    target: String,
}

impl Rule {
    fn evaluate(&self, scores: &[u32]) -> bool {
        if self.is_less_than {
            scores[self.category as usize] < self.value
        } else {
            scores[self.category as usize] > self.value
        }
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    else_workflow: String,
}

impl Workflow {
    fn evaluate(&self, scores: &[u32]) -> String {
        for rule in &self.rules {
            if rule.evaluate(scores) {
                return rule.target.clone();
            }
        }
        self.else_workflow.clone()
    }
}

impl FromStr for Workflow {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = regex::Regex::new(r"(.*)\{(.*)\}").unwrap();
        let captures = re.captures(s).unwrap();
        let name = captures.get(1).unwrap().as_str().to_string();
        let mut rules = captures
            .get(2)
            .unwrap()
            .as_str()
            .split(',')
            .collect::<Vec<&str>>();
        let else_workflow = rules.pop().unwrap().to_string();

        let re = regex::Regex::new(r"(\w)(<|>)(\d+):(.*)").unwrap();
        let rules = rules
            .iter()
            .map(|r| {
                let captures = re.captures(r).unwrap();
                let category = captures.get(1).unwrap().as_str();
                let category = match category {
                    "x" => 0,
                    "m" => 1,
                    "a" => 2,
                    "s" => 3,
                    _ => panic!("Unknown category: {}", category),
                };

                let is_less_than = captures.get(2).unwrap().as_str() == "<";
                let value = captures.get(3).unwrap().as_str().parse().unwrap();
                let target = captures.get(4).unwrap().as_str().to_string();
                Rule {
                    category,
                    is_less_than,
                    value,
                    target,
                }
            })
            .collect();

        Ok(Workflow {
            name,
            rules,
            else_workflow,
        })
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
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(167409079868000));
    }
}
