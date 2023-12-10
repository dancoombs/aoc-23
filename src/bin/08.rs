use std::collections::HashMap;

use itertools::Itertools;
use regex::Regex;

advent_of_code::solution!(8);

pub fn part_one(input: &str) -> Option<u64> {
    let (instructions, conns) = parse(input);

    let mut cur = "AAA".to_owned();
    let mut step = 0;
    let len = instructions.len();
    let insts = instructions.as_bytes();

    while cur != "ZZZ" {
        let idx = step % len;
        let dir = insts[idx];
        if dir == b'L' {
            cur = conns[&cur].0.clone();
        } else if dir == b'R' {
            cur = conns[&cur].1.clone();
        } else {
            panic!("Invalid direction");
        }
        step += 1;
    }

    Some(step as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (instructions, conns) = parse(input);
    let insts = instructions.as_bytes();

    let vals = conns
        .keys()
        .filter(|k| k.as_bytes()[2] == b'A')
        .map(|k| find_cycle_len(k, &conns, insts))
        .collect::<Vec<_>>();
    let lcm = lcm(&vals);

    Some(lcm)
}

fn parse(input: &str) -> (&str, HashMap<String, (String, String)>) {
    let (instructions, connections): (&str, &str) = input.split("\n\n").collect_tuple().unwrap();

    let re = Regex::new(r"(\w{3}) = \((\w{3}), (\w{3})\)").unwrap();

    let conns = connections
        .lines()
        .map(|l| {
            let caps = re.captures(l).unwrap();
            (
                caps.get(1).unwrap().as_str().to_owned(),
                (
                    caps.get(2).unwrap().as_str().to_owned(),
                    caps.get(3).unwrap().as_str().to_owned(),
                ),
            )
        })
        .collect::<HashMap<String, (String, String)>>();

    (instructions, conns)
}

fn find_cycle_len(start: &str, conns: &HashMap<String, (String, String)>, insts: &[u8]) -> u64 {
    let mut cur = start.to_owned();
    let mut step = 0;
    let len = insts.len();

    while cur.as_bytes()[2] != b'Z' {
        let idx = step % len;
        let dir = insts[idx];
        if dir == b'L' {
            cur = conns[&cur].0.clone();
        } else if dir == b'R' {
            cur = conns[&cur].1.clone();
        } else {
            panic!("Invalid direction");
        }
        step += 1;
    }

    step as u64
}

fn lcm(nums: &[u64]) -> u64 {
    let mut result = 1;
    for &num in nums {
        result = num * result / gcd(num, result);
    }
    result
}

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }

    gcd(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(6));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(6));
    }
}
