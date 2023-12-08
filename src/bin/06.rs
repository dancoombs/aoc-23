use itertools::Itertools;

advent_of_code::solution!(6);

pub fn part_one(input: &str) -> Option<u32> {
    let (times, distances): (Vec<u32>, Vec<u32>) = input
        .lines()
        .map(|l| {
            l.split_ascii_whitespace()
                .skip(1)
                .map(|n| n.parse().unwrap())
                .collect()
        })
        .collect_tuple()
        .unwrap();

    let mut res = 1;
    for (t, d) in times.into_iter().zip(distances.into_iter()) {
        let mut cnt = 0;
        for i in 0..t {
            if (i * (t - i)) > d {
                cnt += 1;
            }
        }
        res *= cnt;
    }

    Some(res)
}

pub fn part_two(input: &str) -> Option<u64> {
    let (t, d): (u64, u64) = input
        .lines()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>()
                .parse()
                .unwrap()
        })
        .collect_tuple()
        .unwrap();

    // it - i^2 > d
    // i^2 - it + d < 0
    // -b +/- sqrt(b^2 - 4ac) / 2a
    // t +/- sqrt(t^2 - 4d) / 2
    // t + sqrt(t^2 - 4d) / 2
    // t - sqrt(t^2 - 4d) / 2

    let t = t as f64;
    let d = d as f64;
    let a = ((t - (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).ceil() as u64;
    let b = ((t + (t.powf(2.0) - 4.0 * d).sqrt()) / 2.0).ceil() as u64;

    Some(b - a)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(288));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(71503));
    }
}
