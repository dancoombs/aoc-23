use std::collections::{HashMap, VecDeque};

advent_of_code::solution!(17);

pub fn part_one(input: &str) -> Option<u32> {
    solve(input, 0, 3).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    solve(input, 4, 10).into()
}

fn solve(input: &str, min_dist: usize, max_dist: usize) -> u32 {
    let grid = input
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| (c.to_string()).parse().unwrap())
                .collect::<Vec<u32>>()
        })
        .collect::<Vec<_>>();

    let height = grid.len();
    let width = grid[0].len();
    let mut queue = VecDeque::new();

    let mut dists = HashMap::new();
    queue.push_back(((0, 0), (0, 1), 0, 0));
    dists.insert(((0, 0), (0, 1), 0), 0_u32);
    queue.push_back(((0, 0), (1, 0), 0, 0));
    dists.insert(((0, 0), (1, 0), 0), 0_u32);

    let mut mins = vec![vec![u32::MAX; width]; height];

    while !queue.is_empty() {
        let ((x, y), dir, cnt, dist) = queue.pop_front().unwrap();
        if dist < mins[x as usize][y as usize] && cnt >= min_dist {
            mins[x as usize][y as usize] = dist;
        }

        let mut update = |(x, y): (i32, i32), dir: (i32, i32), cnt: usize, dist: u32| {
            let new_x = x + dir.0;
            let new_y = y + dir.1;
            if new_x < 0 || new_y < 0 || new_x >= height as i32 || new_y >= width as i32 {
                return;
            }

            let cur = dists
                .entry(((new_x, new_y), dir, cnt + 1))
                .or_insert(u32::MAX);
            let inc = grid[new_x as usize][new_y as usize];
            if *cur > dist + inc {
                *cur = dist + inc;
                queue.push_back(((new_x, new_y), dir, cnt + 1, dist + inc));
            }
        };

        // new dir
        match dir {
            (0, 1) => {
                if cnt < max_dist {
                    update((x, y), (0, 1), cnt, dist);
                }

                if cnt >= min_dist {
                    update((x, y), (1, 0), 0, dist);
                    update((x, y), (-1, 0), 0, dist);
                }
            }
            (0, -1) => {
                if cnt < max_dist {
                    update((x, y), (0, -1), cnt, dist);
                }

                if cnt >= min_dist {
                    update((x, y), (1, 0), 0, dist);
                    update((x, y), (-1, 0), 0, dist);
                }
            }
            (1, 0) => {
                if cnt < max_dist {
                    update((x, y), (1, 0), cnt, dist);
                }

                if cnt >= min_dist {
                    update((x, y), (0, 1), 0, dist);
                    update((x, y), (0, -1), 0, dist);
                }
            }
            (-1, 0) => {
                if cnt < max_dist {
                    update((x, y), (-1, 0), cnt, dist);
                }

                if cnt >= min_dist {
                    update((x, y), (0, 1), 0, dist);
                    update((x, y), (0, -1), 0, dist);
                }
            }
            _ => unreachable!(),
        }
    }

    mins[height - 1][width - 1]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(102));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(94));
    }
}
