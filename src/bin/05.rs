use std::collections::BTreeMap;

use itertools::Itertools;
use std::ops::Bound::Excluded;
use std::ops::Bound::Included;
use std::ops::Bound::Unbounded;

advent_of_code::solution!(5);

pub fn part_one(input: &str) -> Option<u64> {
    let parts = input.split("\n\n").collect::<Vec<_>>();
    let seeds = parts[0]
        .split_ascii_whitespace()
        .skip(1)
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();
    let maps = parse_maps(&parts[1..]);

    let min = seeds
        .iter()
        .map(|s| {
            let mut idx = *s;
            for map in &maps {
                idx = map.map(idx);
            }
            idx
        })
        .min()
        .unwrap();

    Some(min)
}

pub fn part_two(input: &str) -> Option<u64> {
    let parts = input.split("\n\n").collect::<Vec<_>>();

    let mut seed_ranges = vec![];
    for chunk in &parts[0].split_ascii_whitespace().skip(1).chunks(2) {
        let (start, count) = chunk
            .map(|s| s.parse::<u64>().unwrap())
            .collect_tuple()
            .unwrap();
        seed_ranges.push((start, start + count));
    }
    seed_ranges.sort_by(|a, b| a.0.cmp(&b.0));

    let maps = parse_maps(&parts[1..]);

    for map in &maps {
        seed_ranges = map.map_ranges(seed_ranges);
    }

    Some(seed_ranges[0].0)
}

fn parse_maps(parts: &[&str]) -> Vec<Mapping> {
    parts
        .iter()
        .map(|m| {
            let mut mapping = Mapping::new();
            m.lines().skip(1).for_each(|l| {
                let (dest, source, length) = l
                    .split_ascii_whitespace()
                    .map(|p| p.parse::<u64>().unwrap())
                    .collect_tuple()
                    .unwrap();
                mapping.add_offset(source, dest as i64 - source as i64, length);
            });
            mapping
        })
        .collect()
}

#[derive(Debug)]
struct Mapping {
    ranges: BTreeMap<u64, i64>,
}

impl Mapping {
    fn new() -> Self {
        let mut ranges = BTreeMap::new();
        ranges.insert(0, 0);

        Self { ranges }
    }

    fn add_offset(&mut self, start: u64, offset: i64, range: u64) {
        let end = start + range;
        self.ranges.insert(start, offset);
        if !self.ranges.contains_key(&end) {
            self.ranges.insert(end, 0);
        }
    }

    fn get_offset(&self, idx: u64) -> i64 {
        *self
            .ranges
            .range((Unbounded, Included(idx)))
            .next_back()
            .unwrap()
            .1
    }

    fn map(&self, idx: u64) -> u64 {
        let offset = *self
            .ranges
            .range((Unbounded, Included(idx)))
            .next_back()
            .unwrap()
            .1;
        (idx as i64 + offset) as u64
    }

    fn map_ranges(&self, ranges: Vec<(u64, u64)>) -> Vec<(u64, u64)> {
        let mut r = ranges
            .into_iter()
            .flat_map(|(start, count)| self.map_range(start, count))
            .collect::<Vec<_>>();
        r.sort_by(|a, b| a.0.cmp(&b.0));
        r
    }

    fn map_range(&self, start: u64, end: u64) -> Vec<(u64, u64)> {
        let mut ranges = vec![];
        let mut cur = start;
        while cur < end {
            let offset = self.get_offset(cur);
            let next = self
                .ranges
                .range((Excluded(cur), Included(end)))
                .next()
                .map_or(end, |n| *n.0);
            ranges.push(((cur as i64 + offset) as u64, (next as i64 + offset) as u64));
            cur = next;
        }

        ranges
    }
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
