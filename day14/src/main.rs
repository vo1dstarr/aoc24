use std::{fs, str::FromStr};

use regex::Regex;

use lazy_static::lazy_static;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input, 101, 103, 100));
    println!("Starting part2:");
    part2(&input, 101, 103);
}

fn part1(input: &str, x_size: i32, y_size: i32, seconds: i32) -> i32 {
    let quads = input
        .lines()
        .map(|s| Guard::try_from(s).unwrap())
        .filter_map(|mut guard| {
            guard.patrol(seconds);
            let space_size = XY {
                x: x_size,
                y: y_size,
            };
            guard.teleport(&space_size);
            guard.quad(&space_size)
        });
    quad_score(quads)
}

fn part2(input: &str, x_size: i32, y_size: i32) {
    let space_size = XY {
        x: x_size,
        y: y_size,
    };
    let mut guards: Vec<_> = input.lines().map(|s| Guard::try_from(s).unwrap()).collect();
    let mut min_score = i32::MAX;
    for seconds in 1.. {
        for guard in &mut guards {
            guard.patrol(1);
            guard.teleport(&space_size);
        }
        // if the picture is in/near the center,
        // then we expect a very low quad score
        let quads = guards.iter().filter_map(|guard| guard.quad(&space_size));
        let score = quad_score(quads);
        if score < min_score {
            min_score = score;
            println!("{} seconds", seconds);
            print_guards(&guards, &space_size);
        }
    }
}

fn quad_score<I: Iterator<Item = Quadrant>>(guards: I) -> i32 {
    let quad_count = guards.fold(QuadCount::new(), |mut count, quad| {
        match quad {
            Quadrant::TL => count.tl += 1,
            Quadrant::TR => count.tr += 1,
            Quadrant::BL => count.bl += 1,
            Quadrant::BR => count.br += 1,
        };
        count
    });
    quad_count.tl * quad_count.tr * quad_count.bl * quad_count.br
}

fn print_guards(guards: &[Guard], space_size: &XY) {
    let mut grid = vec![vec![0; space_size.x as usize]; space_size.y as usize];
    for guard in guards {
        grid[guard.position.y as usize][guard.position.x as usize] += 1;
    }
    for line in &grid {
        for space in line {
            if *space == 0 {
                print!(".");
            } else {
                print!("{}", space);
            }
        }
        println!();
    }
}

lazy_static! {
    static ref REGEX: regex::Regex = Regex::new(r"p=(\d+),(\d+) v=(-?\d+),(-?\d+)").unwrap();
}

impl TryFrom<&str> for Guard {
    type Error = <i32 as FromStr>::Err;

    fn try_from(s: &str) -> Result<Self, Self::Error> {
        let (_, [p_x, p_y, v_x, v_y]) = REGEX.captures(s).unwrap().extract();
        Ok(Guard {
            position: XY {
                x: p_x.parse()?,
                y: p_y.parse()?,
            },
            velocity: XY {
                x: v_x.parse()?,
                y: v_y.parse()?,
            },
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct XY {
    x: i32,
    y: i32,
}

#[derive(Debug)]
struct Guard {
    position: XY,
    velocity: XY,
}

#[derive(Debug)]
enum Quadrant {
    TL,
    TR,
    BL,
    BR,
}

#[derive(Debug)]
struct QuadCount {
    tl: i32,
    tr: i32,
    bl: i32,
    br: i32,
}

impl QuadCount {
    fn new() -> QuadCount {
        QuadCount {
            tl: 0,
            tr: 0,
            bl: 0,
            br: 0,
        }
    }
}

impl Guard {
    fn patrol(&mut self, seconds: i32) {
        self.position.x += self.velocity.x * seconds;
        self.position.y += self.velocity.y * seconds;
    }
    fn teleport(&mut self, space_size: &XY) {
        let mut new_x = self.position.x % space_size.x;
        let mut new_y = self.position.y % space_size.y;
        if new_x < 0 {
            new_x += space_size.x;
        }
        if new_y < 0 {
            new_y += space_size.y;
        }
        self.position.x = new_x;
        self.position.y = new_y;
    }
    fn quad(&self, space_size: &XY) -> Option<Quadrant> {
        let half_x = space_size.x / 2;
        let half_y = space_size.y / 2;
        if self.position.x < half_x {
            if self.position.y < half_y {
                Some(Quadrant::TL)
            } else if self.position.y >= space_size.y - half_y {
                Some(Quadrant::BL)
            } else {
                //middle
                None
            }
        } else if self.position.x >= space_size.x - half_x {
            if self.position.y < half_y {
                Some(Quadrant::TR)
            } else if self.position.y >= space_size.y - half_y {
                Some(Quadrant::BR)
            } else {
                //middle
                None
            }
        } else {
            //middle
            None
        }
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        let answer = part1(input, 11, 7, 100);
        assert_eq!(answer, 12);
    }

    #[test]
    fn test_part1_small1() {
        let input = "p=2,4 v=2,-3";
        let mut guard = Guard::try_from(input).unwrap();
        guard.patrol(5);
        guard.teleport(&XY { x: 11, y: 7 });
        assert_eq!(guard.position, XY { x: 1, y: 3 });
    }
}
