use std::fs::File;
use std::io::{self, BufRead, BufReader, Lines};
use std::iter::FilterMap;
use std::path::Path;

#[derive(std::cmp::PartialEq)]
enum Square {
    Open,
    Tree,
}

impl Square {
    fn from_char(c: char) -> Option<Square> {
        match c {
            '#' => Some(Square::Tree),
            '.' => Some(Square::Open),
            _ => None
        }
    }

    fn is_tree(&self) -> bool {
        *self == Square::Tree
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        file_to_map(lines)
    }
}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}

fn file_to_map(lines: impl Iterator<Item = String>) -> Vec<Vec<char>> {
    let mut map: Vec<Vec<char>> = vec![];
    for (i, line) in lines.enumerate() {
        for (j, c) in line.chars().enumerate() {
            match map.get_mut(j) {
                None => map.push(vec![c]),
                Some(column) => column.push(c),
            };
        }
    }
    map
}
