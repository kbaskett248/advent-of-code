use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

use regex::Regex;

#[macro_use]
extern crate lazy_static;

mod lib;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(lib::read_lines("input.txt").expect("read_lines failed")),
            197
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(lib::read_lines("input.txt").expect("read_lines failed")),
            85324
        );
    }

    #[test]
    fn test_parse_bag_line() {
        let line = "dull silver bags contain 2 striped magenta bags, 2 dark coral bags, 1 bright orange bag, 4 plaid blue bags.";
        let parsed = line.parse::<BagSpec>().ok();
        assert_eq!(
            parsed,
            Some(BagSpec {
                color: String::from("dull silver"),
                contents: vec![
                    BagContent {
                        color: String::from("striped magenta"),
                        count: 2
                    },
                    BagContent {
                        color: String::from("dark coral"),
                        count: 2
                    },
                    BagContent {
                        color: String::from("bright orange"),
                        count: 1
                    },
                    BagContent {
                        color: String::from("plaid blue"),
                        count: 4
                    }
                ]
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

fn part_1(lines: impl Iterator<Item = String>) -> usize {
    let mut containers: HashMap<String, HashSet<String>> = HashMap::new();

    // Parse the BagSpecs, then convert them to a map keyed by the contained
    // bag. The value is the set of all bags that can contain that bag.
    for bagspec in lines.filter_map(|s| s.parse::<BagSpec>().ok()) {
        for content in bagspec.contents {
            match containers.get_mut(content.color.as_str()) {
                Some(set) => {set.insert(bagspec.color.clone());},
                None => {
                    let mut set = HashSet::new();
                    set.insert(bagspec.color.clone());
                    containers.insert(content.color, set);
                },
            };
        };
    };

    // Iterate over a worklist of bags, starting with "shiny gold".
    // For each worklist item, add the bags that can contain that bag,
    // then remove the item from 
    let mut offset = 0;
    let mut bags_with_gold = vec!["shiny gold"];
    while offset < bags_with_gold.len() {
        let bag = bags_with_gold[offset];
        if let Some(bags) = containers.get(bag) {
            for b in bags {
                bags_with_gold.push(b)
            }
        }
        offset += 1;
    }
    // Remove duplicates
    bags_with_gold.sort_unstable();
    bags_with_gold.dedup();
    // Subtract 1 to remove the shiny gold bag from the list
    bags_with_gold.len() - 1
}

fn part_2(lines: impl Iterator<Item = String>) -> usize {
    let specs: HashMap<_, _> = lines
        .filter_map(|line| line.parse::<BagSpec>().ok())
        .map(|spec| (spec.color.clone(), spec))
        .collect();

    // return the number of bags contained in the specified bag
    fn get_bags_count(specs: &HashMap<String, BagSpec>, bag: &str) -> usize {
        let mut count: usize = 0;
        for content in &specs[bag].contents {
            count += (content.count as usize) * (get_bags_count(specs, &content.color) + 1);
        }
        count
    }
    
    get_bags_count(&specs, "shiny gold")
}

#[derive(Debug, Eq)]
struct BagSpec {
    color: String,
    contents: Vec<BagContent>,
}

#[derive(Debug, Eq)]
struct BagContent {
    color: String,
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

impl FromStr for BagSpec {
    type Err = BagSpecParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref RE_LINE: Regex =
                Regex::new(r"^(?P<color>[a-z ]+?) bags contain (?P<contents>[a-z0-9 ,]+).$")
                    .unwrap();
            static ref CONT: Regex =
                Regex::new(r"(?P<count>\d+) (?P<color>[a-z ]+?) bags?").unwrap();
        }
        let caps = RE_LINE.captures(s).ok_or(BagSpecParseError)?;
        let color = caps
            .name("color")
            .ok_or(BagSpecParseError)?
            .as_str()
            .to_owned();
        let content_string = caps.name("contents").ok_or(BagSpecParseError)?.as_str();
        let contents: Vec<BagContent> = CONT
            .captures_iter(content_string)
            .filter_map(|caps| {
                let color = caps.name("color")?.as_str().to_owned();
                let count = caps.name("count")?.as_str().parse::<i8>().ok()?;
                Some(BagContent { color, count })
            })
            .collect();
        Ok(BagSpec { color, contents })
    }
}

#[derive(Debug)]
struct BagSpecParseError;
impl fmt::Display for BagSpecParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "BagSpec parse error")
    }
}
impl Error for BagSpecParseError {}
