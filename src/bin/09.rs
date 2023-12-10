advent_of_code::solution!(9);

pub fn part_one(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .map(extrapolate)
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<i64> {
    Some(
        input
            .lines()
            .map(|l| {
                l.split_ascii_whitespace()
                    .map(|n| n.parse().unwrap())
                    .collect()
            })
            .map(extrapolate_backwards)
            .sum(),
    )
}

fn diffs(nums: Vec<i64>) -> Vec<Vec<i64>> {
    let mut non_zero = true;
    let mut seqs = vec![nums];
    while non_zero {
        non_zero = false;
        let nums = seqs.last().unwrap();
        let mut next = vec![0; nums.len() - 1];
        for i in 1..nums.len() {
            next[i - 1] = nums[i] - nums[i - 1];
            if next[i - 1] != 0 {
                non_zero = true;
            }
        }
        seqs.push(next);
    }
    seqs
}

fn extrapolate(nums: Vec<i64>) -> i64 {
    let seqs = diffs(nums);
    let ret = seqs.iter().fold(0, |acc, seq| acc + seq.last().unwrap());
    ret
}

fn extrapolate_backwards(nums: Vec<i64>) -> i64 {
    let seqs = diffs(nums);
    let ret = seqs.iter().rev().fold(0, |acc, seq| seq[0] - acc);
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(114));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }
}
