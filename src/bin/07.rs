use std::collections::HashMap;

use itertools::Itertools;
use lazy_static::lazy_static;

advent_of_code::solution!(7);

pub fn part_one(input: &str) -> Option<u32> {
    Some(solve(input, false))
}

pub fn part_two(input: &str) -> Option<u32> {
    Some(solve(input, true))
}

fn solve(input: &str, is_joker_hand: bool) -> u32 {
    let mut hands: Vec<(Hand, u32)> = input
        .lines()
        .map(|l| {
            let p = l
                .split_ascii_whitespace()
                .collect_tuple::<(&str, &str)>()
                .unwrap();
            (Hand::new(p.0, is_joker_hand), p.1.parse::<u32>().unwrap())
        })
        .collect();

    hands.sort_by(|(a, _), (b, _)| a.cmp(b));

    hands
        .iter()
        .enumerate()
        .fold(0, |acc, (i, (_, v))| acc + v * (1 + i as u32))
}

#[derive(Debug)]
struct Hand {
    category: Category,
    cards: Vec<char>,
    is_joker_hand: bool,
}

impl Hand {
    fn new(hand: &str, is_joker_hand: bool) -> Self {
        let cards = hand.chars().collect::<Vec<char>>();
        let mut counts = HashMap::new();
        for c in cards.iter() {
            *counts.entry(c).or_insert(0) += 1;
        }

        let values = if is_joker_hand {
            let jokers = counts.remove(&'J').unwrap_or(0);
            let mut values = counts.values().cloned().collect::<Vec<u32>>();
            values.sort();

            if values.len() > 0 {
                let last = values.len() - 1;
                values[last] += jokers;
            } else {
                values.push(jokers);
            }
            values
        } else {
            let mut values = counts.values().cloned().collect::<Vec<u32>>();
            values.sort();
            values
        };

        let category = match values.as_slice() {
            [1, 1, 1, 1, 1] => Category::HighCard,
            [1, 1, 1, 2] => Category::OnePair,
            [1, 2, 2] => Category::TwoPair,
            [1, 1, 3] => Category::ThreeKind,
            [2, 3] => Category::FullHouse,
            [1, 4] => Category::FourKind,
            [5] => Category::FiveKind,
            _ => panic!("Invalid hand: {:?}", hand),
        };

        Self {
            category,
            cards: cards.clone(),
            is_joker_hand,
        }
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.category == other.category && self.cards == other.cards
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.category.cmp(&other.category) {
            std::cmp::Ordering::Equal => {
                for (a, b) in self.cards.iter().zip(other.cards.iter()) {
                    let (a_s, b_s) = if self.is_joker_hand {
                        (
                            JOKER_STRENGTH.get(a).unwrap(),
                            JOKER_STRENGTH.get(b).unwrap(),
                        )
                    } else {
                        (STRENGTH.get(a).unwrap(), STRENGTH.get(b).unwrap())
                    };
                    match a_s.cmp(b_s) {
                        std::cmp::Ordering::Equal => continue,
                        o => return o,
                    }
                }

                std::cmp::Ordering::Equal
            }
            o => o,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Category {
    HighCard,
    OnePair,
    TwoPair,
    ThreeKind,
    FullHouse,
    FourKind,
    FiveKind,
}

lazy_static! {
    static ref STRENGTH: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('J', 11),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
    ]);
    static ref JOKER_STRENGTH: HashMap<char, u32> = HashMap::from([
        ('A', 14),
        ('K', 13),
        ('Q', 12),
        ('T', 10),
        ('9', 9),
        ('8', 8),
        ('7', 7),
        ('6', 6),
        ('5', 5),
        ('4', 4),
        ('3', 3),
        ('2', 2),
        ('J', 1),
    ]);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(6440));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5905));
    }
}
