use itertools::Itertools;
use z3::{
    ast::{Ast, Int},
    Config, Context, Solver,
};

advent_of_code::solution!(24);

const TEST_AREA_MIN: f64 = 200000000000000.0;
const TEST_AREA_MAX: f64 = 400000000000000.0;

pub fn part_one(input: &str) -> Option<u32> {
    let hailstones = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let (px, py, _) = p.split(", ").collect_tuple().unwrap();
            let (vx, vy, _) = v.split(", ").collect_tuple().unwrap();

            let (px, py) = (
                px.trim_start().parse::<i64>().unwrap() as f64,
                py.trim_start().parse::<i64>().unwrap() as f64,
            );
            let (vx, vy) = (
                vx.trim_start().parse::<i64>().unwrap() as f64,
                vy.trim_start().parse::<i64>().unwrap() as f64,
            );

            // -m * x + (1) y + (m * px - py)
            let m = vy / vx;
            let c = m * px - py;

            (-m, 1.0, c, px, vx > 0.0)
        })
        .collect_vec();

    let mut cnt = 0;
    for i in 0..hailstones.len() {
        for j in i + 1..hailstones.len() {
            let (a1, b1, c1, x1, f1) = hailstones[i];
            let (a2, b2, c2, x2, f2) = hailstones[j];

            let d = a1 * b2 - a2 * b1;

            let x = (b1 * c2 - b2 * c1) / d;
            let y = (c1 * a2 - c2 * a1) / d;

            if x >= TEST_AREA_MIN && x <= TEST_AREA_MAX && y >= TEST_AREA_MIN && y <= TEST_AREA_MAX
            {
                if f1 && x < x1 || !f1 && x > x1 || f2 && x < x2 || !f2 && x > x2 {
                    continue;
                }
                cnt += 1;
            }
        }
    }

    Some(cnt)
}

// taken from https://github.com/arthomnix/aoc23/blob/master/src/days/day24.rs
pub fn part_two(input: &str) -> Option<u64> {
    let hailstones = input
        .lines()
        .map(|l| {
            let (p, v) = l.split_once(" @ ").unwrap();
            let (px, py, pz) = p.split(", ").collect_tuple().unwrap();
            let (vx, vy, vz) = v.split(", ").collect_tuple().unwrap();

            (
                (
                    px.trim_start().parse::<i64>().unwrap(),
                    py.trim_start().parse::<i64>().unwrap(),
                    pz.trim_start().parse::<i64>().unwrap(),
                ),
                (
                    vx.trim_start().parse::<i64>().unwrap(),
                    vy.trim_start().parse::<i64>().unwrap(),
                    vz.trim_start().parse::<i64>().unwrap(),
                ),
            )
        })
        .collect_vec();

    let cfg = Config::new();
    let ctx = Context::new(&cfg);
    let solver = Solver::new(&ctx);

    let px = Int::new_const(&ctx, "px");
    let py = Int::new_const(&ctx, "py");
    let pz = Int::new_const(&ctx, "pz");
    let vx = Int::new_const(&ctx, "vx");
    let vy = Int::new_const(&ctx, "vy");
    let vz = Int::new_const(&ctx, "vz");

    for hailstone in &hailstones {
        let pxn = Int::from_i64(&ctx, hailstone.0 .0);
        let pyn = Int::from_i64(&ctx, hailstone.0 .1);
        let pzn = Int::from_i64(&ctx, hailstone.0 .2);
        let vxn = Int::from_i64(&ctx, hailstone.1 .0);
        let vyn = Int::from_i64(&ctx, hailstone.1 .1);
        let vzn = Int::from_i64(&ctx, hailstone.1 .2);
        let tn = Int::fresh_const(&ctx, "t");

        solver.assert(&(&pxn + &vxn * &tn)._eq(&(&px + &vx * &tn)));
        solver.assert(&(&pyn + &vyn * &tn)._eq(&(&py + &vy * &tn)));
        solver.assert(&(&pzn + &vzn * &tn)._eq(&(&pz + &vz * &tn)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&px).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&py).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&pz).unwrap().as_i64().unwrap();
    Some((x + y + z) as u64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(47));
    }
}
