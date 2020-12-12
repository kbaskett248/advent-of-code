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
}

fn main() {
    if let Ok(lines) = read_lines("input.txt") {
        println!("The max seat ID is {}.", part_1(lines));
    }
}

fn part_1(lines: impl Iterator<Item = String>) -> i32 {
    if let Some(max) = lines
        .filter_map(|l| l.parse::<Passport>().ok())
        .map(|p| p.id())
        .max()
    {
        max
    } else {
        0
    }
}

#[derive(Debug)]
struct Passport {
    rep: String,
    row: i32,
    seat: i32,
}

impl Passport {
    fn id(&self) -> i32 {
        (self.row * 8 + self.seat).into()
    }
}

impl FromStr for Passport {
    type Err = PassportError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let row: i32 = bin_str_to_int(s[..7].replace('F', "0").replace('B', "1"))?;
        let seat: i32 = bin_str_to_int(s[7..].replace('L', "0").replace('R', "1"))?;
        Ok(Passport {
            rep: s.to_string(),
            row,
            seat,
        })
    }
}

fn bin_str_to_int(s: String) -> Result<i32, PassportError> {
    match i32::from_str_radix(&s, 2) {
        Err(_) => Err(PassportError),
        Ok(num) => Ok(num),
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
