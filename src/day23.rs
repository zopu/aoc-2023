use std::collections::BTreeMap;

use color_eyre::Result;

use crate::grid::Grid;

type Pos = (usize, usize);

type NodeId = u8;
type Distance = u16;

pub fn run(input: &str) -> Result<(u64, u64)> {
    let grid = Grid::<u8>::parse(input, |c| c as u8);
    let p1 = solve(&grid, true);
    let p2 = solve(&grid, false);
    Ok((p1, p2))
}

fn solve(grid: &Grid<u8>, consider_slopes: bool) -> u64 {
    let (dim_x, dim_y) = grid.dimensions;

    // Convert to a graph
    // Find all the nodes

    let nodes = find_nodes(grid);
    let node_ids: BTreeMap<Pos, NodeId> = nodes
        .iter()
        .enumerate()
        .map(|(i, n)| (*n, i as NodeId))
        .collect();
    let mut edges: BTreeMap<NodeId, Vec<(NodeId, Distance)>> = BTreeMap::new();
    for (i, _n) in nodes.iter().enumerate() {
        edges.insert(i as NodeId, vec![]);
    }

    // For every node
    // .  for every neighbour
    // .    find the next node and the distance
    for (i, n) in nodes.iter().enumerate() {
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (n.0 as i32 + dx, n.1 as i32 + dy);
            if nx < 0 || ny < 0 || nx >= dim_x as i32 || ny >= dim_y as i32 {
                continue;
            }
            if *grid.at(nx as usize, ny as usize) == b'#' {
                continue;
            }
            let slope_ok = if !consider_slopes {
                true
            } else {
                match (dx, dy, grid.at(nx as usize, ny as usize)) {
                    (0, 1, b'v') => true,
                    (1, 0, b'>') => true,
                    (0, -1, b'^') => true,
                    (-1, 0, b'<') => true,
                    (_, _, b'>') | (_, _, b'<') | (_, _, b'^') | (_, _, b'v') => false,
                    _ => true,
                }
            };
            if !slope_ok {
                continue;
            }
            // Walk the node and find next node and distance or dead end
            // NB we're guaranteed only to ever find one next node
            if let Some((next_node, distance)) = walk_path(
                grid,
                &node_ids,
                (nx as usize, ny as usize),
                (n.0, n.1),
                consider_slopes,
            ) {
                edges
                    .entry(i as NodeId)
                    .or_default()
                    .push((next_node, distance + 1));
            }
        }
    }

    let mut nodes_grid = grid.clone();
    for (i, (x, y)) in nodes.iter().enumerate() {
        *nodes_grid.at_mut(*x, *y) = i as u8 + b'0';
    }

    let max_edges: Vec<_> = edges
        .values()
        .map(|v| v.iter().map(|(_, d)| d).max().unwrap_or(&0))
        .cloned()
        .collect();

    let max_possible = max_edges.iter().sum();

    let edge_vec: Vec<Vec<(NodeId, Distance)>> = edges.into_values().collect();

    find_max_distance_to_end(
        &edge_vec,
        &max_edges,
        0,
        1,
        &mut vec![false; nodes.len()],
        max_possible,
        0,
        0,
    )
    .unwrap() as u64
}

#[allow(clippy::too_many_arguments)]
fn find_max_distance_to_end(
    edges: &[Vec<(NodeId, Distance)>],
    max_edges: &[Distance],
    node: NodeId,
    end_node: NodeId,
    visited: &mut Vec<bool>,
    max_possible: Distance,
    current: Distance,
    mut current_max: Distance,
) -> Option<Distance> {
    if current + max_possible <= current_max {
        return None;
    }

    visited[node as usize] = true;
    if node == end_node {
        visited[node as usize] = false;
        return Some(0);
    }
    let mut max_distance = 0;
    for (next_node, distance) in &edges[node as usize] {
        if visited[*next_node as usize] {
            continue;
        }
        if let Some(d) = find_max_distance_to_end(
            edges,
            max_edges,
            *next_node,
            end_node,
            visited,
            max_possible - max_edges[node as usize],
            current + distance,
            current_max,
        ) {
            max_distance = max_distance.max(d + distance);
            current_max = current_max.max(max_distance);
        }
    }
    visited[node as usize] = false;
    if max_distance > 0 {
        Some(max_distance)
    } else {
        None
    }
}

fn walk_path(
    grid: &Grid<u8>,
    node_ids: &BTreeMap<Pos, NodeId>,
    pos: Pos,
    prev: Pos,
    consider_slopes: bool,
) -> Option<(NodeId, Distance)> {
    let (dim_x, dim_y) = grid.dimensions;
    for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
        let (nx, ny) = (pos.0 as i32 + dx, pos.1 as i32 + dy);
        if nx < 0 || ny < 0 || nx >= dim_x as i32 || ny >= dim_y as i32 {
            continue;
        }
        if *grid.at(nx as usize, ny as usize) == b'#' {
            continue;
        }
        if (nx as usize, ny as usize) == prev {
            continue;
        }
        if node_ids.contains_key(&(nx as usize, ny as usize)) {
            return Some((*node_ids.get(&(nx as usize, ny as usize)).unwrap(), 1));
        }
        if consider_slopes {
            // return none if we've hit a slope facing the wrong way
            let slope_ok = match (dx, dy, grid.at(nx as usize, ny as usize)) {
                (0, 1, b'v') => true,
                (1, 0, b'>') => true,
                (0, -1, b'^') => true,
                (-1, 0, b'<') => true,
                (_, _, b'>') | (_, _, b'<') | (_, _, b'^') | (_, _, b'v') => false,
                _ => true,
            };
            if !slope_ok {
                return None;
            }
        }
        return walk_path(
            grid,
            node_ids,
            (nx as usize, ny as usize),
            pos,
            consider_slopes,
        )
        .map(|(n, d)| (n, d + 1));
    }
    None
}

fn find_nodes(grid: &Grid<u8>) -> Vec<Pos> {
    let (dim_x, dim_y) = grid.dimensions;
    let mut nodes = vec![];

    let mut start_x = 0;
    for x in 0..dim_x {
        if *grid.at(x, 0) == b'.' {
            start_x = x;
            break;
        }
    }
    nodes.push((start_x, 0));

    let mut end_x = 0;
    for x in 0..dim_x {
        if *grid.at(x, dim_y - 1) == b'.' {
            end_x = x;
            break;
        }
    }
    nodes.push((end_x, dim_y - 1));

    grid.iter_pts(|x, y, c| {
        if *c != b'.' {
            return;
        }
        let mut neighbour_count = 0;
        for (dx, dy) in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let (nx, ny) = (x as i32 + dx, y as i32 + dy);
            if nx < 0 || ny < 0 || nx >= dim_x as i32 || ny >= dim_y as i32 {
                continue;
            }
            if *grid.at(nx as usize, ny as usize) == b'#' {
                continue;
            }
            neighbour_count += 1;
        }
        if neighbour_count > 2 {
            nodes.push((x, y));
        }
    });
    nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 23, Some(94), None);
    sample_test!(sample_part2, 23, None, Some(154));
    input_test!(part1, 23, Some(2186), None);
    input_test!(part2, 23, None, Some(6802));
}
