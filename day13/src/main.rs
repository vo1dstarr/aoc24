use std::fs;

use regex::Regex;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    println!("Answer to part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    find_sum_min_tokens_with(input, |g| g)
}

fn part2(input: &str) -> usize {
    find_sum_min_tokens_with(input, |mut g| {
        let offset = 10000000000000;
        g.prize.0 += offset;
        g.prize.1 += offset;
        g
    })
}

fn find_sum_min_tokens_with(input: &str, f: impl Fn(Game) -> Game) -> usize {
    let regex = Regex::new(r": X\D(\d+), Y\D(\d+)").unwrap();
    input
        .split("\n\n")
        .map(|s| {
            let mut lines = s.lines();
            let a_line = lines.next().unwrap();
            let b_line = lines.next().unwrap();
            let prize_line = lines.next().unwrap();

            let (_, [a_x, a_y]) = regex.captures(a_line).unwrap().extract();
            let (_, [b_x, b_y]) = regex.captures(b_line).unwrap().extract();
            let (_, [prize_x, prize_y]) = regex.captures(prize_line).unwrap().extract();
            f(Game {
                a: (a_x.parse().unwrap(), a_y.parse().unwrap()),
                b: (b_x.parse().unwrap(), b_y.parse().unwrap()),
                prize: (prize_x.parse().unwrap(), prize_y.parse().unwrap()),
            })
        })
        .filter_map(|g| play_algebra(&g))
        .sum()
}

// 8400 = A*94 + B*22
// 5400 = A*34 + B*67
// Cost = 3A * B

// (8400 - B*22)/94 = A
// 5400 = (8400/94 - 22/94)*34 +67B
// 5400 = (8400*34)/94 -34*22B/94 + 67B
// 5400 - (8400*34)/94 = 67B - 34*22B/94
// 5400 - (8400*34)/94 = (67 - 34*22/94)B
// (5400 - (8400*34)/94) / (67 - 34*22/94) = B
fn play_algebra(game: &Game) -> Option<usize> {
    let prize_x = game.prize.0 as f64;
    let prize_y = game.prize.1 as f64;
    let a_x = game.a.0 as f64;
    let a_y = game.a.1 as f64;
    let b_x = game.b.0 as f64;
    let b_y = game.b.1 as f64;
    let b = (prize_y - prize_x * a_y / a_x) / (b_y - a_y * b_x / a_x);
    let a = (prize_x - b * b_x) / a_x;
    if a < 0. || b < 0. {
        None
    // my epsilon was too strict. started with 1e-10. Had to go all the way down to 1e-3
    } else if (a.round() - a).abs() < 1e-3 && (b.round() - b).abs() < 1e-3 {
        Some(3 * a.round() as usize + b.round() as usize)
    } else {
        None
    }
}

struct Game {
    a: (usize, usize),
    b: (usize, usize),
    prize: (usize, usize),
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let answer = part1(input);
        assert_eq!(answer, 480);
    }

    #[test]
    fn test_part1_small1() {
        let game = Game {
            a: (94, 34),
            b: (22, 67),
            prize: (8400, 5400),
        };
        let answer = play_algebra(&game);
        assert_eq!(answer, Some(280));
    }

    #[test]
    fn test_part2() {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        let answer = part2(input);
        assert_eq!(answer, 875318608908);
    }

    #[test]
    fn test_part2_small1() {
        let game = Game {
            a: (94, 34),
            b: (22, 67),
            prize: (10000000008400, 10000000005400),
        };
        let answer = play_algebra(&game);
        assert!(answer.is_none());
    }

    #[test]
    fn test_part2_small2() {
        let game = Game {
            a: (26, 66),
            b: (67, 21),
            prize: (10000000012748, 10000000012176),
        };
        let answer = play_algebra(&game);
        assert_eq!(answer, Some(3 * 118679050709 + 103199174542));
    }

    #[test]
    fn test_part2_small3() {
        let game = Game {
            a: (17, 86),
            b: (84, 37),
            prize: (10000000007870, 10000000006450),
        };
        let answer = play_algebra(&game);
        assert!(answer.is_none());
    }

    #[test]
    fn test_part2_small4() {
        let game = Game {
            a: (69, 23),
            b: (27, 71),
            prize: (10000000018641, 10000000010279),
        };
        let answer = play_algebra(&game);
        assert_eq!(answer, Some(3 * 102851800151 + 107526881786));
    }
}
