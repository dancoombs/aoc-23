use std::collections::HashMap;

use lazy_static::lazy_static;

advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    Some(
        input
            .lines()
            .map(|l| {
                let digits = l
                    .chars()
                    .filter_map(|c| c.to_digit(10))
                    .collect::<Vec<u32>>();
                digits.first().unwrap() * 10 + digits.last().unwrap()
            })
            .sum::<u32>(),
    )
}

lazy_static! {
    static ref DIGITS: HashMap<&'static str, u32> = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
        ("1", 1),
        ("2", 2),
        ("3", 3),
        ("4", 4),
        ("5", 5),
        ("6", 6),
        ("7", 7),
        ("8", 8),
        ("9", 9)
    ]);
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(input.lines().map(parse_line_two).sum::<u32>())
}

fn parse_line_two(line: &str) -> u32 {
    let min = *DIGITS
        .iter()
        .filter_map(|(k, v)| {
            let idx = line.find(k)?;
            Some((idx, v))
        })
        .min_by_key(|(idx, _)| *idx)
        .unwrap()
        .1;
    let max = *DIGITS
        .iter()
        .filter_map(|(k, v)| {
            let idx = line.rfind(k)?;
            Some((idx, v))
        })
        .max_by_key(|(idx, _)| *idx)
        .unwrap()
        .1;

    min * 10 + max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(142));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(281));
    }
}
