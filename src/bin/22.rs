use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(22);

pub fn part_one(input: &str) -> Option<u32> {
    let bricks = parse_bricks(input);
    let above = find_aboves(&bricks);

    let tops = bricks
        .iter()
        .map(|brick| brick.b.2 + brick.height())
        .collect_vec();

    let tops = fall(&bricks, &above, &tops, usize::MAX);

    let mut can = vec![true; bricks.len()];
    for i in 0..bricks.len() {
        let mut num_on_top = 0;
        let mut last_top = 0;
        for j in above[i].iter() {
            if tops[*j] == tops[i] - bricks[i].height() {
                num_on_top += 1;
                last_top = *j;
            }
        }
        if num_on_top == 1 {
            can[last_top] = false;
        }
    }

    Some(can.iter().filter(|&&x| x).count() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    let bricks = parse_bricks(input);
    let above = find_aboves(&bricks);

    let tops = bricks
        .iter()
        .map(|brick| brick.b.2 + brick.height())
        .collect_vec();
    let tops = fall(&bricks, &above, &tops, usize::MAX);

    let mut sum = 0;
    for skip in 0..bricks.len() {
        let new_tops = fall(&bricks, &above, &tops, skip);

        sum += new_tops
            .iter()
            .zip(tops.iter())
            .filter(|(a, b)| a != b)
            .count();
    }

    Some(sum as u32)
}

fn parse_bricks(input: &str) -> Vec<Brick> {
    input
        .lines()
        .map(|line| {
            let (first, second) = line.split("~").collect_tuple().unwrap();
            let (x0, y0, z0) = first.split(",").collect_tuple().unwrap();
            let (x1, y1, z1) = second.split(",").collect_tuple().unwrap();
            Brick::new(
                (
                    x0.parse().unwrap(),
                    y0.parse().unwrap(),
                    z0.parse().unwrap(),
                ),
                (
                    x1.parse().unwrap(),
                    y1.parse().unwrap(),
                    z1.parse().unwrap(),
                ),
            )
        })
        .collect()
}

fn find_aboves(bricks: &[Brick]) -> Vec<HashSet<usize>> {
    let mut above = vec![HashSet::new(); bricks.len()];

    // determine which bricks are above which and intersect
    for (i, brick) in bricks.iter().enumerate() {
        for (j, other) in bricks.iter().enumerate() {
            if i == j {
                continue;
            }

            if brick.is_above(other) && brick.intersects_xy(other) {
                above[i].insert(j);
            }
        }
    }

    above
}

fn fall(bricks: &[Brick], above: &[HashSet<usize>], tops: &[usize], skip: usize) -> Vec<usize> {
    let mut new_tops = tops.to_vec();

    let mut moved = true;
    while moved {
        moved = false;

        for (i, brick) in bricks.iter().enumerate() {
            if i == skip {
                continue;
            }

            let mut max_below_height = 0;
            for j in above[i].iter() {
                if j == &skip {
                    continue;
                }

                if new_tops[*j] > max_below_height {
                    max_below_height = new_tops[*j];
                }
            }
            let new_top = max_below_height + brick.height();
            if new_top < new_tops[i] {
                new_tops[i] = new_top;
                moved = true;
            }
        }
    }

    new_tops
}

// xyz
type Point = (usize, usize, usize);

struct Brick {
    a: Point,
    b: Point,
}

impl Brick {
    fn new(a: Point, b: Point) -> Self {
        let (min_x, max_x) = if a.0 < b.0 { (a.0, b.0) } else { (b.0, a.0) };
        let (min_y, max_y) = if a.1 < b.1 { (a.1, b.1) } else { (b.1, a.1) };
        let (min_z, max_z) = if a.2 < b.2 { (a.2, b.2) } else { (b.2, a.2) };

        Self {
            a: (min_x, min_y, min_z),
            b: (max_x, max_y, max_z),
        }
    }

    fn is_above(&self, other: &Self) -> bool {
        self.a.2 > other.a.2
    }

    fn intersects_xy(&self, other: &Self) -> bool {
        self.a.0 <= other.b.0
            && self.b.0 >= other.a.0
            && self.a.1 <= other.b.1
            && self.b.1 >= other.a.1
    }

    fn height(&self) -> usize {
        self.b.2 - self.a.2 + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
}
