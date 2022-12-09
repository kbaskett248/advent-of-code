use mylib::read_lines;
use std::collections::HashSet;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("example.txt").expect("read_lines failed")),
            157
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            7845
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            part_2(read_lines("example.txt").expect("read_lines failed")),
            ()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }
}

fn main() {
    let mut start = Instant::now();
    let p1 = part_1(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?} ({:?})", p1, start.elapsed());

    start = Instant::now();
    let p2 = part_2(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 2: {:?} ({:?})", p2, start.elapsed());
}

fn part_1(lines: impl Iterator<Item = String>) -> u32 {
    lines
        .map(|line| {
            let l = line.len() / 2;
            let comp_1: HashSet<char> = line[0..l].chars().collect();
            let comp_2: HashSet<char> = line[l..].chars().collect();
            let common: char = *comp_1.intersection(&comp_2).next().unwrap();
            let value = common as u32;
            if 97 <= value && value <= 123 {
                value - 96
            } else {
                value - 38
            }
        })
        .sum()
}

fn part_2(lines: impl Iterator<Item = String>) {}
