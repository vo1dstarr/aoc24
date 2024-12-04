use regex::*;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let answer = sum_of_mults(&input);
    println!("Sum of mults: {}", answer);

    let part2 = dos_and_donts(&input);
    println!("Part 2: {}", part2);
}

fn sum_of_mults(input: &str) -> i32 {
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    reg.captures_iter(input)
        .map(|cap| {
            let (_, [l, r]) = cap.extract();
            let l: i32 = l.parse().unwrap();
            let r: i32 = r.parse().unwrap();
            l * r
        })
        .sum()
}

fn dos_and_donts(input: &str) -> i32 {
    let reg = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)|do\(\)()()|don't\(\)()()").unwrap();
    let (_, result) = reg
        .captures_iter(input)
        .map(|cap| {
            let (group, [l, r]) = cap.extract();
            match group {
                "do()" => Operator::Do,
                "don't()" => Operator::Dont,
                _ => Operator::Mult(l.parse().unwrap(), r.parse().unwrap()),
            }
        })
        .fold((true, 0), |(on_off, acc), op| match op {
            Operator::Do => (true, acc),
            Operator::Dont => (false, acc),
            Operator::Mult(l, r) => {
                if on_off {
                    (on_off, acc + (l * r))
                } else {
                    (on_off, acc)
                }
            }
        });
    result
}

enum Operator {
    Do,
    Dont,
    Mult(i32, i32),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = r"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let answer = sum_of_mults(input);
        assert_eq!(answer, 161);
    }

    #[test]
    fn test_no_whitespace() {
        let input = "mul ( 2 , 4 )";
        let answer = sum_of_mults(input);
        assert_eq!(answer, 0);

        let input = "mul( 2 , 4 )";
        let answer = sum_of_mults(input);
        assert_eq!(answer, 0);

        let input = "mul(2, 4)";
        let answer = sum_of_mults(input);
        assert_eq!(answer, 0);
    }

    #[test]
    fn test_part2() {
        let input = r"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let result = dos_and_donts(input);
        assert_eq!(result, 48);
    }
}
