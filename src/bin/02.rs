advent_of_code::solution!(2);

pub fn part_one(input: &str) -> Option<u32> {
    let games = input.lines().map(parse_game).collect::<Vec<_>>();

    let max_red = 12_u32;
    let max_blue = 14_u32;
    let max_green = 13_u32;

    let sum = games
        .iter()
        .enumerate()
        .filter_map(|(i, g)| {
            if g.iter()
                .find(|g| g[0] > max_red || g[1] > max_blue || g[2] > max_green)
                .is_some()
            {
                None
            } else {
                Some(i as u32 + 1)
            }
        })
        .sum::<u32>();

    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let sum = input
        .lines()
        .map(parse_game)
        .map(|g| {
            let mut max_red = 0;
            let mut max_blue = 0;
            let mut max_green = 0;
            g.iter().for_each(|g| {
                if g[0] > max_red {
                    max_red = g[0];
                }
                if g[1] > max_blue {
                    max_blue = g[1];
                }
                if g[2] > max_green {
                    max_green = g[2];
                }
            });
            max_red * max_blue * max_green
        })
        .sum();
    Some(sum)
}

fn parse_game(input: &str) -> Vec<[u32; 3]> {
    let g = input.split(": ").collect::<Vec<_>>()[1];
    g.split("; ")
        .map(|r| {
            let mut ret = [0_u32; 3];
            r.split(", ").for_each(|c| {
                let x = c.split(" ").collect::<Vec<_>>();
                let count = x[0].parse::<u32>().unwrap();
                match x[1] {
                    "red" => ret[0] = count,
                    "blue" => ret[1] = count,
                    "green" => ret[2] = count,
                    _ => panic!("Invalid input"),
                }
            });
            ret
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
