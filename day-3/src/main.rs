use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(std::cmp::PartialEq, Debug)]
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
                .filter_map(Square::from_char)
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

    fn navigate(&self, slope: (usize, usize)) -> (usize, usize) {
        let (rise, run) = slope;
        let path = self
            .iter()
            .step_by(run)
            .zip((0..).step_by(rise))
            .map(|(column, height)| column.get(height))
            .take_while(|op| op.is_some());
        let (trees, empty): (Vec<Option<&Square>>, Vec<Option<&Square>>) =
            path.partition(|&x| x.map_or(false, Square::is_tree));
        let num_trees = trees.len();
        (num_trees, num_trees + empty.len())
    }
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        let map = Map::from_lines(lines);
        let (trees, down) = map.navigate((1, 3));
        println!(
            "PART 1\n{} trees were hit.\nYou went {} squares down the hill.",
            trees, down
        )
    }
}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}
