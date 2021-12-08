use mylib::read_lines;
use std::time::Instant;

mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            count_fish(read_lines("example.txt").expect("read_lines failed"), 80),
            5934
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            count_fish(read_lines("input.txt").expect("read_lines failed"), 80),
            380612
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            count_fish(read_lines("example.txt").expect("read_lines failed"), 256),
            26984457539
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            count_fish(read_lines("input.txt").expect("read_lines failed"), 256),
            1710166656900
        );
    }
}

fn main() {
    let mut start = Instant::now();
    let p1 = count_fish(read_lines("input.txt").expect("read_lines failed"), 80);
    println!("PART 1: {:?} ({:?})", p1, start.elapsed());

    start = Instant::now();
    let p2 = count_fish(read_lines("input.txt").expect("read_lines failed"), 256);
    println!("PART 2: {:?} ({:?})", p2, start.elapsed());
}

fn count_fish(mut lines: impl Iterator<Item = String>, num_days: u16) -> u64 {
    let first_line = lines.next().expect("Could not parse first line!");
    first_line
        .split(",")
        .map(|s| s.parse::<types::Lanternfish>())
        .fold(0, |num_fish, fish_result| match fish_result {
            Ok(fish) => num_fish + types::spawn(fish, num_days),
            _ => num_fish,
        })
}
