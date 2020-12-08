use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Some(lines) = read_lines("input.txt") {
        for line in lines {
            println!("{}", line)
        }
    } else {
        println!("None")
    }
}

fn read_lines<P>(filename: P) -> Option<Vec<i16>>
where P: AsRef<Path>, {
    let file = File::open(filename).ok()?;
    let lines = io::BufReader::new(file).lines();
    Some(
        lines.filter_map(
            |x| {
                if let Ok(line) = x {
                    line.parse::<i16>().ok()
                } else {
                    None
                }
            }
        ).collect()
    )
}
