use mylib::{chunk_lines, parse_lines, read_lines};
use std::time::Instant;

mod types;

#[macro_use]
extern crate lazy_static;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("example.txt").expect("read_lines failed")),
            "CMZ"
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            "TBVFVDZPN"
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            part_2(read_lines("example.txt").expect("read_lines failed")),
            "MCD"
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            "VLCWHTDSZ"
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

fn part_1(lines: impl Iterator<Item = String>) -> String {
    let mut chunked = chunk_lines(lines);
    let arrangement_lines = chunked.next().unwrap();
    let move_lines = chunked.next().unwrap();
    let mut stacks = types::Stacks::from_lines(arrangement_lines);
    let moves = parse_lines::<types::Move>(move_lines.iter().cloned());

    for m in moves {
        stacks.move_crates(&m);
    }

    stacks.top_crates().iter().collect::<String>()
}

fn part_2(lines: impl Iterator<Item = String>) -> String {
    let mut chunked = chunk_lines(lines);
    let arrangement_lines = chunked.next().unwrap();
    let move_lines = chunked.next().unwrap();
    let mut stacks = types::Stacks::from_lines(arrangement_lines);
    let moves = parse_lines::<types::Move>(move_lines.iter().cloned());

    for m in moves {
        stacks.move_crates_2(&m);
    }

    stacks.top_crates().iter().collect::<String>()
}
