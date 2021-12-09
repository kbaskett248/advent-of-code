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
