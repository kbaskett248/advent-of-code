// https://adventofcode.com/2021/day/1

use mylib::{ read_lines, parse_lines };
use std::time::Instant;

// mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("p1.example.txt").expect("read_lines failed")),
            7
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            1477
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
    let start = Instant::now();
    let p1 = part_1(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?} ({:?})", p1, start.elapsed());

    // start = Instant::now();
    // let p2 = part_2(read_lines("input.txt").expect("read_lines failed"));
    // println!("PART 2: {:?} ({:?})", p2, start.elapsed());
}

// How many measurements are larger than the previous measurement?
fn part_1(lines: impl Iterator<Item = String>) -> i32 {
    let nums: Vec<u16> = parse_lines(lines).collect();
    let result = nums.iter()
                     .zip(nums.iter().skip(1))
                     .fold(0, |counter: i32, (&num1, &num2)| {
                         if num2 > num1 {
                             return counter + 1;
                         }
                         return counter;
                     });
    return result;
}

fn part_2(lines: impl Iterator<Item = String>) {}
