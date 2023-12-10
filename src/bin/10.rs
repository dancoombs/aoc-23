use std::collections::HashSet;

advent_of_code::solution!(10);

pub fn part_one(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|l| l.chars().map(Into::into).collect())
        .collect();

    let loop_ = find_loop(&grid);
    Some(loop_.len() as u32 / 2)
}

pub fn part_two(input: &str) -> Option<u32> {
    let grid: Vec<Vec<Cell>> = input
        .lines()
        .map(|l| l.chars().map(Into::into).collect())
        .collect();

    let loop_ = find_loop(&grid);
    let in_loop = loop_.iter().cloned().collect::<HashSet<(i32, i32)>>();
    let mut count = 0;
    for i in 0..grid.len() {
        for j in 0..grid[0].len() {
            if in_loop.contains(&(i as i32, j as i32)) {
                continue;
            }

            let mut x = i;
            let mut y = j;
            let mut crosses = 0;

            while x < grid.len() && y < grid[0].len() {
                if in_loop.contains(&(x as i32, y as i32)) {
                    let cell = &grid[x][y];
                    if !(cell.down && cell.left) && !(cell.up && cell.right) {
                        crosses += 1;
                    }
                }
                x += 1;
                y += 1;
            }

            if crosses % 2 == 1 {
                count += 1;
            }
        }
    }

    // TODO this overcounts or undercounts by 1 on tests and input. Not sure why

    Some(count)
}

fn find_loop(grid: &Vec<Vec<Cell>>) -> Vec<(i32, i32)> {
    let rows = grid.len();
    let cols = grid[0].len();

    let mut x = 0;
    let mut y = 0;
    for i in 0..rows {
        for j in 0..cols {
            if grid[i][j].is_start() {
                x = i;
                y = j;
                break;
            }
        }
    }

    let start = (x as i32, y as i32);
    for dir in DIRS {
        let mut dir = dir;
        let mut cur = start;
        let mut cur_loop = vec![];

        loop {
            cur_loop.push(cur);
            let next = (cur.0 + dir.0, cur.1 + dir.1);
            if next.0 < 0 || next.0 >= rows as i32 || next.1 < 0 || next.1 >= cols as i32 {
                break;
            }
            let next_cell = &grid[next.0 as usize][next.1 as usize];

            if next_cell.is_start() {
                return cur_loop;
            }
            if !next_cell.can_enter(dir) {
                break;
            }

            cur = next;
            dir = next_cell.next_dir(dir);
        }
    }

    panic!("No loop found");
}

const DIRS: [(i32, i32); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];

#[derive(Debug)]
struct Cell {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Cell {
    fn is_start(&self) -> bool {
        self.up && self.down && self.left && self.right
    }

    fn can_enter(&self, dir: (i32, i32)) -> bool {
        match dir {
            (0, 1) => self.left,
            (0, -1) => self.right,
            (-1, 0) => self.down,
            (1, 0) => self.up,
            _ => panic!("Unknown direction: {:?}", dir),
        }
    }

    fn next_dir(&self, dir: (i32, i32)) -> (i32, i32) {
        match dir {
            (0, 1) => {
                if self.up {
                    (-1, 0)
                } else if self.down {
                    (1, 0)
                } else if self.right {
                    (0, 1)
                } else {
                    panic!("No valid direction")
                }
            }
            (0, -1) => {
                if self.up {
                    (-1, 0)
                } else if self.down {
                    (1, 0)
                } else if self.left {
                    (0, -1)
                } else {
                    panic!("No valid direction")
                }
            }
            (-1, 0) => {
                if self.right {
                    (0, 1)
                } else if self.left {
                    (0, -1)
                } else if self.up {
                    (-1, 0)
                } else {
                    panic!("No valid direction")
                }
            }
            (1, 0) => {
                if self.right {
                    (0, 1)
                } else if self.left {
                    (0, -1)
                } else if self.down {
                    (1, 0)
                } else {
                    panic!("No valid direction")
                }
            }
            _ => panic!("Unknown direction: {:?}", dir),
        }
    }
}

impl From<char> for Cell {
    fn from(c: char) -> Self {
        match c {
            '.' => Cell {
                up: false,
                down: false,
                left: false,
                right: false,
            },
            'S' => Cell {
                up: true,
                down: true,
                left: true,
                right: true,
            },
            '|' => Cell {
                up: true,
                down: true,
                left: false,
                right: false,
            },
            '-' => Cell {
                up: false,
                down: false,
                left: true,
                right: true,
            },
            '7' => Cell {
                up: false,
                down: true,
                left: true,
                right: false,
            },
            'L' => Cell {
                up: true,
                down: false,
                left: false,
                right: true,
            },
            'J' => Cell {
                up: true,
                down: false,
                left: true,
                right: false,
            },
            'F' => Cell {
                up: false,
                down: true,
                left: false,
                right: true,
            },
            _ => panic!("Unknown cell type: {}", c),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file_part(
            "examples", DAY, 1,
        ));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file_part(
            "examples", DAY, 2,
        ));
        assert_eq!(result, Some(10));
    }
}
