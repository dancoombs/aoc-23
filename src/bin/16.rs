use std::collections::HashSet;

advent_of_code::solution!(16);

pub fn part_one(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    Some(solve_from_start(&grid, 0, 0, (0, 1)))
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut max = 0;
    // top & bottom
    for y in 0..grid[0].len() {
        let result = solve_from_start(&grid, 0, y as i32, (1, 0));
        if result > max {
            max = result;
        }
        let result = solve_from_start(&grid, (grid.len() - 1) as i32, y as i32, (-1, 0));
        if result > max {
            max = result;
        }
    }
    // left & right
    for x in 0..grid.len() {
        let result = solve_from_start(&grid, x as i32, 0, (0, 1));
        if result > max {
            max = result;
        }
        let result = solve_from_start(&grid, x as i32, (grid[0].len() - 1) as i32, (0, -1));
        if result > max {
            max = result;
        }
    }

    Some(max)
}

fn solve_from_start(grid: &Vec<Vec<char>>, x_pos: i32, y_pos: i32, dir: (i32, i32)) -> u32 {
    let height = grid.len() as i32;
    let width = grid[0].len() as i32;
    let mut beams = vec![((x_pos, y_pos), dir)];
    let mut energized = HashSet::new();
    let mut seen = HashSet::new();

    while !beams.is_empty() {
        let ((mut x_pos, mut y_pos), mut dir) = beams.pop().unwrap();

        while x_pos >= 0 && x_pos < height && y_pos >= 0 && y_pos < width {
            if !seen.insert(((x_pos, y_pos), dir)) {
                break;
            }

            energized.insert((x_pos, y_pos));
            match grid[x_pos as usize][y_pos as usize] {
                '|' => {
                    if dir.1 != 0 {
                        dir = (1, 0);
                        beams.push(((x_pos, y_pos), (-1, 0)));
                    }
                }
                '-' => {
                    if dir.0 != 0 {
                        dir = (0, 1);
                        beams.push(((x_pos, y_pos), (0, -1)));
                    }
                }
                '/' => match dir {
                    (1, 0) => dir = (0, -1),
                    (0, 1) => dir = (-1, 0),
                    (-1, 0) => dir = (0, 1),
                    (0, -1) => dir = (1, 0),
                    _ => panic!("Unknown direction: {:?}", dir),
                },
                '\\' => match dir {
                    (1, 0) => dir = (0, 1),
                    (0, 1) => dir = (1, 0),
                    (-1, 0) => dir = (0, -1),
                    (0, -1) => dir = (-1, 0),
                    _ => panic!("Unknown direction: {:?}", dir),
                },
                '.' => {}
                _ => panic!(
                    "Unknown character: {}",
                    grid[x_pos as usize][y_pos as usize]
                ),
            }
            x_pos += dir.0;
            y_pos += dir.1;
        }
    }

    energized.len() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(46));
    }

    // #[test]
    // fn test_part_two() {
    //     let result = part_two(&advent_of_code::template::read_file("examples", DAY));
    //     assert_eq!(result, None);
    // }
}
