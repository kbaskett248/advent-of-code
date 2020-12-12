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
        assert_eq!(
            parsed,
            Some(BagSpec {
                color: "dull silver",
                contents: vec![BagContent {
                    color: "striped magenta",
                    count: 2
                },
                BagContent {
                    color: "dark coral",
                    count: 2
                },
                BagContent {
                    color: "bright orange",
                    count: 1
                },
                BagContent {
                    color: "plaid blue",
                    count: 4
                }]
            })
        );
    }
}

fn main() {
    let p1 = part_1(lib::read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?}", p1);
    let p2 = part_2(lib::read_lines("input.txt").expect("read_lines failed"));
    println!("PART 2: {:?}", p2);
}

fn part_1(lines: impl Iterator<Item = String>) -> () {}

fn part_2(lines: impl Iterator<Item = String>) -> () {}

#[derive(Debug, Eq)]
struct BagSpec {
    color: &'static str,
    contents: Vec<BagContent>,
}

#[derive(Debug, Eq)]
struct BagContent {
    color: &'static str,
    count: i8,
}

impl PartialEq for BagSpec {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color
    }
}

impl PartialEq for BagContent {
    fn eq(&self, other: &Self) -> bool {
        (self.color == other.color) && (self.count == other.count)
    }
}

fn parse_bag_line(line: &'static str) -> Option<BagSpec> {
    lazy_static! {
        static ref RE_LINE: Regex =
            Regex::new(r"^(?P<color>[a-z ]+?) bags contain (?P<contents>[a-z0-9 ,]+).$").unwrap();
        static ref CONT: Regex = Regex::new(r"^(?P<count>\d+) (?P<color>[a-z ]+?) bags?").unwrap();
    }
    let caps = RE_LINE.captures(line)?;
    let color = caps.name("color")?.as_str();
    let content_string = caps.name("contents")?.as_str();
    let contents: Vec<BagContent> = CONT
        .captures_iter(content_string)
        .filter_map(|caps| {
            let color = caps.name("color")?.as_str();
            let count = caps.name("count")?.as_str().parse::<i8>().ok()?;
            Some(BagContent { color, count })
        })
        .collect();
    Some(BagSpec { color, contents })
}
