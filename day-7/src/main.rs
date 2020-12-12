use regex::Regex;

#[macro_use]
extern crate lazy_static;

mod lib;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_part_1() {
        assert_eq!(
            part_1(lib::read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }

    #[test]
    #[ignore]
    fn test_part_2() {
        assert_eq!(
            part_2(lib::read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }

    #[test]
    fn test_parse_bag_line() {
        let line = "dull silver bags contain 2 striped magenta bags, 2 dark coral bags, 1 bright orange bag, 4 plaid blue bags.";
        let parsed = parse_bag_line(line);
        assert_eq!(parsed, Some(BagSpec{color: "dull silver"}));
    }
}

fn main() {
    let p1 = part_1(lib::read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?}", p1);
    let p2 = part_2(lib::read_lines("input.txt").expect("read_lines failed"));
    println!("PART 2: {:?}", p2);
}

fn part_1(lines: impl Iterator<Item = String>) -> () {
    
}

fn part_2(lines: impl Iterator<Item = String>) -> () {
    
}

#[derive(Debug, Eq)]
struct BagSpec {
    color: & 'static str,
    // contents: Vec<BagContent>
}

#[derive(Debug)]
struct BagContent {
    color: String,
    count: i8
}

impl PartialEq for BagSpec {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

fn parse_bag_line(line: & 'static str) -> Option<BagSpec> {
    lazy_static! {
        static ref RE_LINE: Regex = Regex::new(r"^(?P<color>[a-z ]+?) bags contain (?P<contents>[a-z0-9 ,]+).$").unwrap();
    }
    let caps = RE_LINE.captures(line)?;
    let color = caps.name("color")?.as_str();
    Some(BagSpec{color})
}
