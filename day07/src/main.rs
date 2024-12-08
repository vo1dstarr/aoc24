use std::{collections::VecDeque, fs};

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let mut split_iter = line.split(": ");
            let test: i64 = split_iter.next().unwrap().parse().unwrap();
            let nums: Vec<i64> = split_iter
                .next()
                .unwrap()
                .split(' ')
                .map(|s| s.parse().unwrap())
                .collect();
            (test, nums)
        })
        .flat_map(|(test, nums)| {
            if valid_ops_exist(test, &nums) {
                Some(test)
            } else {
                None
            }
        })
        .sum()
}

fn valid_ops_exist(test: i64, nums: &[i64]) -> bool {
    let mut queue = VecDeque::new();
    queue.push_back(nums[0]);
    for rhs in &nums[1..] {
        let mut new_queue = VecDeque::new();
        while let Some(lhs) = queue.pop_front() {
            let new_nums = [lhs + rhs, lhs * rhs];
            for num in new_nums {
                // if num is greater than test, then we can prune this branch from the search tree
                if num <= test {
                    new_queue.push_back(num);
                }
            }
        }
        queue = new_queue;
    }
    for n in queue {
        if n == test {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";
        let answer = part1(input);
        assert_eq!(answer, 3749);
    }
}
