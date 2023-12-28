use color_eyre::eyre::anyhow;
use color_eyre::Result;
use nom::bytes::complete::tag;
use nom::character::complete::{i128 as ni128, multispace0};
use nom::sequence::tuple;
use nom::IResult;

#[derive(Debug, PartialEq)]
struct Line {
    at: (f64, f64, f64),
    dir: (i16, i16, i16),
}

impl Line {
    fn new(at: (f64, f64, f64), dir: (i16, i16, i16)) -> Self {
        Self { at, dir }
    }

    fn slope_and_offset2d(&self) -> (f64, f64) {
        let (x1, y1, _z1) = self.at;
        let (x2, y2, _z2) = (
            (self.at.0 + self.dir.0 as f64),
            (self.at.1 + self.dir.1 as f64),
            (self.at.2 + self.dir.2 as f64),
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
            return None;
        }
        if !other.point_in_future(p_x, p_y) {
            return None;
        }

        Some((p_x, p_y))
    }

    // We have some numerical instability to deal with, so we only check approximate intersection
    fn approx_intersects(&self, other: &Line) -> bool {
        if let Some((ix, _iy)) = self.intersects2d(other) {
            let (x1, _y1, z1) = self.at;
            let (dx, _dy, dz) = self.dir;
            let t = (ix - x1) / dx as f64;
            let iz = z1 + t * dz as f64;
            let z2 = other.at.2 + t * other.dir.2 as f64;
            if (iz - z2).abs() > 100.0 {
                println!("z1: {}, z2: {}, iz: {}", z1, z2, iz);
                return false;
            }
            true
        } else {
            println!("No intersection");
            false
        }
    }

    fn point_in_future(&self, x2: f64, y2: f64) -> bool {
        let (x1, y1, _z1) = self.at;
        let (dx, dy, _dz) = self.dir;
        if dx > 0 && x2 < x1 {
            return false;
        }
        if dx < 0 && x2 > x1 {
            return false;
        }
        if dy > 0 && y2 < y1 {
            return false;
        }
        if dy < 0 && y2 > y1 {
            return false;
        }
        true
    }
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

    let line_direction_dot_plane_normal = line.dir.0 as f64 * plane_normal.0
        + line.dir.1 as f64 * plane_normal.1
        + line.dir.2 as f64 * plane_normal.2;

    let t = -dot_product / line_direction_dot_plane_normal;

    (
        line.at.0 + t * line.dir.0 as f64,
        line.at.1 + t * line.dir.1 as f64,
        line.at.2 + t * line.dir.2 as f64,
    )
}

fn cross_product(dir1: (f64, f64, f64), dir2: (f64, f64, f64)) -> (f64, f64, f64) {
    (
        dir1.1 * dir2.2 - dir1.2 * dir2.1,
        dir1.2 * dir2.0 - dir1.0 * dir2.2,
        dir1.0 * dir2.1 - dir1.1 * dir2.0,
    )
}

pub fn run(input: &str) -> Result<(u64, u64)> {
    let p1 = part1(input, 200_000_000_000_000.0, 400_000_000_000_000.0)?;
    // let p2 = part2(input)?;
    let p2 = part2_faster(input)?;
    Ok((p1, p2))
}

fn part2_faster(input: &str) -> Result<u64> {
    let mut lines: Vec<_> = input
        .lines()
        .map(|l| Ok(parse_line(l).map_err(|e| anyhow!("Parse error: {e}"))?.1))
        .collect::<Result<Vec<_>>>()?;

    lines.sort_by(|a, b| a.dir.0.partial_cmp(&b.dir.0).unwrap());

    let pairs_same_dx: Vec<_> = lines
        .windows(2)
        .filter(|lines| lines[0].dir.0 == lines[1].dir.0)
        .map(|lines| (lines[0].at.0, lines[1].at.0, lines[0].dir.0))
        .collect();

    let dx_candidates = (-500..500)
        .filter(|n| {
            *n != 0
                && pairs_same_dx.iter().all(|(a, b, dx)| {
                    (n - dx != 0) && (*a as i64 - *b as i64).abs() % (n - dx).abs() as i64 == 0
                })
        })
        .collect::<Vec<_>>();

    lines.sort_by(|a, b| a.dir.1.partial_cmp(&b.dir.1).unwrap());

    let pairs_same_dy: Vec<_> = lines
        .windows(2)
        .filter(|lines| lines[0].dir.1 == lines[1].dir.1)
        .map(|lines| (lines[0].at.1, lines[1].at.1, lines[0].dir.1))
        .collect();

    let dy_candidates = (-500..500)
        .filter(|n| {
            *n != 0
                && pairs_same_dy.iter().all(|(a, b, dy)| {
                    (n - dy != 0) && (*a as i64 - *b as i64).abs() % (n - dy).abs() as i64 == 0
                })
        })
        .collect::<Vec<_>>();

    lines.sort_by(|a, b| a.dir.2.partial_cmp(&b.dir.2).unwrap());

    let pairs_same_dz: Vec<_> = lines
        .windows(2)
        .filter(|lines| lines[0].dir.2 == lines[1].dir.2)
        .map(|lines| (lines[0].at.2, lines[1].at.2, lines[0].dir.2))
        .collect();

    let dz_candidates = (-500..500)
        .filter(|n| {
            *n != 0
                && pairs_same_dz.iter().all(|(a, b, dz)| {
                    (n - dz != 0) && (*a as i64 - *b as i64).abs() % (n - dz).abs() as i64 == 0
                })
        })
        .collect::<Vec<_>>();

    // For every set of candidates, let's assume it's correct and check if it matches the other hailstones
    // NB this looks like an n^3, but in practice with the real input each candidate set has only one element
    for dx in &dx_candidates {
        for dy in &dy_candidates {
            for dz in &dz_candidates {
                if let Some((x, y, z)) = check_dxyz(&lines, (*dx, *dy, *dz)) {
                    return Ok((x as u64) + (y as u64) + z as u64);
                }
            }
        }
    }
    Ok(0)
}

fn check_dxyz(hailstones: &[Line], (dx, dy, dz): (i16, i16, i16)) -> Option<(f64, f64, f64)> {
    let (hs0_dx, hs0_dy, hs0_dz) = hailstones[0].dir;
    let plane_normal = cross_product(
        (hs0_dx as f64, hs0_dy as f64, hs0_dz as f64),
        (dx as f64, dy as f64, dz as f64),
    );
    let plane_point = hailstones[0].at;
    let intersection = line_plane_intersection(&hailstones[1], plane_point, plane_normal);
    if !is_int(intersection.0) || !is_int(intersection.1) || !is_int(intersection.2) {
        return None;
    }
    let (ix, iy, iz) = intersection;
    let t = ((ix - hailstones[1].at.0) / hailstones[1].dir.0 as f64).abs();
    let candidate = (ix - t * dx as f64, iy - t * dy as f64, iz - t * dz as f64);
    // Now we need to check the candidate against every hailstone
    for hs in hailstones {
        if !hs.approx_intersects(&Line::new(candidate, (dx, dy, dz))) {
            return None;
        }
    }

    Some(candidate)
}

fn is_int(x: f64) -> bool {
    x.fract() == 0.0
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
            (d as i16, e as i16, f as i16),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part2, 24, None, Some(47));
    input_test!(part1, 24, Some(19523), None);

    #[test]
    fn sample_part1() -> Result<()> {
        let input = std::fs::read_to_string(format!("inputs/24/sample.txt"))?;
        let p1 = super::part1(&input, 7.0, 27.0)?;
        assert_eq!(2, p1);
        Ok(())
    }
}
