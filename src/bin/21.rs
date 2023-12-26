use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(21);

const DIRS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input.lines().map(|l| l.chars().collect_vec()).collect_vec();
    let mut locs = HashSet::new();
    'found: for i in 0..grid.len() {
        for j in 0..grid[i].len() {
            if grid[i][j] == 'S' {
                locs.insert((i as i32, j as i32));
                break 'found;
            }
        }
    }

    for _ in 0..64 {
        let mut new_locs = HashSet::new();
        for (i, j) in locs {
            for dir in DIRS {
                let new_i = i + dir.0;
                let new_j = j + dir.1;
                if new_i >= 0
                    && new_j >= 0
                    && new_i < grid.len() as i32
                    && new_j < grid[0].len() as i32
                {
                    if grid[new_i as usize][new_j as usize] == '.'
                        || grid[new_i as usize][new_j as usize] == 'S'
                    {
                        new_locs.insert((new_i, new_j));
                    }
                }
            }
        }
        locs = new_locs;
    }

    Some(locs.len() as u32)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part_one() {
    //     let result = part_one(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, Some(16));
    // }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
