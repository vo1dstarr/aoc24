use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    // println!("Answer to part2: {}", part2(&input));
}

type Matrix<T> = Vec<Vec<T>>;

fn part1(input: &str) -> usize {
    let mut ant_matrix = Vec::new();
    let mut node_matrix = Vec::new();
    for line in input.lines() {
        let mut ant_row = Vec::new();
        let mut node_row = Vec::new();
        for c in line.chars() {
            ant_row.push(match c {
                '.' => AntSpace::Empty,
                _ => AntSpace::Antena(c),
            });
            node_row.push(NodeSpace::Empty);
        }
        ant_matrix.push(ant_row);
        node_matrix.push(node_row);
    }
    // dbg!(&ant_matrix);

    let ant_map = map_ants(&ant_matrix);
    for antenas in ant_map.values() {
        generate_nodes(&mut node_matrix, antenas);
    }

    // dbg!(&node_matrix);
    count_nodes(&node_matrix)
}

fn map_ants(matrix: &Matrix<AntSpace>) -> HashMap<char, Vec<(usize, usize)>> {
    let mut map: HashMap<char, Vec<_>> = HashMap::new();
    for (y, row) in matrix.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if let AntSpace::Antena(freq) = space {
                map.entry(*freq)
                    .and_modify(|v| v.push((y, x)))
                    .or_insert_with(|| vec![(y, x)]);
            }
        }
    }
    map
}

fn generate_nodes(matrix: &mut Matrix<NodeSpace>, antenas: &[(usize, usize)]) {
    if antenas.len() < 2 {
        return;
    }
    for i in 0..antenas.len() {
        for j in i + 1..antenas.len() {
            let ant1 = antenas[i];
            let ant2 = antenas[j];
            let ant1y = ant1.0 as isize;
            let ant1x = ant1.1 as isize;
            let ant2y = ant2.0 as isize;
            let ant2x = ant2.1 as isize;
            let diffy = ant1y - ant2y;
            let diffx = ant1x - ant2x;
            insert_nodes_in_direction(matrix, ant1y, ant1x, diffy, diffx);
            insert_nodes_in_direction(matrix, ant2y, ant2x, -diffy, -diffx);
        }
    }
}

fn insert_nodes_in_direction(
    matrix: &mut Matrix<NodeSpace>,
    starty: isize,
    startx: isize,
    offsety: isize,
    offsetx: isize,
) {
    let m_len = matrix.len();
    let m_wid = matrix[0].len();

    let mut y = starty;
    let mut x = startx;
    // loop {
    y += offsety;
    x += offsetx;
    if y < 0 || x < 0 || y >= m_len as isize || x >= m_wid as isize {
        return;
    }
    // println!("Gen node at {},{}", y, x);
    matrix[y as usize][x as usize] = NodeSpace::Node;
    // }
}

fn count_nodes(matrix: &Matrix<NodeSpace>) -> usize {
    matrix
        .iter()
        .map(|line| {
            line.iter()
                .filter(|space| match space {
                    NodeSpace::Empty => false,
                    NodeSpace::Node => true,
                })
                .count()
        })
        .sum()
}

#[derive(Debug)]
enum AntSpace {
    Empty,
    Antena(char),
}

#[derive(Debug)]
enum NodeSpace {
    Empty,
    Node,
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        let answer = part1(input);
        assert_eq!(answer, 14);
    }
}
