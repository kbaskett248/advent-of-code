use mylib::read_lines;
use std::time::Instant;

mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("example.txt").expect("read_lines failed")),
            ()
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            ()
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
    let first_line = lines.next();
    first_line.split(",")
              .map(| s | -> s.parse::<types::Lanternfish>())
              .fold(0, | num_fish, fish | -> num_fish + fish.spawn(80))
}

fn part_2(lines: impl Iterator<Item = String>) {}
