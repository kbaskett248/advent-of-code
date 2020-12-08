use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    if let Some(mut lines) = read_lines("input.txt") {
        lines.sort_unstable();
        if let Some((num1, num2)) = find_pair(&lines) {
            println!("{} x {} = {}", num1, num2, (num1 as i64) * (num2 as i64))
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

fn find_pair(numbers: &[i16]) -> Option<(i16, i16)> {
    for num in numbers {
        let diff: i16 = 2020 - *num;
        if let Ok(_) = numbers.binary_search(&diff) {
            return Some((*num, diff));
        }
    }
    None
}
