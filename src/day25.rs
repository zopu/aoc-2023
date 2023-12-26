use color_eyre::{eyre::anyhow, Result};
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, multispace1},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};
use petgraph::graph::UnGraph;
use rustworkx_core::connectivity::stoer_wagner_min_cut;

use crate::symbol_table::SymbolTable;

type NodeId = u16;

pub fn run(input: &str) -> Result<(u64, u64)> {
    let mut st = SymbolTable::new();
    let parsed_lines: Vec<(NodeId, Vec<NodeId>)> = input
        .lines()
        .map(|l| {
            let (_, (node, conns)) =
                parse_line(l, &mut st).map_err(|e| anyhow!("Parse error: {e}"))?;
            Ok((node, conns))
        })
        .collect::<Result<Vec<_>>>()?;

    let mut graph: UnGraph<u16, ()> = UnGraph::new_undirected();

    let node_indexes = (0..st.len())
        .map(|i| graph.add_node(i as u16))
        .collect::<Vec<_>>();
    for line in parsed_lines {
        for conn in line.1 {
            graph.add_edge(
                node_indexes[line.0 as usize],
                node_indexes[conn as usize],
                (),
            );
        }
    }

    let min_cut_res: Result<Option<(usize, Vec<_>)>> = stoer_wagner_min_cut(&graph, |_| Ok(1));
    let min_cut_res = min_cut_res?.unwrap();

    let partition_1 = min_cut_res.1.len() as u64;
    let p1 = partition_1 * (st.len() as u64 - partition_1);
    Ok((p1, 0))
}

// Parse lines like "tsx: vrm vsc bjj dbq cth vbm gmb cjd"
fn parse_line<'a>(line: &'a str, st: &mut SymbolTable) -> IResult<&'a str, (NodeId, Vec<NodeId>)> {
    let (remaining, (n, _, conns)) =
        tuple((alpha1, tag(": "), separated_list0(multispace1, alpha1)))(line)?;
    let nid = st.get(n);
    let conns = conns.iter().map(|n| st.get(n)).collect::<Vec<NodeId>>();
    Ok((remaining, (nid, conns)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::runner::test::{input_test, sample_test};

    sample_test!(sample_part1, 25, Some(54), None);
    input_test!(part1, 25, Some(506202), None);
}
