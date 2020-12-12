use std::error::Error;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        println!("The max seat ID is {}.", part_1(lines));
    }
}

fn part_1(lines: impl Iterator<Item = String>) -> i16 {
    lines
        .map(|l| (l[0..7], l[7..]))
        .

    0
}

struct Passport {
    rep: String,
    row: i8,
    seat: i8,
}

impl FromStr for Passport {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s[0..7].chars()
            .map(|c| {
                match c {
                    'F' => Ok(0),
                    'B' => 1,
                    _ => PassportError
                }
            });

        Ok(Passport{
            rep: s.to_string(),
            row: 0,
            seat: 0
        })
    }
}

#[derive(Debug)]
struct PassportError;
impl fmt::Display for PassportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Passport Parse Error")
    }
}
impl Error for PassportError {}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}
