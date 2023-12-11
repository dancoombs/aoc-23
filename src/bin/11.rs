advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u64> {
    Some(solve(input, 2))
}

const EMPTY_EXPAND: u64 = 100; // modify for real input vs tests

pub fn part_two(input: &str) -> Option<u64> {
    Some(solve(input, EMPTY_EXPAND))
}

fn solve(input: &str, empty_distance: u64) -> u64 {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // find empty rows
    let empty_rows = (0..grid.len())
        .map(|i| grid[i].iter().all(|&c| c == '.'))
        .collect::<Vec<_>>();

    // find empty columns
    let empty_cols = (0..grid[0].len())
        .map(|j| (0..grid.len()).map(|i| grid[i][j]).all(|c| c == '.'))
        .collect::<Vec<_>>();

    // find galaxies
    let mut galaxies = vec![];
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '.' {
                continue;
            }
            galaxies.push((i, j));
        }
    }

    // distances
    let mut sum = 0;
    for i in 0..galaxies.len() {
        for j in (i + 1)..galaxies.len() {
            let (mut x1, mut y1) = galaxies[i];
            let (mut x2, mut y2) = galaxies[j];
            if x2 < x1 {
                std::mem::swap(&mut x1, &mut x2);
            }
            if y2 < y1 {
                std::mem::swap(&mut y1, &mut y2);
            }

            let mut dist = 0;
            for x in x1..x2 {
                if empty_rows[x] {
                    dist += empty_distance;
                } else {
                    dist += 1;
                }
            }
            for y in y1..y2 {
                if empty_cols[y] {
                    dist += empty_distance;
                } else {
                    dist += 1;
                }
            }

            sum += dist;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(374));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8410));
    }
}
