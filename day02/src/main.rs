use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();

    let num_safe = safe_reports(&input);
    println!("Number of safe reports: {}", num_safe);

    let num_damp_safe = damp_safe_reports(&input);
    println!(
        "Number of safe reports with Problem Dampener: {}",
        num_damp_safe
    );
}

fn safe_reports(input: &str) -> i32 {
    safe_reports_with(input, is_safe)
}

fn safe_reports_with(input: &str, f: impl Fn(&[i32]) -> bool) -> i32 {
    input
        .lines()
        .map(|s| {
            s.split_whitespace()
                .map(|s| s.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .map(|v| f(&v))
        .fold(0, |acc, report| {
            let mut result = acc;
            if report {
                result += 1;
            }
            result
        })
}

fn is_safe(levels: &[i32]) -> bool {
    levels
        .windows(2)
        .map(|window| window[0] - window[1])
        .map(|diff| {
            if diff > 3 || diff == 0 || diff < -3 {
                Err(diff)
            } else {
                Ok(diff)
            }
        })
        .try_fold(0, |acc, level| {
            if acc == 0 {
                Ok(level?)
            } else {
                match (acc > 0, level? > 0) {
                    (false, false) => Ok(acc),
                    (true, true) => Ok(acc),
                    _ => Err(0),
                }
            }
        })
        .is_ok()
}

fn damp_safe_reports(input: &str) -> i32 {
    safe_reports_with(input, is_damp_safe)
}

fn is_damp_safe(levels: &[i32]) -> bool {
    if is_safe(levels) {
        return true;
    }
    for i in 0..levels.len() {
        let mut damp_report = Vec::with_capacity(levels.len() - 1);
        let split = levels.split_at(i);
        damp_report.extend_from_slice(split.0);
        let final_slice = split.1.get(1..);
        if let Some(slice) = final_slice {
            damp_report.extend_from_slice(slice);
        }
        if is_safe(&damp_report) {
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
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = safe_reports(input);
        assert_eq!(result, 2);
    }

    #[test]
    fn test_is_safe1() {
        let input = [7, 6, 4, 2, 1];
        let result = is_safe(&input);
        assert!(result);
    }

    #[test]
    fn test_is_safe2() {
        let input = [1, 2, 7, 8, 9];
        let result = is_safe(&input);
        assert!(!result);
    }

    #[test]
    fn test_is_safe3() {
        let input = [9, 7, 6, 2, 1];
        let result = is_safe(&input);
        assert!(!result);
    }

    #[test]
    fn test_is_safe4() {
        let input = [1, 3, 2, 4, 5];
        let result = is_safe(&input);
        assert!(!result);
    }

    #[test]
    fn test_is_safe5() {
        let input = [8, 6, 4, 4, 1];
        let result = is_safe(&input);
        assert!(!result);
    }

    #[test]
    fn test_is_safe6() {
        let input = [1, 3, 6, 7, 9];
        let result = is_safe(&input);
        assert!(result);
    }

    #[test]
    fn test_part2() {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        let result = damp_safe_reports(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn test_is_damp_safe1() {
        let input = [7, 6, 4, 2, 1];
        let result = is_damp_safe(&input);
        assert!(result);
    }

    #[test]
    fn test_is_damp_safe2() {
        let input = [1, 2, 7, 8, 9];
        let result = is_damp_safe(&input);
        assert!(!result);
    }

    #[test]
    fn test_is_damp_safe3() {
        let input = [9, 7, 6, 2, 1];
        let result = is_damp_safe(&input);
        assert!(!result);
    }

    #[test]
    fn test_is_damp_safe4() {
        let input = [1, 3, 2, 4, 5];
        let result = is_damp_safe(&input);
        assert!(result);
    }

    #[test]
    fn test_is_damp_safe5() {
        let input = [8, 6, 4, 4, 1];
        let result = is_damp_safe(&input);
        assert!(result);
    }

    #[test]
    fn test_is_damp_safe6() {
        let input = [1, 3, 6, 7, 9];
        let result = is_damp_safe(&input);
        assert!(result);
    }
}
