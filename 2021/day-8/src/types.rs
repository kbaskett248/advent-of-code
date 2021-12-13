use std::collections::HashMap;
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

pub struct MappedEntry {
    pub digit_mapping: HashMap<String, u8>,
    pub digits: Vec<String>,
}

fn sort_string(s: &String) -> String {
    let mut sorted_digit = s.chars().collect::<Vec<char>>();
    sorted_digit.sort_unstable();
    String::from_iter(sorted_digit)
}

// 5: 2, 3, 5,
// 6: 0, 6, 9

impl From<Entry> for MappedEntry {
    fn from(entry: Entry) -> MappedEntry {
        let mut digit_mapping: HashMap<String, u8> = HashMap::with_capacity(10);
        let mut remaining_patterns: Vec<String> = Vec::with_capacity(6);
        for pattern in entry.patterns {
            let sorted_pattern = sort_string(&pattern);
            match sorted_pattern.len() {
                2 => {
                    digit_mapping.insert(sorted_pattern, 1);
                }
                3 => {
                    digit_mapping.insert(sorted_pattern, 7);
                }
                4 => {
                    digit_mapping.insert(sorted_pattern, 4);
                }
                7 => {
                    digit_mapping.insert(sorted_pattern, 8);
                }
                _ => remaining_patterns.push(sorted_pattern),
            };
        }
        for pattern in remaining_patterns {}
        MappedEntry {
            digit_mapping,
            digits: entry.digits,
        }
    }
}
