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

fn read_lines<P>(filename: P) -> Result<Vec<i16>, >
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    let lines = io::BufReader::new(file).lines();
    Some(
        lines.filter_map(
            |x| {
                if let Ok(line) = x {
                    Some(line.parse::<i16>().unwrap())
                } else {
                    None
                }
            }
        ).collect()
    )
}
