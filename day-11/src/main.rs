use std::time::Instant;
use mylib::read_lines;

mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            37
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
    let mut chart = types::SeatingChart::from_lines(lines);
    let first = chart.next().expect("No seating chart");
    if let Some(final_chart) = chart.scan(first, |prev, next| {
        if next == *prev {
            None
        } else {
            Some(next)
        }
    })
    .last() {
        final_chart.count_occupied()
    } else {
        0
    }
}

fn part_2(lines: impl Iterator<Item = String>) {}
