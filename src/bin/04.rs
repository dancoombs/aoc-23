advent_of_code::solution!(4);

use std::collections::HashSet;

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(|l| {
            let matched = card_matching(l);

            if matched > 0 {
                2_u32.pow(matched as u32 - 1)
            } else {
                0
            }
        })
        .sum();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let cards = input.lines().map(card_matching).collect::<Vec<_>>();
    let mut hand = vec![1_u32; cards.len()];
    for i in 0..cards.len() {
        for j in (i + 1)..(i + 1 + cards[i] as usize) {
            if j < cards.len() {
                hand[j] += hand[i];
            }
        }
    }

    Some(hand.iter().sum())
}

fn card_matching(card: &str) -> u32 {
    let (winning, draw) = card
        .split(": ")
        .nth(1)
        .unwrap()
        .split(" | ")
        .collect_tuple()
        .unwrap();

    let winning = winning
        .split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<HashSet<_>>();

    draw.split_ascii_whitespace()
        .map(|n| n.parse::<u32>().unwrap())
        .filter(|n| winning.contains(n))
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(13));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(30));
    }
}
