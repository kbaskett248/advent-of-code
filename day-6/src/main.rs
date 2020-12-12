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
            880
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed"))
                .expect("found no pass")
                .id()
                + 1,
            731
        );
    }
}

fn main() {
    println!("Hello, world!");
}

fn part_1(lines: impl Iterator<Item = String>) {

}

fn part_2(lines: impl Iterator<Item = String>) {

}

fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}

fn chunk_lines(lines: impl Iterator<Item = String>) -> impl Iterator<Item = String> {
    lines
        .scan(vec![], |container, line| match line.as_str() {
            "" => {
                let chunk = Some(container.join(" "));
                container.clear();
                chunk
            }
            _ => {
                container.push(line);
                Some("".to_string())
            }
        })
        .filter(|x| x != "")
}
