use std::convert::identity;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::str::FromStr;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

pub fn read_lines<P>(filename: P) -> io::Result<impl Iterator<Item = String>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines().filter_map(|x| x.ok()))
}

pub fn chunk_lines(lines: impl Iterator<Item = String>) -> impl Iterator<Item = Vec<String>> {
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

pub fn chunk_lines_n(
    lines: impl Iterator<Item = String>,
    count: usize,
) -> impl Iterator<Item = Vec<String>> {
    lines
        .scan(vec![], move |container, line| {
            container.push(line);
            if container.len() >= count {
                let chunk = Some(Some(container.clone()));
                container.clear();
                chunk
            } else {
                Some(None)
            }
        })
        .filter_map(identity)
}

pub fn parse_lines<T>(lines: impl Iterator<Item = String>) -> impl Iterator<Item = T>
where
    T: FromStr,
{
    lines.filter_map(|line| line.parse::<T>().ok())
}
