advent_of_code::solution!(18);

use itertools::Itertools;

pub fn part_one(input: &str) -> Option<u32> {
    let insts = input
        .lines()
        .map(|l| {
            let (dir, cnt, code) = l.split(' ').collect_tuple().unwrap();
            let cnt = cnt.parse::<i32>().unwrap();
            (dir, cnt, code)
        })
        .collect::<Vec<_>>();

    let mut cur = (0_i32, 0_i32);
    let mut max_i = 0;
    let mut min_i = 0;
    let mut max_j = 0;
    let mut min_j = 0;

    for inst in &insts {
        match inst.0 {
            "R" => {
                cur.1 += inst.1;
                if cur.1 > max_j {
                    max_j = cur.1;
                }
            }
            "L" => {
                cur.1 -= inst.1;
                if cur.1 < min_j {
                    min_j = cur.1;
                }
            }
            "U" => {
                cur.0 += inst.1;
                if cur.0 > max_i {
                    max_i = cur.0;
                }
            }
            "D" => {
                cur.0 -= inst.1;
                if cur.0 < min_i {
                    min_i = cur.0;
                }
            }
            _ => panic!("Invalid direction"),
        }
    }

    let mut grid = vec![vec!['.'; (max_j - min_j + 1) as usize]; (max_i - min_i + 1) as usize];
    let mut cur = (-min_i, -min_j);

    for inst in insts {
        match inst.0 {
            "R" => {
                for _ in 0..inst.1 {
                    cur.1 += 1;
                    grid[cur.0 as usize][cur.1 as usize] = '#';
                }
            }
            "L" => {
                for _ in 0..inst.1 {
                    cur.1 -= 1;
                    grid[cur.0 as usize][cur.1 as usize] = '#';
                }
            }
            "U" => {
                for _ in 0..inst.1 {
                    cur.0 += 1;
                    grid[cur.0 as usize][cur.1 as usize] = '#';
                }
            }
            "D" => {
                for _ in 0..inst.1 {
                    cur.0 -= 1;
                    grid[cur.0 as usize][cur.1 as usize] = '#';
                }
            }
            _ => panic!("Invalid direction"),
        }
    }

    for row in grid.iter().rev() {
        println!("{:?}", row.iter().collect::<String>());
    }
    println!("");

    let mut cnt = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if grid[i][j] == '#' {
                cnt += 1;
                continue;
            }

            let mut ray = (i, j);
            let mut intersects = 0;
            while ray.1 < grid[0].len() {
                if grid[ray.0][ray.1] == '#'
                    && ray.0 + 1 < grid.len()
                    && grid[ray.0 + 1][ray.1] == '#'
                {
                    intersects += 1;
                }
                ray.1 += 1;
            }
            if intersects % 2 == 1 {
                grid[i][j] = '#';
                cnt += 1;
            }
        }
    }

    for row in grid.iter().rev() {
        println!("{:?}", row.iter().collect::<String>());
    }

    Some(cnt)
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(62));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
