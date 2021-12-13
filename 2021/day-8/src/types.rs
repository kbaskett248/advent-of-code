use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug)]
pub struct Entry {
    pub patterns: Vec<String>,
    pub digits: Vec<String>,
}

impl FromStr for Entry {
    type Err = EntryParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split('|');
        let patterns = parts
            .next()
            .ok_or(EntryParseError {})?
            .trim()
            .split(' ')
            .map(|a| a.to_string())
            .collect();
        let digits = parts
            .next()
            .ok_or(EntryParseError {})?
            .trim()
            .split(' ')
            .map(|a| a.to_string())
            .collect();
        Ok(Entry { patterns, digits })
    }
}

#[derive(Debug)]
pub struct EntryParseError {}

impl Display for EntryParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Entry parse error")
    }
}
impl Error for EntryParseError {}

#[derive(Debug)]
pub struct MappedEntry {
    pub digit_mapping: HashMap<String, u8>,
    pub digits: Vec<String>,
}

fn sort_string(s: &str) -> String {
    let mut sorted_digit = s.chars().collect::<Vec<char>>();
    sorted_digit.sort_unstable();
    String::from_iter(sorted_digit)
}

impl From<Entry> for MappedEntry {
    fn from(entry: Entry) -> MappedEntry {
        let mut digit_mapping: HashMap<String, u8> = HashMap::with_capacity(10);
        let mut set_mapping: HashMap<u8, HashSet<char>> = HashMap::with_capacity(10);
        let mut remaining_patterns: Vec<String> = Vec::with_capacity(6);
        for pattern in entry.patterns {
            let sorted_pattern = sort_string(&pattern);
            let pattern_set: HashSet<char> = HashSet::from_iter(pattern.chars());
            let matched_number = match sorted_pattern.len() {
                2 => 1,
                3 => 7,
                4 => 4,
                7 => 8,
                _ => 10,
            };
            if matched_number == 10 {
                remaining_patterns.push(sorted_pattern)
            } else {
                digit_mapping.insert(sorted_pattern, matched_number);
                set_mapping.insert(matched_number, pattern_set);
            }
        }
        for pattern in remaining_patterns {
            let pattern_set: HashSet<char> = HashSet::from_iter(pattern.chars());
            let matched_number = match pattern.len() {
                5 => {
                    if set_mapping
                        .get(&1)
                        .expect("1 missing")
                        .difference(&pattern_set)
                        .count()
                        == 0
                    {
                        3
                    } else if set_mapping
                        .get(&4)
                        .expect("4 missing")
                        .difference(&pattern_set)
                        .count()
                        == 1
                    {
                        5
                    } else {
                        2
                    }
                }
                6 => {
                    if set_mapping
                        .get(&1)
                        .expect("1 missing")
                        .difference(&pattern_set)
                        .count()
                        == 1
                    {
                        6
                    } else if set_mapping
                        .get(&4)
                        .expect("4 missing")
                        .difference(&pattern_set)
                        .count()
                        == 0
                    {
                        9
                    } else {
                        0
                    }
                }
                _ => 10,
            };
            digit_mapping.insert(pattern, matched_number);
        }
        MappedEntry {
            digit_mapping,
            digits: entry.digits.iter().map(|s| sort_string(s)).collect(),
        }
    }
}

impl MappedEntry {
    pub fn value(&self) -> u32 {
        self.digits
            .iter()
            .map(|d| std::char::from_digit(*self.digit_mapping.get(d).unwrap() as u32, 10).unwrap())
            .collect::<String>()[..]
            .parse::<u32>()
            .unwrap()
    }
}
