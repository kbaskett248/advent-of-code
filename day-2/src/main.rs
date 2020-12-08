use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Password {
    text: String,
    rule: String,
}

impl Password {
    fn new(line: &str) -> Password {
        let password = Password {
            text: "text".to_string(),
            rule: "rule".to_string(),
        };
        password
    }
}

fn main() {
    if let Some(lines) = read_lines("input.txt") {
        for line in lines {
            let password = Password::new(&line);
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
