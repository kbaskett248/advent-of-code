use std::convert::TryInto;
use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Password {
    text: String,
    letter: char,
    min: i8,
    max: i8,
}

impl Password {
    fn new(line: &str) -> Option<Password> {
        let mut i = line.split(':');
        let rule = i.next()?.trim();
        let text = i.next()?.trim();
        let mut j = rule.split(' ');
        let numbers = j.next()?.trim();
        let letter: char = j.next()?.trim().chars().next()?;
        let mut k = numbers.split('-');
        let min: i8 = k.next()?.parse::<i8>().ok()?;
        let max: i8 = k.next()?.parse::<i8>().ok()?;
        Some(Password {
            text: (*text).to_string(),
            letter: letter,
            min: min,
            max: max,
        })
    }

    fn valid_part_1(&self) -> bool {
        let char_list: Vec<char> = self.text.chars().filter(|x| *x == self.letter).collect();
        let length: i8 = char_list.len().try_into().unwrap();
        length >= self.min && length <= self.max
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}: {}-{} {}",
            self.text, self.min, self.max, self.letter
        )
    }
}

fn main() {
    if let Some(lines) = read_lines("input.txt") {
        let valid_pws: Vec<Password> = lines
            .iter()
            .filter_map(|line| Password::new(&line))
            .filter(|pw| pw.valid_part_1())
            .collect();
        println!("There are {} valid passwords", valid_pws.len());
    }
}

fn read_lines<P>(filename: P) -> Option<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    Some(lines.filter_map(|x| x.ok()).collect())
}
