use mylib::read_lines;
use std::time::Instant;

use std::cmp::min;

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
            168
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            98257206
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

fn part_2(mut lines: impl Iterator<Item = String>) -> u32 {
    let first_line = lines.next().expect("Could not parse first line!");
    let numbers: Vec<u16> = first_line
        .split(',')
        .filter_map(|s| s.parse::<u16>().ok())
        .collect();
    let mean = (stats::mean(numbers.iter().cloned()).round()) as u16;
    min(
        numbers
            .iter()
            .map(|x| (*x as i32 - mean as i32).abs() as u32)
            .map(|x| x * (x + 1) / 2)
            .sum(),
        numbers
            .iter()
            .map(|x| (*x as i32 - mean as i32 + 1).abs() as u32)
            .map(|x| x * (x + 1) / 2)
            .sum(),
    )
}
