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
            5934
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            380612
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            part_2(read_lines("example.txt").expect("read_lines failed")),
            26984457539
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            0
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

fn part_1(mut lines: impl Iterator<Item = String>) -> u64 {
    let first_line = lines.next().expect("Could not parse first line!");
    first_line.split(",")
              .map(| s | {s.parse::<types::Lanternfish>()})
              .fold(0, | num_fish, fish_result | {
                  match fish_result {
                      Ok(fish) => num_fish + fish.spawn(80),
                      _ => num_fish,
                  }
              })
}

fn part_2(mut lines: impl Iterator<Item = String>) -> u64 {
    let first_line = lines.next().expect("Could not parse first line!");
    first_line.split(",")
              .map(| s | {s.parse::<types::Lanternfish>()})
              .fold(0, | num_fish, fish_result | {
                  match fish_result {
                      Ok(fish) => num_fish + fish.spawn(256),
                      _ => num_fish,
                  }
              })
}
