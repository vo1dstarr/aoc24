use std::fs;

use petgraph::{algo::has_path_connecting, graph::DiGraph, visit::EdgeFiltered};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    // println!("Answer to part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let matrix = parse_to_matrix(input);
    let (graph, trailheads, trailends) = build_graph(matrix);
    let filtered = EdgeFiltered::from_fn(&graph, |e| *(e.weight()) == 1);
    trailheads
        .iter()
        .map(|head_idx| {
            trailends
                .iter()
                .map(|end_idx| {
                    if has_path_connecting(&filtered, *head_idx, *end_idx, None) {
                        1
                    } else {
                        0
                    }
                })
                .sum::<i32>()
        })
        .sum()
}

fn build_graph(
    matrix: Vec<Vec<i32>>,
) -> (
    petgraph::Graph<usize, i32, petgraph::Directed>,
    Vec<petgraph::prelude::NodeIndex>,
    Vec<petgraph::prelude::NodeIndex>,
) {
    let m_len = matrix.len();
    let m_wid = matrix[0].len();
    let mut graph = DiGraph::new();
    let mut trailheads = Vec::new();
    let mut trailends = Vec::new();
    let mut g_ids = Vec::new();
    for (y, row) in matrix.iter().enumerate() {
        let mut g_id_row = Vec::new();
        for (x, val) in row.iter().enumerate() {
            let m_id = y * m_wid + x;
            let graph_id = graph.add_node(m_id);
            if *val == 0 {
                trailheads.push(graph_id);
            } else if *val == 9 {
                trailends.push(graph_id);
            }
            g_id_row.push((*val, graph_id));
        }
        g_ids.push(g_id_row);
    }
    for (y, row) in g_ids.iter().enumerate() {
        for (x, (val, g_id)) in row.iter().enumerate() {
            if x + 1 < m_wid {
                let (next_val, next_id) = g_ids[y][x + 1];
                graph.add_edge(*g_id, next_id, next_val - val);
            }
            if y + 1 < m_len {
                let (next_val, next_id) = g_ids[y + 1][x];
                graph.add_edge(*g_id, next_id, next_val - val);
            }
            if x > 0 {
                let (next_val, next_id) = g_ids[y][x - 1];
                graph.add_edge(*g_id, next_id, next_val - val);
            }
            if y > 0 {
                let (next_val, next_id) = g_ids[y - 1][x];
                graph.add_edge(*g_id, next_id, next_val - val);
            }
        }
    }
    (graph, trailheads, trailends)
}

fn parse_to_matrix(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i32)
                .collect()
        })
        .collect()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let answer = part1(input);
        assert_eq!(answer, 36);
    }
}
