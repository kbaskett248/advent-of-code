use std::convert::identity;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            6416
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }
}

fn main() {
    let p1 = part_1(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {}", p1);
}

fn part_1(lines: impl Iterator<Item = String>) -> usize {
    chunk_lines(lines)
        .map(|forms| {
            let mut answers: Vec<char> = forms
                .iter()
                .flat_map(|x| x.chars())
                .collect();
            answers.sort_unstable();
            answers.dedup();
            answers.len()
        })
        .sum()
}

fn part_2(lines: impl Iterator<Item = String>) {
    panic!("Implement part 2");
}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}

fn chunk_lines(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Vec<String>> {
    lines
        .scan(vec![], |container, line| match line.as_str() {
            "" => {
                let chunk = Some(Some(container.clone()));
                container.clear();
                chunk
            }
            _ => {
                container.push(line);
                Some(None)
            }
        })
        .filter_map(identity)
}
