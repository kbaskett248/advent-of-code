use std::convert::identity;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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
