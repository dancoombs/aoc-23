advent_of_code::solution!(11);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    // expand
    // expand rows
    let mut i = 0;
    while i < grid.len() {
        let mut has_galaxy = false;
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            grid.insert(i, vec!['.'; grid[0].len()]);
            i += 2;
        } else {
            i += 1;
        }
    }
    i = 0;
    // expand cols
    while i < grid[0].len() {
        let mut has_galaxy = false;
        for j in 0..grid.len() {
            if grid[j][i] == '#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            for j in 0..grid.len() {
                grid[j].insert(i, '.');
            }
            i += 2;
        } else {
            i += 1;
        }
    }

    // print grid
    // for i in 0..grid.len() {
    //     for j in 0..grid[0].len() {
    //         print!("{}", grid[i][j]);
    //     }
    //     println!();
    // }

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
            let (x1, y1) = galaxies[i];
            let (x2, y2) = galaxies[j];
            let dist = (x1 as i32 - x2 as i32).abs() + (y1 as i32 - y2 as i32).abs();
            sum += dist as u32;
        }
    }

    Some(sum)
}

const EMPTY_EXPAND: u64 = 1000000; // modify for real input vs tests

pub fn part_two(input: &str) -> Option<u64> {
    let grid: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    // find empty rows
    let mut empty_rows = vec![false; grid.len()];
    for i in 0..grid.len() {
        let mut has_galaxy = false;
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            empty_rows[i] = true;
        }
    }
    // find empty columns
    let mut empty_cols = vec![false; grid[0].len()];
    for i in 0..grid[0].len() {
        let mut has_galaxy = false;
        for j in 0..grid.len() {
            if grid[j][i] == '#' {
                has_galaxy = true;
                break;
            }
        }
        if !has_galaxy {
            empty_cols[i] = true;
        }
    }

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

            let mut x_dist = 0;
            for x in x1..x2 {
                if empty_rows[x] {
                    x_dist += EMPTY_EXPAND;
                } else {
                    x_dist += 1;
                }
            }
            let mut y_dist = 0;
            for y in y1..y2 {
                if empty_cols[y] {
                    y_dist += EMPTY_EXPAND;
                } else {
                    y_dist += 1;
                }
            }

            let dist = x_dist + y_dist;

            sum += dist;
        }
    }

    Some(sum)
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
