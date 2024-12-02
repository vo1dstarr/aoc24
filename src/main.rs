use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let diff = difference_score(&input);
    println!("difference: {}", diff);

    let sim = similarity_score(&input);
    println!("similarity: {}", sim);
}

fn difference_score(input: &str) -> i32 {
    let mut first = vec![];
    let mut second = vec![];

    let parsed = parse_iter(input);

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

// refactoring this out was a mistake wtf
// clippy says I don't need the lifetimes, but if I remove them the compiler yells at me
#[allow(clippy::needless_lifetimes)]
fn parse_iter<'s>(
    input: &'s str,
) -> impl Iterator<Item = impl Iterator<Item = i32> + use<'s>> + use<'s> {
    let parsed = input
        .lines()
        .map(|line| line.split_whitespace().map(|s| s.parse::<i32>().unwrap()));
    parsed
}

fn similarity_score(input: &str) -> i32 {
    let parsed = parse_iter(input);

    let mut first = vec![];
    let mut second = HashMap::new();

    for mut line in parsed {
        first.push(line.next().unwrap());
        second
            .entry(line.next().unwrap())
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let sum: i32 = first.iter().map(|n| second.get(n).unwrap_or(&0) * n).sum();

    sum
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = difference_score(input);
        assert_eq!(result, 11);
    }

    #[test]
    fn test_part2() {
        let input = "3   4
4   3
2   5
1   3
3   9
3   3";
        let result = similarity_score(input);
        assert_eq!(result, 31);
    }
}
