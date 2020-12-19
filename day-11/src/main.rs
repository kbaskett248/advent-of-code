use mylib::read_lines;
use std::time::Instant;

mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            2113
        );
    }

    #[test]
    fn test_part_1_example() {
        assert_eq!(
            part_1(read_lines("p1.example.txt").expect("read_lines failed")),
            37
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

// Simulate your seating area by applying the seating rules repeatedly
// until no seats change state. How many seats end up occupied?
fn part_1(lines: impl Iterator<Item = String>) -> usize {
    let mut chart = types::SeatingChart::from_lines(lines, 4).peekable();
    let mut current = chart.next().expect("No seating chart");
    while current != *chart.peek().expect("Failed to return item") {
        current = chart.next().expect("Failed to return item");
    }
    println!("{}", current.to_string());
    current.count_occupied()
}

fn part_2(lines: impl Iterator<Item = String>) {}
