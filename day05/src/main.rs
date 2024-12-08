use std::{cell::Cell, collections::HashSet, fs};

use crepe::crepe;
use itertools::Itertools;

crepe! {
    @input
    struct Rule(i32, i32);

    @output
    struct Before(i32, i32);

    Before(x,y) <- Rule(x,y);
    Before(x,z) <- Before(x,y), Before(y,z), !Rule(z,x);
}

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    println!("Answer to part2: {}", part2(&input));
}

fn part1(input: &str) -> i32 {
    let mut split_iter = input.split("\n\n");
    let rules = split_iter.next().unwrap();
    let orders = split_iter.next().unwrap();
    let rules_vec: Vec<_> = rules
        .lines()
        .map(|s| {
            let mut split = s.split('|');
            Rule(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let orders_vec: Vec<Vec<i32>> = orders
        .lines()
        .map(|s| {
            s.split(',')
                .map(|str_num| str_num.parse().unwrap())
                .collect()
        })
        .collect();
    let ordered = run_logic(&rules_vec);
    let result: i32 = orders_vec
        .iter()
        .filter_map(|v| {
            let valid = is_valid_order(v, &ordered);
            if valid {
                //find middle number
                let size = v.len();
                Some(v[size / 2])
            } else {
                None
            }
        })
        .sum();
    result
}

fn part2(input: &str) -> i32 {
    let mut split_iter = input.split("\n\n");
    let rules = split_iter.next().unwrap();
    let orders = split_iter.next().unwrap();
    let rules_vec: Vec<_> = rules
        .lines()
        .map(|s| {
            let mut split = s.split('|');
            Rule(
                split.next().unwrap().parse().unwrap(),
                split.next().unwrap().parse().unwrap(),
            )
        })
        .collect();
    let orders_vec: Vec<Vec<i32>> = orders
        .lines()
        .map(|s| {
            s.split(',')
                .map(|str_num| str_num.parse().unwrap())
                .collect()
        })
        .collect();
    let ordered = run_logic(&rules_vec);
    let result: i32 = orders_vec
        .iter()
        .filter_map(|v| {
            let valid = is_valid_order(v, &ordered);
            if valid {
                None
            } else {
                //fix the order and find the middle page
                let new_order = fix_order(v, &ordered);
                let size = new_order.len();
                Some(new_order[size / 2])
            }
        })
        .sum();
    result
}

fn fix_order(list: &[i32], ordered: &HashSet<Before>) -> Vec<i32> {
    let mut list = Vec::from(list);
    loop {
        let slice_of_cells: &[Cell<i32>] = Cell::from_mut(list.as_mut_slice()).as_slice_of_cells();
        let mut is_in_order = true;
        for (x, y) in slice_of_cells.iter().tuple_windows() {
            if !ordered.contains(&Before(x.get(), y.get())) {
                is_in_order = false;
                Cell::swap(x, y);
                break;
            }
        }
        if is_in_order {
            break;
        }
    }
    list
}

fn run_logic(rules: &[Rule]) -> HashSet<Before> {
    let mut runtime = Crepe::new();
    runtime.extend(rules.iter());
    let (ordered,) = runtime.run();
    ordered
}

fn is_valid_order(list: &[i32], ordered: &HashSet<Before>) -> bool {
    let mut is_in_order = true;
    for (x, y) in list.iter().tuple_windows() {
        if !ordered.contains(&Before(*x, *y)) {
            is_in_order = false;
        }
    }
    is_in_order
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let answer = part1(input);
        assert_eq!(answer, 143);
    }

    fn logic(rules: &[Rule], list: &[i32]) -> bool {
        let ordered = run_logic(rules);
        is_valid_order(list, &ordered)
    }

    #[test]
    fn test_simple() {
        let result = logic(&[Rule(97, 13)], &[97, 13]);
        assert!(result);
    }

    #[test]
    fn test_simple_bad() {
        let result = logic(&[Rule(97, 13)], &[13, 97]);
        assert!(!result);
    }

    #[test]
    fn test_slightly_simple() {
        let result = logic(&[Rule(97, 13), Rule(13, 61)], &[97, 13, 61]);
        assert!(result);
    }

    #[test]
    fn test_slightly_simple_bad() {
        let result = logic(&[Rule(97, 13), Rule(13, 61)], &[97, 61, 13]);
        assert!(!result);
    }

    #[test]
    fn test_part2() {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        let answer = part2(input);
        assert_eq!(answer, 123);
    }
}
