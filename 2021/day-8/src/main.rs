use mylib::{parse_lines, read_lines};
use std::time::Instant;

mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("example.txt").expect("read_lines failed")),
            26
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            342
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            part_2(read_lines("example.txt").expect("read_lines failed")),
            61229
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            1068933
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

fn part_1(lines: impl Iterator<Item = String>) -> u16 {
    parse_lines(lines).fold(0, |mut counter, entry: types::Entry| {
        for digit in entry.digits {
            match digit.len() {
                2 => counter += 1,
                3 => counter += 1,
                4 => counter += 1,
                7 => counter += 1,
                _ => (),
            }
        }
        counter
    })
}

fn part_2(lines: impl Iterator<Item = String>) -> u32 {
    parse_lines(lines)
        .map(|e: types::Entry| types::MappedEntry::from(e).value())
        .sum()
}
