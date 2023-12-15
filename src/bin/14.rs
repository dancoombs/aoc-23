use std::collections::HashMap;

advent_of_code::solution!(14);

pub fn part_one(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    tilt_north(&mut grid);
    score(&grid).into()
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut m = HashMap::<Vec<Vec<char>>, u64>::new();
    let mut remaining = 0;
    for i in 0..1000000000 {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);

        if let Some(v) = m.get(&grid) {
            println!("Found a loop at {i} {v}");
            remaining = (1000000000 - 1 - i) % (i - v);
            break;
        } else {
            m.insert(grid.clone(), i);
        }
    }

    for _ in 0..remaining {
        tilt_north(&mut grid);
        tilt_west(&mut grid);
        tilt_south(&mut grid);
        tilt_east(&mut grid);
    }

    score(&grid).into()
}

fn score(grid: &Vec<Vec<char>>) -> u32 {
    let mut score = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == 'O' {
                score += grid.len() - i;
            }
        }
    }
    score as u32
}

fn tilt_north(grid: &mut Vec<Vec<char>>) {
    let h = grid.len();
    let w = grid[0].len();

    for i in 1..h {
        for j in 0..w {
            if grid[i][j] != 'O' {
                continue;
            }

            let mut cur_h = i;
            while cur_h >= 1 {
                if grid[cur_h - 1][j] != '.' {
                    break;
                }
                grid[cur_h][j] = '.';
                grid[cur_h - 1][j] = 'O';
                cur_h -= 1;
            }
        }
    }
}

fn tilt_west(grid: &mut Vec<Vec<char>>) {
    let h = grid.len();
    let w = grid[0].len();

    for j in 1..w {
        for i in 0..h {
            if grid[i][j] != 'O' {
                continue;
            }

            let mut cur_w = j;
            while cur_w >= 1 {
                if grid[i][cur_w - 1] != '.' {
                    break;
                }
                grid[i][cur_w] = '.';
                grid[i][cur_w - 1] = 'O';
                cur_w -= 1;
            }
        }
    }
}

fn tilt_south(grid: &mut Vec<Vec<char>>) {
    let h = grid.len();
    let w = grid[0].len();

    for i in (0..h - 1).rev() {
        for j in 0..w {
            if grid[i][j] != 'O' {
                continue;
            }

            let mut cur_h = i;
            while cur_h < h - 1 {
                if grid[cur_h + 1][j] != '.' {
                    break;
                }
                grid[cur_h][j] = '.';
                grid[cur_h + 1][j] = 'O';
                cur_h += 1;
            }
        }
    }
}

fn tilt_east(grid: &mut Vec<Vec<char>>) {
    let h = grid.len();
    let w = grid[0].len();

    for j in (0..w - 1).rev() {
        for i in 0..h {
            if grid[i][j] != 'O' {
                continue;
            }

            let mut cur_w = j;
            while cur_w < w - 1 {
                if grid[i][cur_w + 1] != '.' {
                    break;
                }
                grid[i][cur_w] = '.';
                grid[i][cur_w + 1] = 'O';
                cur_w += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(136));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(64));
    }
}
