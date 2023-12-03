use regex::Regex;

advent_of_code::solution!(3);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    let s = input
        .lines()
        .map(|l| l.chars().collect())
        .collect::<Vec<Vec<char>>>();
    for i in 0..s.len() {
        let mut j = 0;
        while j < s[0].len() {
            if s[i][j].is_digit(10) {
                let mut k = j + 1;
                while k < s[0].len() && s[i][k].is_digit(10) {
                    k += 1;
                }
                let num = s[i][j..k].iter().enumerate().fold(0, |acc, (d, c)| {
                    acc + c.to_digit(10).unwrap() * 10_u32.pow((k - j - d - 1) as u32)
                });

                for (row, col) in get_neighbors(s.len(), s[0].len(), i, j, k - 1) {
                    if s[row][col] != '.' && !s[row][col].is_digit(10) {
                        sum += num;
                        break;
                    }
                }

                j = k;
            } else {
                j += 1;
            }
        }
    }

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let s = input.lines().collect::<Vec<_>>();
    let rows = s.len();
    let cols = s[0].len();
    let mut neighbors = vec![vec![vec![]; cols]; rows];

    let re = Regex::new(r"\d+").unwrap();

    s.iter().enumerate().for_each(|(row, l)| {
        re.find_iter(&l).for_each(|m| {
            let num = m.as_str().parse::<u32>().unwrap();
            let indices = get_neighbors(rows, cols, row, m.start(), m.end() - 1);
            indices.iter().for_each(|(r, c)| {
                neighbors[*r][*c].push(num);
            });
        });
    });

    let sum = s
        .iter()
        .enumerate()
        .map(|(row, l)| {
            l.chars()
                .enumerate()
                .filter(|(_, c)| *c == '*')
                .map(|(col, _)| {
                    if neighbors[row][col].len() == 2 {
                        neighbors[row][col][0] * neighbors[row][col][1]
                    } else {
                        0
                    }
                })
                .sum::<u32>()
        })
        .sum::<u32>();

    Some(sum)
}

// m = num rows
// n = num columns
// l = cur row
// i = start column
// j = end column
fn get_neighbors(m: usize, n: usize, l: usize, i: usize, j: usize) -> Vec<(usize, usize)> {
    let mut ret = Vec::new();
    let start = i.saturating_sub(1);
    let end = (j + 1).min(n - 1);
    if l > 0 {
        for k in start..=end {
            ret.push((l - 1, k));
        }
    }
    if l < m - 1 {
        for k in start..=end {
            ret.push((l + 1, k));
        }
    }
    if start < i {
        ret.push((l, start));
    }
    if end > j {
        ret.push((l, end));
    }
    ret
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(4361));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(467835));
    }
}
