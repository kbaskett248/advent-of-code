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
            _ => None,
        }
    }

    fn is_tree(&self) -> bool {
        *self == Square::Tree
    }
}

struct Map {
    map: Vec<Vec<Square>>,
}

impl Map {
    fn from_lines(lines: impl Iterator<Item = String>) -> Map {
        let mut map: Vec<Vec<Square>> = vec![];
        for line in lines {
            for (j, c) in line
                .chars()
                .filter_map(|x| Square::from_char(x))
                .enumerate()
            {
                match map.get_mut(j) {
                    None => map.push(vec![c]),
                    Some(column) => column.push(c),
                };
            }
        }
        Map { map }
    }

    fn iter(&self) -> impl Iterator<Item = &Vec<Square>> {
        self.map.iter().cycle()
    }

    fn navigate(&self, slope: (i8, i8)) -> (i16, i16, i16) {
        (0, 0, 0)
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let map = Map::from_lines(lines);
        let (trees, down, right) = map.navigate((1, 3));
        println!("PART 1\n{} trees were hit.\nYou went {} squares down the hill.\nYou went {} squares right.", trees, down, right)
    }
}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}
