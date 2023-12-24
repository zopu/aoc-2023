use color_eyre::eyre::anyhow;
use color_eyre::Result;
use nom::bytes::complete::tag;
use nom::character::complete::{i128 as ni128, multispace0};
use nom::sequence::tuple;
use nom::IResult;

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

    #[allow(dead_code)]
    fn intersects_old(&self, other: &Line) -> Option<(f64, f64)> {
        // Why doesn't this give the same answer??
        let (x1, y1, _z1) = self.at;
        let (x2, y2, _z2) = (
            (self.at.0 + self.dir.0),
            (self.at.1 + self.dir.1),
            (self.at.2 + self.dir.2),
        );
        let (x3, y3, _z3) = other.at;
        let (x4, y4, _z4) = (
            (other.at.0 + other.dir.0),
            (other.at.1 + other.dir.1),
            (other.at.2 + other.dir.2),
        );

        let i_x_n = (x1 * y2 - y1 * x2) * (x3 - x4) - (x1 - x2) * (x3 * y4 - y3 * x4);
        let i_x_d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if i_x_d == 0.0 {
            println!("Hailstones {:?}, {:?} paths are parallel (x)", self, other);
            return None;
        }
        let i_y_n = (x1 * y2 - y1 * x2) * (y3 - y4) - (y1 - y2) * (x3 * y4 - y3 * x4);
        let i_y_d = (x1 - x2) * (y3 - y4) - (y1 - y2) * (x3 - x4);
        if i_y_d == 0.0 {
            println!("Hailstones {:?}, {:?} paths are parallel (y)", self, other);
            return None;
        }
        let i_x = i_x_n / i_x_d;
        let i_y = i_y_n / i_y_d;
        // println!("Intersection at {}, {}", i_x, i_y);

        // Check that i_x, i_y is in the future
        if !self.point_in_future(i_x, i_y) {
            // println!("Intersection in past between {:?} and {:?}", self, other);
            return None;
        }
        if !other.point_in_future(i_x, i_y) {
            // println!("Intersection in past between {:?} and {:?}", self, other);
            return None;
        }

        Some((i_x, i_y))
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
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let p1 = part1(input, 200_000_000_000_000.0, 400_000_000_000_000.0)?;

    // 19521 too low
    Ok((p1, 0))
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
    use crate::runner::test::input_test;

    input_test!(part1, 24, Some(19523), None);

    #[test]
    fn sample_part1() -> Result<()> {
        let input = std::fs::read_to_string(format!("inputs/24/sample.txt"))?;
        let p1 = super::part1(&input, 7.0, 27.0)?;
        assert_eq!(2, p1);
        Ok(())
    }
}
