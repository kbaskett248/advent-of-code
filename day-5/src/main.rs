use std::cmp::{Ord, Ordering};
use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            880
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed"))
                .expect("found no pass")
                .id()
                + 1,
            731
        );
    }
}

fn main() {
    let lines: Vec<String> = read_lines("input.txt")
        .expect("Couldn't read file")
        .collect();
    println!("The max seat ID is {}.", part_1(lines.iter().cloned()));
    if let Some(pass) = part_2(lines.iter().cloned()) {
        println!(
            "The previous pass is {:?}.\nThe next ID is {}.",
            pass,
            pass.id() + 1
        )
    }
}

fn part_1(lines: impl Iterator<Item = String>) -> i32 {
    if let Some(max) = lines
        .filter_map(|l| l.parse::<BoardingPass>().ok())
        .map(|p| p.id())
        .max()
    {
        max
    } else {
        0
    }
}

fn part_2(lines: impl Iterator<Item = String>) -> Option<BoardingPass> {
    let mut passes: Vec<BoardingPass> = lines
        .filter_map(|l| l.parse::<BoardingPass>().ok())
        .collect();
    passes.sort_unstable();
    if let Some((first, _)) = passes
        .iter()
        .cloned()
        .zip(passes.iter().skip(1))
        .inspect(|a| println!("{:?}", a))
        .skip_while(|(x, y)| y.id() - x.id() != 2)
        .nth(0)
    {
        Some(first)
    } else {
        None
    }
}

#[derive(Debug, Clone, Eq)]
struct BoardingPass {
    rep: String,
    row: i32,
    seat: i32,
}

impl BoardingPass {
    fn id(&self) -> i32 {
        (self.row * 8 + self.seat).into()
    }
}

impl FromStr for BoardingPass {
    type Err = BoardingPassError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row: i32 = bin_str_to_int(s[..7].replace('F', "0").replace('B', "1"))?;
        let seat: i32 = bin_str_to_int(s[7..].replace('L', "0").replace('R', "1"))?;
        Ok(BoardingPass {
            rep: s.to_string(),
            row,
            seat,
        })
    }
}

impl Ord for BoardingPass {
    fn cmp(&self, other: &Self) -> Ordering {
        self.id().cmp(&other.id())
    }
}

impl PartialOrd for BoardingPass {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for BoardingPass {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

fn bin_str_to_int(s: String) -> Result<i32, BoardingPassError> {
    match i32::from_str_radix(&s, 2) {
        Err(_) => Err(BoardingPassError),
        Ok(num) => Ok(num),
    }
}

#[derive(Debug)]
struct BoardingPassError;
impl fmt::Display for BoardingPassError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BoardingPass Parse Error")
    }
}
impl Error for BoardingPassError {}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}
