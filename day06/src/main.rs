use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    println!("Answer to part2: {}", part2(&input));
}

type Matrix = Vec<Vec<Space>>;

fn part1(input: &str) -> i32 {
    let (mut matrix, mut guard) = parse(input);
    while let Status::Continue = guard.action(&mut matrix) {}
    matrix
        .iter()
        .map(|row| {
            row.iter()
                .map(|space| if let Space::Visited(_) = space { 1 } else { 0 })
                .sum::<i32>()
        })
        .sum()
}

fn parse(input: &str) -> (Vec<Vec<Space>>, Guard) {
    let matrix: Matrix = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '.' => Space::Empty,
                    '#' => Space::Obstruction,
                    '^' => Space::Visited(HashSet::from([Direction::Up])),
                    _ => panic!(),
                })
                .collect()
        })
        .collect();
    let mut guard = Guard {
        location: (0, 0),
        direction: Direction::Up,
    };
    'outer: for (y, row) in matrix.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if let Space::Visited(_) = space {
                guard.location = (y, x);
                break 'outer;
            }
        }
    }
    (matrix, guard)
}

fn part2(input: &str) -> i32 {
    let mut num_loops = 0;
    let (matrix, guard) = parse(input);
    for (y, row) in matrix.iter().enumerate() {
        for (x, space) in row.iter().enumerate() {
            if let Space::Empty = space {
                let mut new_matrix = matrix.clone();
                new_matrix[y][x] = Space::Obstruction;
                let mut new_guard = guard.clone();
                loop {
                    match new_guard.action(&mut new_matrix) {
                        Status::Continue => (),
                        Status::Looped => {
                            num_loops += 1;
                            break;
                        }
                        Status::Exited => break,
                    }
                }
            }
        }
    }
    num_loops
}

#[derive(Clone)]
struct Guard {
    location: (usize, usize),
    direction: Direction,
}

enum Status {
    Continue,
    Looped,
    Exited,
}

impl Guard {
    fn action(&mut self, matrix: &mut Matrix) -> Status {
        let m_len = matrix.len();
        let m_wid = matrix[0].len();
        loop {
            let (mut next_y, mut next_x) = self.location;
            match self.direction {
                Direction::Up => {
                    if next_y == 0 {
                        return Status::Exited;
                    }
                    next_y -= 1
                }
                Direction::Right => {
                    if next_x >= m_wid - 1 {
                        return Status::Exited;
                    }
                    next_x += 1
                }
                Direction::Down => {
                    if next_y >= m_len - 1 {
                        return Status::Exited;
                    }
                    next_y += 1
                }
                Direction::Left => {
                    if next_x == 0 {
                        return Status::Exited;
                    }
                    next_x -= 1
                }
            }
            if let Space::Obstruction = matrix[next_y][next_x] {
                match self.direction {
                    Direction::Up => self.direction = Direction::Right,
                    Direction::Right => self.direction = Direction::Down,
                    Direction::Down => self.direction = Direction::Left,
                    Direction::Left => self.direction = Direction::Up,
                }
            } else {
                self.location = (next_y, next_x);
                if let Space::Visited(set) = &mut matrix[next_y][next_x] {
                    if set.contains(&self.direction) {
                        return Status::Looped;
                    }
                    set.insert(self.direction);
                } else {
                    matrix[next_y][next_x] = Space::Visited(HashSet::from([self.direction]));
                };
                break;
            }
        }
        Status::Continue
    }
}

#[derive(PartialEq, Eq, Hash, Clone, Copy)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Clone)]
enum Space {
    Empty,
    Obstruction,
    Visited(HashSet<Direction>),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let answer = part1(input);
        assert_eq!(answer, 41);
    }

    #[test]
    fn test_part2() {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        let answer = part2(input);
        assert_eq!(answer, 6);
    }
}
