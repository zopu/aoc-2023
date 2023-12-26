use color_eyre::eyre::anyhow;
use color_eyre::Result;
use nom::bytes::complete::tag;
use nom::character::complete::{i128 as ni128, multispace0};
use nom::sequence::tuple;
use nom::IResult;
use z3::ast::Ast;

#[derive(Debug, PartialEq)]
struct Line {
    at: (f64, f64, f64),
    dir: (f64, f64, f64),
}

impl Line {
    fn new(at: (f64, f64, f64), dir: (f64, f64, f64)) -> Self {
        Self { at, dir }
    }

    fn slope_and_offset2d(&self) -> (f64, f64) {
        let (x1, y1, _z1) = self.at;
        let (x2, y2, _z2) = (
            (self.at.0 + self.dir.0),
            (self.at.1 + self.dir.1),
            (self.at.2 + self.dir.2),
        );
        let a = (y2 - y1) / (x2 - x1);
        let b = y1 - a * x1;
        (a, b)
    }

    fn intersects2d(&self, other: &Line) -> Option<(f64, f64)> {
        let (a, c) = self.slope_and_offset2d();
        let (b, d) = other.slope_and_offset2d();

        if a == b {
            // println!("Hailstones {:?}, {:?} paths are parallel", self, other);
            return None;
        }

        let p_x = (d - c) / (a - b);
        let p_y = a * p_x + c;

        if !self.point_in_future(p_x, p_y) {
            // println!("Intersection in past between {:?} and {:?}", self, other);
            return None;
        }
        if !other.point_in_future(p_x, p_y) {
            // println!("Intersection in past between {:?} and {:?}", self, other);
            return None;
        }

        Some((p_x, p_y))
    }

    fn point_in_future(&self, x2: f64, y2: f64) -> bool {
        let (x1, y1, _z1) = self.at;
        let (dx, dy, _dz) = self.dir;
        if dx > 0.0 && x2 < x1 {
            return false;
        }
        if dx < 0.0 && x2 > x1 {
            return false;
        }
        if dy > 0.0 && y2 < y1 {
            return false;
        }
        if dy < 0.0 && y2 > y1 {
            return false;
        }
        true
    }

    fn normalize(&self) -> Line {
        let (dx, dy, dz) = self.dir;
        let len = (dx * dx + dy * dy + dz * dz).sqrt();
        Line::new(self.at, (dx / len, dy / len, dz / len))
    }
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let p1 = part1(input, 200_000_000_000_000.0, 400_000_000_000_000.0)?;
    // let p2 = part2(input)?;
    let p2 = part2_z3(input)?;
    Ok((p1, p2))
}

#[allow(unused)]
fn part2_geo(input: &str) -> Result<u64> {
    let lines: Vec<_> = input
        .lines()
        .map(|l| Ok(parse_line(l).map_err(|e| anyhow!("Parse error: {e}"))?.1))
        .collect::<Result<Vec<_>>>()?;

    // For each pair of lines, we can find their intersection and the plane that
    // contains both of them.

    // Our rock's path must either lie on this plane or intersect it at their intersection point

    // Find the plane containing lines 1 and 2
    todo!("This doesn't work because these lines don't intersect");
    println!("Lines: {:?}", lines);
    let plane1_normal = normalize(cross_product(
        lines[0].normalize().dir,
        lines[1].normalize().dir,
    ));

    // Find the plane containing lines 3 and 4
    let plane2_normal = normalize(cross_product(
        lines[2].normalize().dir,
        lines[3].normalize().dir,
    ));

    println!("Plane 1 normal: {:?}", plane1_normal);
    println!("Plane 2 normal: {:?}", plane2_normal);

    // Find an intersection point b/w plane 1 and line 3
    let intersection_point =
        line_plane_intersection(&lines[3].normalize(), lines[0].at, plane1_normal);
    println!("Intersection point: {:?}", intersection_point);

    // Find the intersection of these two planes, which is a line
    let intersection_dir = cross_product(plane1_normal, plane2_normal);

    println!("Intersection line dir: {:?}", intersection_dir);

    let test_line = Line::new((24.0, 13.0, 10.0), (-3.0, 1.0, 2.0));
    println!(
        "test: {:?}",
        line_plane_intersection(&test_line, lines[0].at, plane1_normal)
    );

    println!(
        "test: {:?}",
        line_plane_intersection(&test_line, lines[1].at, plane1_normal)
    );

    // Find the line's time at intersection with line 5
    // Work out where this line is at time 0

    Ok(0)
}

fn part2_z3(input: &str) -> Result<u64> {
    let lines: Vec<_> = input
        .lines()
        .map(|l| Ok(parse_line(l).map_err(|e| anyhow!("Parse error: {e}"))?.1))
        .collect::<Result<Vec<_>>>()?;

    let cfg = z3::Config::new();
    let ctx = z3::Context::new(&cfg);
    let solver = z3::Solver::new(&ctx);

    let x = z3::ast::Int::new_const(&ctx, "x");
    let y = z3::ast::Int::new_const(&ctx, "y");
    let z = z3::ast::Int::new_const(&ctx, "z");
    let dx = z3::ast::Int::new_const(&ctx, "dx");
    let dy = z3::ast::Int::new_const(&ctx, "dy");
    let dz = z3::ast::Int::new_const(&ctx, "dz");

    for hailstone in lines {
        let hs_x = z3::ast::Int::from_i64(&ctx, hailstone.at.0 as i64);
        let hs_y = z3::ast::Int::from_i64(&ctx, hailstone.at.1 as i64);
        let hs_z = z3::ast::Int::from_i64(&ctx, hailstone.at.2 as i64);
        let hs_dx = z3::ast::Int::from_i64(&ctx, hailstone.dir.0 as i64);
        let hs_dy = z3::ast::Int::from_i64(&ctx, hailstone.dir.1 as i64);
        let hs_dz = z3::ast::Int::from_i64(&ctx, hailstone.dir.2 as i64);
        let t_n = z3::ast::Int::fresh_const(&ctx, "t");

        solver.assert(&(&hs_x + &hs_dx * &t_n)._eq(&(&x + &dx * &t_n)));
        solver.assert(&(&hs_y + &hs_dy * &t_n)._eq(&(&y + &dy * &t_n)));
        solver.assert(&(&hs_z + &hs_dz * &t_n)._eq(&(&z + &dz * &t_n)));
    }

    solver.check();
    let model = solver.get_model().unwrap();
    let x = model.get_const_interp(&x).unwrap().as_i64().unwrap();
    let y = model.get_const_interp(&y).unwrap().as_i64().unwrap();
    let z = model.get_const_interp(&z).unwrap().as_i64().unwrap();

    // 6728171924899227280 too high
    Ok((x + y + z) as u64)
}

fn normalize(dir: (f64, f64, f64)) -> (f64, f64, f64) {
    let (dx, dy, dz) = dir;
    let len = (dx * dx + dy * dy + dz * dz).sqrt();
    (dx / len, dy / len, dz / len)
}

fn line_plane_intersection(
    line: &Line,
    plane_point: (f64, f64, f64),
    plane_normal: (f64, f64, f64),
) -> (f64, f64, f64) {
    // Calculate the intersection point
    let line_origin_to_plane_point = (
        line.at.0 - plane_point.0,
        line.at.1 - plane_point.1,
        line.at.2 - plane_point.2,
    );

    let dot_product = line_origin_to_plane_point.0 * plane_normal.0
        + line_origin_to_plane_point.1 * plane_normal.1
        + line_origin_to_plane_point.2 * plane_normal.2;

    let line_direction_dot_plane_normal =
        line.dir.0 * plane_normal.0 + line.dir.1 * plane_normal.1 + line.dir.2 * plane_normal.2;

    let t = -dot_product / line_direction_dot_plane_normal;

    (
        line.at.0 + t * line.dir.0,
        line.at.1 + t * line.dir.1,
        line.at.2 + t * line.dir.2,
    )
}

fn cross_product(dir1: (f64, f64, f64), dir2: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        dir1.1 * dir2.2 - dir1.2 * dir2.1,
        dir1.2 * dir2.0 - dir1.0 * dir2.2,
        dir1.0 * dir2.1 - dir1.1 * dir2.0,
    )
}

fn part1(input: &str, min_xy: f64, max_xy: f64) -> Result<u64> {
    let lines: Vec<_> = input
        .lines()
        .map(|l| Ok(parse_line(l).map_err(|e| anyhow!("Parse error: {e}"))?.1))
        .collect::<Result<Vec<_>>>()?;

    let mut count = 0;
    for (i, l1) in lines.iter().enumerate() {
        for l2 in lines.iter().take(i) {
            if l1 == l2 {
                continue;
            }
            if let Some((x, y)) = l1.intersects2d(l2) {
                // println!("Intersects: {:?} {:?} at {}, {}", l1, l2, x, y);
                if x >= min_xy && x <= max_xy && y >= min_xy && y <= max_xy {
                    // println!("Good intersection");
                    // println!("Intersects: {:?} {:?} at {}, {}", l1, l2, x, y);
                    // println!("Intersects at {}, {}", x, y);
                    count += 1;
                } else {
                    // println!("Intersects outside area: {:?} {:?} at {}, {}", l1, l2, x, y);
                }
            }
        }
    }
    Ok(count)
}

// Parse lines like 19, 13, 30 @ -2,  1, -2
fn parse_line(input: &str) -> IResult<&str, Line> {
    let (remaining, (a, _, _, _, b, _, _, _, c, _, _, _, d, _, _, _, e, _, _, _, f)) =
        tuple((
            ni128,
            multispace0,
            tag(","),
            multispace0,
            ni128,
            multispace0,
            tag(","),
            multispace0,
            ni128,
            multispace0,
            tag("@"),
            multispace0,
            ni128,
            multispace0,
            tag(", "),
            multispace0,
            ni128,
            multispace0,
            tag(", "),
            multispace0,
            ni128,
        ))(input)?;

    Ok((
        remaining,
        Line::new(
            (a as f64, b as f64, c as f64),
            (d as f64, e as f64, f as f64),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part2, 24, None, Some(0));
    input_test!(part1, 24, Some(19523), None);

    #[test]
    fn sample_part1() -> Result<()> {
        let input = std::fs::read_to_string(format!("inputs/24/sample.txt"))?;
        let p1 = super::part1(&input, 7.0, 27.0)?;
        assert_eq!(2, p1);
        Ok(())
    }
}
