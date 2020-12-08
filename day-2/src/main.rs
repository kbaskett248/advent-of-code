use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

struct Password{
    text: String,
    rule: String
}

fn main() {
    println!("Hello, world!");
}

fn read_lines<P>(filename: P) -> Option<Vec<String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    Some(
        lines
            .filter_map(|x| x.ok())
            .collect()
    )
}
