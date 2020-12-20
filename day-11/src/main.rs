use std::cmp::{min, max};
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
    fn neighbors(seats: &Vec<Vec<types::Tile>>, row: usize, col: usize) -> Vec<types::Tile> {
        let r_min: usize = max(row as i8 - 1, 0) as usize;
        let r_max: usize = min(row + 1, seats.len() - 1) as usize;

        let c_min: usize = max(col as i8 - 1, 0) as usize;
        let c_max: usize = min(col + 1, seats[row].len() - 1) as usize;

        let mut tiles = Vec::with_capacity(8);

        for r in r_min..=r_max {
            for c in c_min..=c_max {
                if !(r == row && c == col) {
                    tiles.push(seats[r][c]);
                }
            }
        }

        tiles
    }
    
    let mut chart = types::SeatingChart::from_lines(lines, 4, neighbors).peekable();
    let mut current = chart.next().expect("No seating chart");
    while current != *chart.peek().expect("Failed to return item") {
        current = chart.next().expect("Failed to return item");
    }
    println!("{}", current.to_string());
    current.count_occupied()
}

fn part_2(lines: impl Iterator<Item = String>) {}
