// https://adventofcode.com/2021/day/2

use mylib::{ read_lines, parse_lines };
use std::time::Instant;

mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("p1.example.txt").expect("read_lines failed")),
            150
        );
    }

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            2027977
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            part_2(read_lines("p1.example.txt").expect("read_lines failed")),
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

// How many measurements are larger than the previous measurement?
fn part_1(lines: impl Iterator<Item = String>) -> u32 {
    let commands: Vec<types::Command> = parse_lines(lines).collect();
    let state = types::State::new();
    let final_state = commands.iter()
            .fold(state, | mut state, command | {
                state.process(command);
                state
            });
    final_state.product()
}

fn part_2(lines: impl Iterator<Item = String>) {
}
