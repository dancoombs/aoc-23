use std::collections::HashSet;

use itertools::Itertools;

advent_of_code::solution!(23);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);

    let mut seen = HashSet::new();
    find_longest_path_slippery(&grid, start, end, &mut seen, 0)
}

fn find_longest_path_slippery(
    grid: &Vec<Vec<char>>,
    cur: (usize, usize),
    goal: (usize, usize),
    seen: &mut HashSet<(usize, usize)>,
    cur_len: u32,
) -> Option<u32> {
    if cur == goal {
        return Some(cur_len);
    }

    let mut max: Option<u32> = None;

    for (x, y) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
        if grid[cur.0][cur.1] == '>' && y != 1 {
            continue;
        } else if grid[cur.0][cur.1] == '<' && y != -1 {
            continue;
        } else if grid[cur.0][cur.1] == '^' && x != -1 {
            continue;
        } else if grid[cur.0][cur.1] == 'v' && x != 1 {
            continue;
        }

        let next = (cur.0 as i32 + x, cur.1 as i32 + y);
        if next.0 < 0 || next.1 < 0 || next.0 >= grid.len() as i32 || next.1 >= grid[0].len() as i32
        {
            continue;
        }

        let next = (next.0 as usize, next.1 as usize);
        if seen.contains(&next) {
            continue;
        }

        if grid[next.0][next.1] == '#' {
            continue;
        } else if grid[next.0][next.1] == '<' && y == 1 {
            continue;
        } else if grid[next.0][next.1] == '>' && y == -1 {
            continue;
        } else if grid[next.0][next.1] == '^' && x == 1 {
            continue;
        } else if grid[next.0][next.1] == 'v' && x == -1 {
            continue;
        }

        seen.insert(cur);
        let dist = find_longest_path_slippery(grid, next, goal, seen, cur_len + 1);
        seen.remove(&cur);

        if let Some(dist) = dist {
            max = Some(max.map_or(dist, |m| m.max(dist)));
        }
    }

    max
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|line| line.chars().collect_vec())
        .collect_vec();

    let start = (0, 1);
    let end = (grid.len() - 1, grid[0].len() - 2);

    let mut seen = HashSet::new();
    find_longest_path(&grid, start, end, &mut seen, (0, 0), 0)
}

fn find_longest_path(
    grid: &Vec<Vec<char>>,
    mut cur: (usize, usize),
    goal: (usize, usize),
    forks_seen: &mut HashSet<(usize, usize)>,
    mut prev: (usize, usize),
    mut cur_len: u32,
) -> Option<u32> {
    // find forks and dead ends
    let mut options = vec![];
    loop {
        for (x, y) in vec![(0, 1), (0, -1), (1, 0), (-1, 0)] {
            let next = (cur.0 as i32 + x, cur.1 as i32 + y);
            if next.0 < 0
                || next.1 < 0
                || next.0 >= grid.len() as i32
                || next.1 >= grid[0].len() as i32
            {
                continue;
            }
            let next = (next.0 as usize, next.1 as usize);
            if next == prev || grid[next.0][next.1] == '#' {
                continue;
            }
            options.push(next);
        }

        if options.len() == 1 {
            prev = cur;
            cur = options[0];
            cur_len += 1;
            options.clear();
        } else {
            break;
        }
    }

    // dead ends
    if cur == goal {
        return Some(cur_len);
    } else if options.is_empty() {
        return None;
    }

    // forks
    if forks_seen.contains(&cur) {
        return None;
    }

    forks_seen.insert(cur);

    let mut max: Option<u32> = None;
    for next in options {
        if let Some(dist) = find_longest_path(grid, next, goal, forks_seen, cur, cur_len + 1) {
            max = Some(max.map_or(dist, |m| m.max(dist)));
        }
    }

    forks_seen.remove(&cur);

    max
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(154));
    }
}
