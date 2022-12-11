use mylib::{chunk_lines_n, read_lines};
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
            70
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            2790
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
            char_to_priority(&common)
        })
        .sum()
}

fn char_to_priority(&c: &char) -> u32 {
    let value = c as u32;
    if 97 <= value && value <= 123 {
        value - 96
    } else {
        value - 38
    }
}

fn part_2(lines: impl Iterator<Item = String>) -> u32 {
    chunk_lines_n(lines, 3)
        .map(|line_vec| {
            let user_1: HashSet<char> = line_vec[0].chars().collect();
            let user_2: HashSet<char> = line_vec[1].chars().collect();
            let user_3: HashSet<char> = line_vec[2].chars().collect();
            match user_1
                .intersection(&user_2)
                .map(|a| *a)
                .collect::<HashSet<char>>()
                .intersection(&user_3)
                .next()
            {
                Some(c) => char_to_priority(c),
                None => 0,
            }
        })
        .sum()
}
