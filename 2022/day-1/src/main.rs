use mylib::{chunk_lines, parse_lines, read_lines};
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("example.txt").expect("read_lines failed")),
            24000
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            66487
        );
    }

    // #[test]
    // fn test_part_2_example() {
    //     assert_eq!(
    //         part_2(read_lines("example.txt").expect("read_lines failed")),
    //         45000
    //     );
    // }

    // #[test]
    // fn test_part_2() {
    //     assert_eq!(
    //         part_2(read_lines("input.txt").expect("read_lines failed")),
    //         ()
    //     );
    // }
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
    total_cal_iter(lines).max().unwrap()
}

fn total_cal_iter(lines: impl Iterator<Item = String>) -> impl Iterator<Item = u32> {
    chunk_lines(lines).map(|chunk| {
        let calories = parse_lines::<u32>(chunk.into_iter());
        calories.sum::<u32>()
    })
}

fn part_2(lines: impl Iterator<Item = String>) {}
