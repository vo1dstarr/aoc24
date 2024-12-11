use std::fs;

use petgraph::{
    algo::{all_simple_paths, has_path_connecting},
    graph::DiGraph,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    println!("Answer to part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let matrix = parse_to_matrix(input);
    let (graph, trailheads, trailends) = build_graph(matrix);
    count_scores(
        &trailheads,
        &trailends,
        &graph,
        |graph, head_idx, end_idx| {
            if has_path_connecting(&graph, *head_idx, *end_idx, None) {
                1
            } else {
                0
            }
        },
    )
}

fn part2(input: &str) -> usize {
    let matrix = parse_to_matrix(input);
    let (graph, trailheads, trailends) = build_graph(matrix);
    count_scores(
        &trailheads,
        &trailends,
        &graph,
        |graph, head_idx, end_idx| {
            all_simple_paths::<Vec<_>, _>(graph, *head_idx, *end_idx, 0, None).count()
        },
    )
}

fn count_scores(
    trailheads: &[petgraph::prelude::NodeIndex],
    trailends: &[petgraph::prelude::NodeIndex],
    graph: &petgraph::Graph<usize, u32>,
    score_fn: impl Fn(
        &petgraph::Graph<usize, u32>,
        &petgraph::prelude::NodeIndex,
        &petgraph::prelude::NodeIndex,
    ) -> usize,
) -> usize {
    trailheads
        .iter()
        .map(|head_idx| {
            trailends
                .iter()
                .map(|end_idx| score_fn(graph, head_idx, end_idx))
                .sum::<usize>()
        })
        .sum()
}

fn build_graph(
    matrix: Vec<Vec<i32>>,
) -> (
    petgraph::Graph<usize, u32, petgraph::Directed>,
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
                let weight = next_val - val;
                if weight == 1 {
                    graph.add_edge(*g_id, next_id, weight as u32);
                }
            }
            if y + 1 < m_len {
                let (next_val, next_id) = g_ids[y + 1][x];
                let weight = next_val - val;
                if weight == 1 {
                    graph.add_edge(*g_id, next_id, weight as u32);
                }
            }
            if x > 0 {
                let (next_val, next_id) = g_ids[y][x - 1];
                let weight = next_val - val;
                if weight == 1 {
                    graph.add_edge(*g_id, next_id, weight as u32);
                }
            }
            if y > 0 {
                let (next_val, next_id) = g_ids[y - 1][x];
                let weight = next_val - val;
                if weight == 1 {
                    graph.add_edge(*g_id, next_id, weight as u32);
                }
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

    #[test]
    fn test_part2() {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        let answer = part2(input);
        assert_eq!(answer, 81);
    }
}
