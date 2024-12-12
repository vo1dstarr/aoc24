use std::{
    collections::{HashSet, VecDeque},
    fs,
};

use petgraph::{
    unionfind::UnionFind,
    visit::{Bfs, EdgeRef, NodeIndexable},
    Graph,
};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    println!("Answer to part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut graph = Graph::new_undirected();
    let garden: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();
    let g_len = garden.len();
    let g_wid = garden[0].len();

    let graph_index_table: Vec<Vec<_>> = garden
        .iter()
        .enumerate()
        .map(|(y, line)| {
            line.iter()
                .enumerate()
                .map(|(x, _)| graph.add_node((y, x)))
                .collect()
        })
        .collect();
    for (y, line) in garden.iter().enumerate() {
        for (x, plot) in line.iter().enumerate() {
            if x > 0 && *plot == garden[y][x - 1] {
                graph.update_edge(graph_index_table[y][x], graph_index_table[y][x - 1], ());
            }
            if x < g_wid - 1 && *plot == garden[y][x + 1] {
                graph.update_edge(graph_index_table[y][x], graph_index_table[y][x + 1], ());
            }
            if y > 0 && *plot == garden[y - 1][x] {
                graph.update_edge(graph_index_table[y][x], graph_index_table[y - 1][x], ());
            }
            if y < g_len - 1 && *plot == garden[y + 1][x] {
                graph.update_edge(graph_index_table[y][x], graph_index_table[y + 1][x], ());
            }
        }
    }
    //find each region
    let mut vertex_sets = UnionFind::new(graph.node_bound());
    for edge in graph.edge_references() {
        let (a, b) = (edge.source(), edge.target());
        // union the two vertices of the edge
        vertex_sets.union(graph.to_index(a), graph.to_index(b));
    }
    let mut roots = vertex_sets.into_labeling();
    roots.sort_unstable();
    roots.dedup();
    //for each region, walk the graph
    //for each node, each edge is a lack of a fence
    roots
        .iter()
        .map(|idx| {
            let idx = graph.from_index(*idx);
            let mut area = 0;
            let mut perim = 0;
            let mut bfs = Bfs::new(&graph, idx);
            while let Some(nx) = bfs.next(&graph) {
                area += 1;
                perim += 4 - graph.neighbors(nx).count();
            }
            area * perim
        })
        .sum()
}

//can't think of how to reuse the graph method from part1, so start over
fn part2(input: &str) -> usize {
    let garden = gen_garden(input);
    let g_len = garden.len();
    let g_wid = garden[0].len();

    (0..=find_max_region(&garden))
        .map(|i| {
            let mut area = 0;
            let mut walls = 0;
            //compute area and horizontal walls
            for line in garden.iter() {
                let mut found_top_wall = false;
                let mut found_bottom_wall = false;
                for plot in line.iter() {
                    if plot.region != i {
                        if found_top_wall {
                            walls += 1;
                            found_top_wall = false;
                        }
                        if found_bottom_wall {
                            walls += 1;
                            found_bottom_wall = false;
                        }
                        continue;
                    }
                    area += 1;
                    if plot.top_wall {
                        found_top_wall = true;
                    } else if found_top_wall {
                        walls += 1;
                        found_top_wall = false;
                    }
                    if plot.down_wall {
                        found_bottom_wall = true;
                    } else if found_bottom_wall {
                        walls += 1;
                        found_bottom_wall = false;
                    }
                }
                if found_top_wall {
                    walls += 1;
                }
                if found_bottom_wall {
                    walls += 1;
                }
            }
            //compute vertical walls
            for x in 0..g_wid {
                let mut found_right_wall = false;
                let mut found_left_wall = false;
                #[allow(clippy::needless_range_loop)]
                for y in 0..g_len {
                    let plot = &garden[y][x];
                    if plot.region != i {
                        if found_right_wall {
                            walls += 1;
                            found_right_wall = false;
                        }
                        if found_left_wall {
                            walls += 1;
                            found_left_wall = false;
                        }
                        continue;
                    }
                    if plot.right_wall {
                        found_right_wall = true;
                    } else if found_right_wall {
                        walls += 1;
                        found_right_wall = false;
                    }
                    if plot.left_wall {
                        found_left_wall = true;
                    } else if found_left_wall {
                        walls += 1;
                        found_left_wall = false;
                    }
                }
                if found_right_wall {
                    walls += 1;
                }
                if found_left_wall {
                    walls += 1;
                }
            }
            area * walls
        })
        .sum()
}

fn find_max_region(garden: &[Vec<Crop>]) -> usize {
    let mut result = 0;
    for line in garden.iter() {
        for plot in line.iter() {
            result = result.max(plot.region);
        }
    }
    result
}

fn gen_garden(input: &str) -> Vec<Vec<Crop>> {
    let mut garden: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(Crop::new).collect())
        .collect();
    let g_len = garden.len();
    let g_wid = garden[0].len();
    let mut region_idx = 0;
    let mut row_hint = 0;
    let mut visited = HashSet::new();
    #[allow(clippy::never_loop)]
    while let Some((y, x)) = find_first_unvisited(&garden, &visited, row_hint) {
        row_hint = y;
        let mut queue = VecDeque::new();
        visited.insert((y, x));
        queue.push_front((y, x));
        while let Some((y, x)) = queue.pop_back() {
            garden[y][x].region = region_idx;
            if x > 0 && garden[y][x].crop == garden[y][x - 1].crop {
                if visited.insert((y, x - 1)) {
                    queue.push_front((y, x - 1));
                }
            } else {
                garden[y][x].left_wall = true;
            }
            if x < g_wid - 1 && garden[y][x].crop == garden[y][x + 1].crop {
                if visited.insert((y, x + 1)) {
                    queue.push_front((y, x + 1));
                }
            } else {
                garden[y][x].right_wall = true;
            }
            if y > 0 && garden[y][x].crop == garden[y - 1][x].crop {
                if visited.insert((y - 1, x)) {
                    queue.push_front((y - 1, x));
                }
            } else {
                garden[y][x].top_wall = true;
            }
            if y < g_len - 1 && garden[y][x].crop == garden[y + 1][x].crop {
                if visited.insert((y + 1, x)) {
                    queue.push_front((y + 1, x));
                }
            } else {
                garden[y][x].down_wall = true;
            }
        }
        region_idx += 1;
    }
    garden
}

fn find_first_unvisited(
    garden: &[Vec<Crop>],
    visited: &HashSet<(usize, usize)>,
    row_hint: usize,
) -> Option<(usize, usize)> {
    for (y, line) in garden.iter().enumerate().skip(row_hint) {
        for (x, _) in line.iter().enumerate() {
            if !visited.contains(&(y, x)) {
                return Some((y, x));
            }
        }
    }
    None
}

#[derive(Debug)]
struct Crop {
    crop: char,
    top_wall: bool,
    right_wall: bool,
    down_wall: bool,
    left_wall: bool,
    region: usize,
}

impl Crop {
    fn new(crop: char) -> Crop {
        Crop {
            crop,
            top_wall: false,
            right_wall: false,
            down_wall: false,
            left_wall: false,
            region: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        let answer = part1(input);
        assert_eq!(answer, 1930);
    }

    #[test]
    fn test_part1_small1() {
        let input = "AAAA
BBCD
BBCC
EEEC
";
        let answer = part1(input);
        assert_eq!(answer, 140);
    }

    #[test]
    fn test_part1_small2() {
        let input = "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO
";
        let answer = part1(input);
        assert_eq!(answer, 772);
    }

    #[test]
    fn test_part2_small1() {
        let input = "AAAA
BBCD
BBCC
EEEC
";
        let answer = part2(input);
        assert_eq!(answer, 80);
    }

    #[test]
    fn test_part2_small2() {
        let input = "EEEEE
EXXXX
EEEEE
EXXXX
EEEEE
";
        let answer = part2(input);
        assert_eq!(answer, 236);
    }

    #[test]
    fn test_part2_small3() {
        let input = "AAAAAA
AAABBA
AAABBA
ABBAAA
ABBAAA
AAAAAA
";
        let answer = part2(input);
        assert_eq!(answer, 368);
    }

    #[test]
    fn test_part2() {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE
";
        let answer = part2(input);
        assert_eq!(answer, 1206);
    }
}
