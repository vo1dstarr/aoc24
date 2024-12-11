use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    println!("Answer to part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut stones: Vec<usize> = input
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();
    for _ in 0..25 {
        blink(&mut stones);
    }
    stones.len()
}

fn part2(input: &str) -> usize {
    let mut memoize = Memoize {
        cache: HashMap::new(),
    };
    input
        .split_whitespace()
        .map(|s| (s.parse().unwrap()))
        .map(|n| memoize.num_stones(0, n))
        .sum()
}

struct Memoize {
    cache: HashMap<(i32, usize), usize>,
}

impl Memoize {
    fn num_stones(&mut self, blinks: i32, num: usize) -> usize {
        if blinks == 75 {
            return 1;
        }
        if let Some(count) = self.cache.get(&(blinks, num)) {
            return *count;
        }
        let count = transform(num)
            .into_iter()
            .map(|new_val| self.num_stones(blinks + 1, new_val))
            .sum();
        self.cache.insert((blinks, num), count);
        count
    }
}

fn blink(stones: &mut Vec<usize>) {
    let new_stones: Vec<_> = stones.iter().map(|n| transform(*n)).collect();
    let mut i = 0;
    for sub in new_stones.into_iter() {
        let len = sub.len();
        stones.splice(i..=i, sub);
        i += len;
    }
}

fn transform(num: usize) -> Vec<usize> {
    if num == 0 {
        return vec![1];
    }

    let mut s = num.to_string();
    if s.len() % 2 == 0 {
        let right = s.split_off(s.len() / 2);
        return vec![s.parse().unwrap(), right.parse().unwrap()];
    }

    vec![num * 2024]
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "125 17";
        let answer = part1(input);
        assert_eq!(answer, 55312);
    }
}
