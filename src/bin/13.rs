advent_of_code::solution!(13);

pub fn part_one(input: &str) -> Option<u32> {
    input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.chars().collect()).collect())
        .map(|p| find_reflection(p, (None, None)))
        .map(|r| score_reflections(r).unwrap_or(0))
        .sum::<u32>()
        .into()
}

fn score_reflections(reflections: (Option<u32>, Option<u32>)) -> Option<u32> {
    if let Some(h) = reflections.0 {
        Some((h + 1) * 100)
    } else if let Some(v) = reflections.1 {
        Some(v + 1)
    } else {
        None
    }
}

fn find_reflection(
    cells: Vec<Vec<char>>,
    skips: (Option<u32>, Option<u32>),
) -> (Option<u32>, Option<u32>) {
    let horizontal = check_horizontal_reflection(&cells, skips.0);
    let vertical = check_vertical_reflection(&cells, skips.1);

    (horizontal, vertical)
}

fn check_horizontal_reflection(cells: &Vec<Vec<char>>, skip: Option<u32>) -> Option<u32> {
    if cells.len() < 2 {
        return None;
    }

    let row_ids = cells
        .iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<Vec<_>>();

    check_reflection_by_id(&row_ids, skip)
}

fn check_vertical_reflection(cells: &Vec<Vec<char>>, skip: Option<u32>) -> Option<u32> {
    if cells[0].len() < 2 {
        return None;
    }

    let len = cells[0].len();

    let mut column_ids = vec![];
    for i in 0..len {
        let mut column = String::with_capacity(cells.len());
        for j in 0..cells.len() {
            column.push(cells[j][i]);
        }
        column_ids.push(column);
    }

    check_reflection_by_id(&column_ids, skip)
}

fn check_reflection_by_id(ids: &[String], skip: Option<u32>) -> Option<u32> {
    let len = ids.len();

    for i in 0..len - 1 {
        if skip.is_some_and(|x| x == i as u32) {
            continue;
        }

        let mut up: usize = i;
        let mut down = i + 1;
        let mut is_reflection = true;

        loop {
            if ids[up] != ids[down] {
                is_reflection = false;
                break;
            }
            if up == 0 || down == len - 1 {
                break;
            }

            up -= 1;
            down += 1;
        }

        if is_reflection {
            return Some(i as u32);
        }
    }

    None
}

pub fn part_two(input: &str) -> Option<u32> {
    let patterns = input
        .split("\n\n")
        .map(|p| p.lines().map(|l| l.chars().collect()).collect())
        .collect::<Vec<Vec<Vec<char>>>>();

    let mut sum = 0;

    'pattern: for pattern in patterns {
        let originals = find_reflection(pattern.clone(), (None, None));
        for i in 0..pattern.len() {
            for j in 0..pattern[i].len() {
                let mut new_pattern = pattern.clone();

                if pattern[i][j] == '#' {
                    new_pattern[i][j] = '.';
                } else {
                    new_pattern[i][j] = '#';
                }

                let reflections = find_reflection(new_pattern, originals);
                if let Some(i) = score_reflections(reflections) {
                    sum += i;
                    continue 'pattern;
                }
            }
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
        assert_eq!(result, Some(405));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(400));
    }
}
