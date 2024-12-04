use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input.txt").unwrap();
    let result = num_xmas(&input);
    println!("Num of XMAS: {}", result);
    println!("Num of X-Mas: {}", num_mas_cross(&input));
}

fn num_xmas(input: &str) -> i32 {
    let mut count = 0;
    let needle = b"XMAS";
    let matrix: Vec<Vec<u8>> = input.lines().map(|s| s.bytes().collect()).collect();
    let matrix = Matrix::new(matrix);
    for y in 0..matrix.y_len {
        for x in 0..matrix.x_len {
            let letter = matrix.data[y][x];
            if letter != needle[0] {
                continue;
            }
            if search(matrix.right_iter(y, x), needle) {
                count += 1;
            }
            if search(matrix.left_iter(y, x), needle) {
                count += 1;
            }
            if search(matrix.up_iter(y, x), needle) {
                count += 1;
            }
            if search(matrix.down_iter(y, x), needle) {
                count += 1;
            }
            if search(matrix.up_right_iter(y, x), needle) {
                count += 1;
            }
            if search(matrix.up_left_iter(y, x), needle) {
                count += 1;
            }
            if search(matrix.down_right_iter(y, x), needle) {
                count += 1;
            }
            if search(matrix.down_left_iter(y, x), needle) {
                count += 1;
            }
        }
    }
    count
}

fn search<'m>(matrix: impl Iterator<Item = &'m u8>, needle: &[u8]) -> bool {
    let mut matches = 0;
    for (hay, needle) in matrix.zip(needle.iter()) {
        if *hay == *needle {
            matches += 1;
        } else {
            break;
        }
    }
    if matches == needle.len() {
        return true;
    }
    false
}

fn num_mas_cross(input: &str) -> i32 {
    let mut count = 0;
    let needle = b"MAS";
    let matrix: Vec<Vec<u8>> = input.lines().map(|s| s.bytes().collect()).collect();
    let matrix = Matrix::new(matrix);
    for y in 0..matrix.y_len {
        for x in 0..matrix.x_len {
            let letter = matrix.data[y][x];
            if letter != b'A' {
                continue;
            }
            let mut back_slash = false;
            let mut top_left = matrix.up_left_iter(y, x);
            top_left.next();
            if let Some((new_y, new_x)) = top_left.get_state() {
                if search(matrix.down_right_iter(new_y, new_x), needle) {
                    back_slash = true;
                } else {
                    let mut bottom_right = matrix.down_right_iter(y, x);
                    bottom_right.next();
                    if let Some((new_y, new_x)) = bottom_right.get_state() {
                        if search(matrix.up_left_iter(new_y, new_x), needle) {
                            back_slash = true;
                        }
                    }
                }
            }
            if !back_slash {
                continue;
            }
            let mut forward_slash = false;
            let mut top_right = matrix.up_right_iter(y, x);
            top_right.next();
            if let Some((new_y, new_x)) = top_right.get_state() {
                if search(matrix.down_left_iter(new_y, new_x), needle) {
                    forward_slash = true;
                } else {
                    let mut bottom_left = matrix.down_left_iter(y, x);
                    bottom_left.next();
                    if let Some((new_y, new_x)) = bottom_left.get_state() {
                        if search(matrix.up_right_iter(new_y, new_x), needle) {
                            forward_slash = true;
                        }
                    }
                }
            }
            if forward_slash {
                count += 1;
            }
        }
    }
    count
}

struct Matrix {
    data: Vec<Vec<u8>>,
    y_len: usize,
    x_len: usize,
}

struct MatrixRight<'a> {
    data: &'a Matrix,
    y: usize,
    x: usize,
}

struct MatrixLeft<'a> {
    data: &'a Matrix,
    y: usize,
    x: usize,
    flag: bool,
}

struct MatrixUp<'a> {
    data: &'a Matrix,
    y: usize,
    x: usize,
    flag: bool,
}

struct MatrixDown<'a> {
    data: &'a Matrix,
    y: usize,
    x: usize,
}

struct MatrixUpRight<'a> {
    data: &'a Matrix,
    y: usize,
    x: usize,
    flag: bool,
}

struct MatrixUpLeft<'a> {
    data: &'a Matrix,
    y: usize,
    x: usize,
    flag: bool,
}

struct MatrixDownRight<'a> {
    data: &'a Matrix,
    y: usize,
    x: usize,
}

struct MatrixDownLeft<'a> {
    data: &'a Matrix,
    y: usize,
    x: usize,
    flag: bool,
}

impl Matrix {
    fn new(data: Vec<Vec<u8>>) -> Matrix {
        let y_len = data.len();
        let x_len = data[0].len();
        Matrix { data, y_len, x_len }
    }
    fn right_iter(&self, y: usize, x: usize) -> MatrixRight {
        MatrixRight { data: self, y, x }
    }
    fn left_iter(&self, y: usize, x: usize) -> MatrixLeft {
        MatrixLeft {
            data: self,
            y,
            x,
            flag: false,
        }
    }
    fn up_iter(&self, y: usize, x: usize) -> MatrixUp {
        MatrixUp {
            data: self,
            y,
            x,
            flag: false,
        }
    }
    fn down_iter(&self, y: usize, x: usize) -> MatrixDown {
        MatrixDown { data: self, y, x }
    }
    fn up_right_iter(&self, y: usize, x: usize) -> MatrixUpRight {
        MatrixUpRight {
            data: self,
            y,
            x,
            flag: false,
        }
    }
    fn up_left_iter(&self, y: usize, x: usize) -> MatrixUpLeft {
        MatrixUpLeft {
            data: self,
            y,
            x,
            flag: false,
        }
    }
    fn down_right_iter(&self, y: usize, x: usize) -> MatrixDownRight {
        MatrixDownRight { data: self, y, x }
    }
    fn down_left_iter(&self, y: usize, x: usize) -> MatrixDownLeft {
        MatrixDownLeft {
            data: self,
            y,
            x,
            flag: false,
        }
    }
}

// trait StateCheck {
//     fn safe(&self) -> bool;

//     fn get_state(&self) -> Option<(&u8, &u8)>;
// }
impl<'a> MatrixRight<'a> {
    fn safe(&self) -> bool {
        self.x < self.data.x_len
    }
    fn get_state(&self) -> Option<(usize, usize)> {
        if self.safe() {
            Some((self.y, self.x))
        } else {
            None
        }
    }
}
impl<'a> Iterator for MatrixRight<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.safe() {
            Some(&self.data.data[self.y][self.x])
        } else {
            None
        };
        self.x += 1;
        ret
    }
}
impl<'a> MatrixLeft<'a> {
    fn safe(&self) -> bool {
        !self.flag
    }
    fn get_state(&self) -> Option<(usize, usize)> {
        if self.safe() {
            Some((self.y, self.x))
        } else {
            None
        }
    }
}
impl<'a> Iterator for MatrixLeft<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.safe() {
            Some(&self.data.data[self.y][self.x])
        } else {
            None
        };
        if self.x == 0 {
            self.flag = true;
        } else {
            self.x -= 1;
        }
        ret
    }
}
impl<'a> MatrixUp<'a> {
    fn safe(&self) -> bool {
        !self.flag
    }
    fn get_state(&self) -> Option<(usize, usize)> {
        if self.safe() {
            Some((self.y, self.x))
        } else {
            None
        }
    }
}
impl<'a> Iterator for MatrixUp<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.safe() {
            Some(&self.data.data[self.y][self.x])
        } else {
            None
        };
        if self.y == 0 {
            self.flag = true;
        } else {
            self.y -= 1;
        }
        ret
    }
}
impl<'a> MatrixDown<'a> {
    fn safe(&self) -> bool {
        self.y < self.data.y_len
    }
    fn get_state(&self) -> Option<(usize, usize)> {
        if self.safe() {
            Some((self.y, self.x))
        } else {
            None
        }
    }
}
impl<'a> Iterator for MatrixDown<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.safe() {
            Some(&self.data.data[self.y][self.x])
        } else {
            None
        };
        self.y += 1;
        ret
    }
}
impl<'a> MatrixUpRight<'a> {
    fn safe(&self) -> bool {
        !self.flag && self.x < self.data.x_len
    }
    fn get_state(&self) -> Option<(usize, usize)> {
        if self.safe() {
            Some((self.y, self.x))
        } else {
            None
        }
    }
}
impl<'a> Iterator for MatrixUpRight<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.safe() {
            Some(&self.data.data[self.y][self.x])
        } else {
            None
        };
        if self.y == 0 {
            self.flag = true;
        } else {
            self.y -= 1;
        }
        self.x += 1;
        ret
    }
}
impl<'a> MatrixUpLeft<'a> {
    fn safe(&self) -> bool {
        !self.flag
    }
    fn get_state(&self) -> Option<(usize, usize)> {
        if self.safe() {
            Some((self.y, self.x))
        } else {
            None
        }
    }
}
impl<'a> Iterator for MatrixUpLeft<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.safe() {
            Some(&self.data.data[self.y][self.x])
        } else {
            None
        };
        if self.y == 0 || self.x == 0 {
            self.flag = true;
        } else {
            self.y -= 1;
            self.x -= 1;
        }
        ret
    }
}
impl<'a> MatrixDownRight<'a> {
    fn safe(&self) -> bool {
        self.y < self.data.y_len && self.x < self.data.x_len
    }
    fn get_state(&self) -> Option<(usize, usize)> {
        if self.safe() {
            Some((self.y, self.x))
        } else {
            None
        }
    }
}
impl<'a> Iterator for MatrixDownRight<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.safe() {
            Some(&self.data.data[self.y][self.x])
        } else {
            None
        };
        self.y += 1;
        self.x += 1;
        ret
    }
}
impl<'a> MatrixDownLeft<'a> {
    fn safe(&self) -> bool {
        !self.flag && self.y < self.data.y_len
    }
    fn get_state(&self) -> Option<(usize, usize)> {
        if self.safe() {
            Some((self.y, self.x))
        } else {
            None
        }
    }
}
impl<'a> Iterator for MatrixDownLeft<'a> {
    type Item = &'a u8;

    fn next(&mut self) -> Option<Self::Item> {
        let ret = if self.safe() {
            Some(&self.data.data[self.y][self.x])
        } else {
            None
        };
        if self.x == 0 {
            self.flag = true;
        } else {
            self.x -= 1;
        }
        self.y += 1;
        ret
    }
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let answer = num_xmas(input);
        assert_eq!(answer, 18);
    }

    #[test]
    fn test_part2() {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        let answer = num_mas_cross(input);
        assert_eq!(answer, 9);
    }

    #[test]
    fn test_simple1_part2() {
        let input = "MOM
OAO
SOS";
        let answer = num_mas_cross(input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_simple2_part2() {
        let input = "SOM
OAO
SOM";
        let answer = num_mas_cross(input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_simple3_part2() {
        let input = "MOS
OAO
MOS";
        let answer = num_mas_cross(input);
        assert_eq!(answer, 1);
    }

    #[test]
    fn test_simple4_part2() {
        let input = "SOS
OAO
MOM";
        let answer = num_mas_cross(input);
        assert_eq!(answer, 1);
    }
}
