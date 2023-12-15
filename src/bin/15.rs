use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(15);

pub fn part_one(input: &str) -> Option<u64> {
    input
        .trim_end()
        .split(',')
        .map(hash)
        .fold(0, |acc, s| acc + s as u64)
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    let seq = input.trim_end().split(',').collect::<Vec<_>>();

    let mut hash_cache = HashMap::new();
    let mut boxes: HashMap<u64, Vec<(&str, u64)>> = HashMap::new();

    for s in seq {
        if s.ends_with('-') {
            let lens = &s[..s.len() - 1];
            let box_num = hash_with_cache(lens, &mut hash_cache);
            if let Some(lenses) = boxes.get_mut(&box_num) {
                if let Some(i) = lenses.iter().position(|l| l.0 == lens) {
                    lenses.remove(i);
                }
            }
        } else {
            let (lens, power) = s.split('=').collect_tuple().unwrap();
            let box_num = hash_with_cache(lens, &mut hash_cache);
            let lenses = boxes.entry(box_num).or_insert_with(Vec::new);

            if let Some(i) = lenses.iter().position(|l| l.0 == lens) {
                lenses[i] = (lens, power.parse().unwrap());
            } else {
                lenses.push((lens, power.parse().unwrap()));
            }
        }
    }

    let mut sum = 0;
    for (box_num, lenses) in boxes {
        for (i, (_, power)) in lenses.iter().enumerate() {
            sum += (box_num + 1) * (i + 1) as u64 * power;
        }
    }

    Some(sum)
}

fn hash_with_cache<'a>(s: &str, cache: &mut HashMap<String, u64>) -> u64 {
    if let Some(v) = cache.get(s) {
        return *v;
    }
    let v = hash(s);
    cache.insert(s.to_owned(), v);
    v
}

fn hash(s: &str) -> u64 {
    s.chars().fold(0, |acc, c| ((acc + c as u64) * 17) % 256)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(1320));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(145));
    }
}
