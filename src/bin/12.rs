use std::collections::HashMap;

use itertools::Itertools;

advent_of_code::solution!(12);

pub fn part_one(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|l| {
            let (springs, counts) = parse_line(l);
            num_valid_permutations(&springs, &counts)
        })
        .sum::<u64>()
        .into()
}

pub fn part_two(input: &str) -> Option<u64> {
    input
        .lines()
        .map(|l| {
            let (springs, counts) = parse_line(l);

            let mut unfold_springs = vec![];
            let mut unfold_counts = vec![];
            for _ in 0..5 {
                unfold_springs.push('?');
                unfold_springs.extend(springs.iter());
                unfold_counts.extend(counts.iter());
            }

            num_valid_permutations(&unfold_springs, &unfold_counts)
        })
        .sum::<u64>()
        .into()
}

fn parse_line(input: &str) -> (Vec<char>, Vec<usize>) {
    let (springs, counts): (&str, &str) = input.split_ascii_whitespace().collect_tuple().unwrap();
    let springs = springs.chars().collect::<Vec<_>>();
    let counts = counts
        .split(',')
        .map(|s| s.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    (springs, counts)
}

fn num_valid_permutations(springs: &[char], counts: &[usize]) -> u64 {
    let spring_idx = 0;
    let count_idx = 0;
    let mut memo = HashMap::new();

    fn inner(
        springs: &[char],
        counts: &[usize],
        spring_idx: usize,
        count_idx: usize,
        memo: &mut HashMap<(usize, usize), u64>,
    ) -> u64 {
        if memo.contains_key(&(spring_idx, count_idx)) {
            return memo[&(spring_idx, count_idx)];
        }

        let ret = (|| {
            if spring_idx == springs.len() {
                if count_idx == counts.len() {
                    return 1;
                } else {
                    return 0;
                }
            }

            let mut start_sequence = || {
                // cannot start new sequence
                if count_idx == counts.len() {
                    return 0;
                }

                // start a sequence
                // consume at least expected strings
                let expected = counts[count_idx];
                let mut cur_count = 0;
                while (spring_idx + cur_count) < springs.len() && cur_count < expected {
                    match springs[spring_idx + cur_count] {
                        '.' => return 0, // invalid sequence
                        '#' | '?' => cur_count += 1,
                        _ => panic!("invalid spring"),
                    }
                }

                if cur_count == expected {
                    if (spring_idx + cur_count) == springs.len() {
                        // end state
                        if count_idx + 1 == counts.len() {
                            return 1;
                        } else {
                            return 0;
                        }
                    }

                    match springs[spring_idx + cur_count] {
                        '.' | '?' => {
                            return inner(
                                springs,
                                counts,
                                spring_idx + cur_count + 1,
                                count_idx + 1,
                                memo,
                            )
                        }
                        '#' => return 0,
                        _ => panic!("invalid spring"),
                    }
                } else {
                    return 0;
                }
            };

            match springs[spring_idx] {
                '.' => inner(springs, counts, spring_idx + 1, count_idx, memo),
                '#' => start_sequence(),
                '?' => start_sequence() + inner(springs, counts, spring_idx + 1, count_idx, memo),
                _ => panic!("invalid spring"),
            }
        })();

        memo.insert((spring_idx, count_idx), ret);
        ret
    }

    inner(springs, counts, spring_idx, count_idx, &mut memo)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(21));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(525152));
    }
}
