use rustworkx_core::{connectivity::stoer_wagner_min_cut, petgraph::prelude::UnGraph};
use std::collections::HashMap;

fn main() {
  let input = include_str!("./input1.txt");
  let output1 = part1(input);

  dbg!(output1);
}

fn part1(input: &str) -> usize {
  // Use rustworkx to build a graph and run Stoer-Wagner.
  let mut graph: UnGraph<&str, ()> = rustworkx_core::petgraph::Graph::new_undirected();
  let mut nodes = HashMap::new();
  input.lines().for_each(|line| {
    let parts: Vec<_> = line.split(":").collect();
    let node = parts[0];
    let node = *nodes.entry(node).or_insert_with(|| graph.add_node(node));
    let edges = parts[1].trim().split(" ");
    for edge in edges {
      let edge = *nodes.entry(edge).or_insert_with(|| graph.add_node(edge));
      graph.add_edge(node, edge, ());
    }
  });

  println!("rustworkx:");
  let now = std::time::Instant::now();
  match stoer_wagner_min_cut(&graph, |_| Ok::<i32, ()>(1)) {
    Err(_) => unreachable!(),
    Ok(None) => println!("no solution found"),
    Ok(Some((cut, partition))) => {
      println!("rustworkx-minmum-cut: {}", cut);
      let p1 = partition.len() * (nodes.len() - partition.len());
      println!("p1-rustworkx: {} ({:?})", p1, now.elapsed());
      return p1;
    }
  }

  0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn it_works() {
    let input = r#"jqt: rhn xhk nvd
rsh: frs pzl lsr
xhk: hfx
cmg: qnr nvd lhk bvb
rhn: xhk bvb hfx
bvb: xhk hfx
pzl: lsr hfx nvd
qnr: nvd
ntq: jqt hfx bvb xhk
nvd: lhk
lsr: lhk
rzs: qnr cmg lsr rsh
frs: qnr lhk lsr"#;
    let score = part1(input);
    print!("{}", score);
    assert_eq!(score, 54);
  }
}
