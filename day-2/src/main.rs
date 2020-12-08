use std::fmt;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Password {
    text: String,
    rule: String,
}

impl Password {
    fn new(line: &str) -> Option<Password> {
        let mut i = line.split(':');
        let rule = i.next()?.trim();
        let text = i.next()?.trim();
        Some(Password {
            text: (*text).to_string(),
            rule: (*rule).to_string(),
        })
    }
}

impl fmt::Display for Password {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}: {}", self.text, self.rule)
    }
}

fn main() {
    if let Some(lines) = read_lines("input.txt") {
        for line in lines {
            if let Some(password) = Password::new(&line) {
                println!("{}", password)
            }
        }
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
