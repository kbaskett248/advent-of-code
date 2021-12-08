use mylib::read_lines;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("example.txt").expect("read_lines failed")),
            37
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            347509
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

fn part_1(mut lines: impl Iterator<Item = String>) -> u32 {
    let first_line = lines.next().expect("Could not parse first line!");
    let numbers: Vec<u16> = first_line
        .split(',')
        .filter_map(|s| s.parse::<u16>().ok())
        .collect();
    let median = stats::median(numbers.iter().cloned())
        .expect("No median found")
        .round() as u16;
    numbers
        .iter()
        .map(|x| (*x as i32 - median as i32).abs() as u32)
        .sum()
}

fn part_2(lines: impl Iterator<Item = String>) {}
