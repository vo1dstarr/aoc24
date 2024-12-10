use std::fs;

fn main() {
    let input = fs::read_to_string("input.txt").unwrap();
    println!("Answer to part1: {}", part1(&input));
    println!("Answer to part2: {}", part2(&input));
}

fn part1(input: &str) -> usize {
    let mut disk = parse_disk(input);
    defrag(&mut disk);
    checksum(&disk)
}

fn part2(input: &str) -> usize {
    let mut disk = parse_disk(input);
    whole_file_defrag(&mut disk);
    checksum(&disk)
}

fn parse_disk(input: &str) -> Vec<Option<usize>> {
    let mut disk = Vec::new();
    for (i, c) in input.char_indices() {
        let blocks = c.to_digit(10).unwrap();
        let val = if i % 2 == 0 {
            let id = i / 2;
            Some(id)
        } else {
            None
        };

        for _ in 0..blocks {
            disk.push(val);
        }
    }
    disk
}

fn defrag(disk: &mut [Option<usize>]) {
    let mut start = 0;
    let mut end = disk.len() - 1;
    while start < end {
        if disk[end].is_some() {
            if disk[start].is_none() {
                disk.swap(start, end);
            } else {
                start += 1;
            }
        } else {
            end -= 1;
        }
    }
}

fn whole_file_defrag(disk: &mut [Option<usize>]) {
    let mut last_data_idx = disk.len() - 1;
    while disk[last_data_idx].is_none() {
        last_data_idx -= 1;
    }
    let mut id = disk[last_data_idx].unwrap();
    while id > 0 {
        let (fstart, fend) = find_file(disk, id, last_data_idx);
        last_data_idx = fend;
        let size = fend - fstart + 1;
        if let Some((estart, eend)) = find_empty(disk, size, fstart) {
            for (i, j) in (fstart..=fend).zip(estart..=eend) {
                disk.swap(i, j);
            }
        }
        id -= 1;
    }
}

fn find_file(disk: &[Option<usize>], id: usize, hint: usize) -> (usize, usize) {
    let mut start = 0;
    let mut end = 0;
    let mut found = false;

    for i in (0..=hint).rev() {
        if let Some(val) = disk[i] {
            if val == id {
                if !found {
                    found = true;
                    end = i;
                }
            } else if found {
                start = i + 1;
                break;
            }
        } else if found {
            start = i + 1;
            break;
        }
    }
    (start, end)
}

fn find_empty(disk: &[Option<usize>], size: usize, before_idx: usize) -> Option<(usize, usize)> {
    for i in 0..before_idx {
        if disk.iter().skip(i).take(size).all(Option::is_none) {
            return Some((i, i + size - 1));
        }
    }
    None
}

fn checksum(disk: &[Option<usize>]) -> usize {
    disk.iter()
        .enumerate()
        .filter_map(|(i, val)| val.map(|num| num * i))
        .sum()
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_part1() {
        let input = "2333133121414131402";
        let answer = part1(input);
        assert_eq!(answer, 1928);
    }

    #[test]
    fn test_part2() {
        let input = "2333133121414131402";
        let answer = part2(input);
        assert_eq!(answer, 2858);
    }
}
