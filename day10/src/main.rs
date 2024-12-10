use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    // println!("Answer to part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    todo!();
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
