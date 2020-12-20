// https://adventofcode.com/2020/day/11

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
            1865
        );
    }

    #[test]
    fn test_part_2_example() {
        assert_eq!(
            part_2(read_lines("p1.example.txt").expect("read_lines failed")),
            26
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
    // println!("{}", current.to_string());
    current.count_occupied()
}

// Given the new visibility method and the rule change for occupied seats becoming empty, 
// once equilibrium is reached, how many seats end up occupied?
fn part_2(lines: impl Iterator<Item = String>) -> usize {
    fn neighbors(seats: &Vec<Vec<types::Tile>>, row: usize, col: usize) -> Vec<types::Tile> {
        let modifiers: Vec<fn(i8) -> i8> = vec![|n| n-1, |n| n, |n| n+1];

        let r_max = (seats.len() - 1) as i8;
        let c_max = (seats[row].len() - 1) as i8;

        let mut tiles = Vec::with_capacity(8);

        for rm in &modifiers {
            for cm in &modifiers {
                let mut r = rm(row as i8);
                let mut c = cm(col as i8);

                if (r, c) == (row as i8, col as i8) {
                    continue
                }

                while r >= 0 && r <= r_max && c >= 0 && c <= c_max {
                    match seats[r as usize][c as usize] {
                        types::Tile::Floor => {
                            r = rm(r);
                            c = cm(c);
                        },
                        seat @ types::Tile::Seat{..} => {
                            tiles.push(seat);
                            break
                        }
                    }
                }
            }
        }

        tiles
    }
    
    let mut chart = types::SeatingChart::from_lines(lines, 5, neighbors).peekable();
    let mut current = chart.next().expect("No seating chart");
    while current != *chart.peek().expect("Failed to return item") {
        current = chart.next().expect("Failed to return item");
    }
    // println!("{}", current.to_string());
    current.count_occupied()
}
