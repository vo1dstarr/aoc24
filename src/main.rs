use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let sum = run(&input);
    println!("sum: {}", sum);
}

fn run(input: &str) -> i32 {
    let mut first = vec![];
    let mut second = vec![];

    let parsed = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i32>().unwrap()));

    for mut line in parsed {
        first.push(line.next().unwrap());
        second.push(line.next().unwrap());
    }

    first.sort_unstable();
    second.sort_unstable();

    let sum: i32 = first
        .iter()
        .zip(second.iter())
        .map(|(f, s)| (f - s).abs())
        .sum();

    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_add() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = run(input);
        assert_eq!(result, 11);
    }
}
