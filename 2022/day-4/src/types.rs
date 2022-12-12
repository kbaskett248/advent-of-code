use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectionAssignment {
    pub start: usize,
    pub end: usize,
}

impl SectionAssignment {
    fn contains(&self, other: &SectionAssignment) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn partially_contains(&self, other: &SectionAssignment) -> bool {
        (self.start >= other.start && self.start <= other.end)
            || (self.end >= other.start && self.end <= other.end)
    }
}

impl FromStr for SectionAssignment {
    type Err = SectionAssignmentParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut bounds = s.split('-');
        let start = bounds
            .next()
            .ok_or(SectionAssignmentParseError {})?
            .parse::<usize>()
            .map_err(|_| SectionAssignmentParseError {})?;
        let end = bounds
            .next()
            .ok_or(SectionAssignmentParseError {})?
            .parse::<usize>()
            .map_err(|_| SectionAssignmentParseError {})?;
        Ok(SectionAssignment { start, end })
    }
}

#[derive(Debug)]
pub struct SectionAssignmentParseError {}

impl Display for SectionAssignmentParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SectionAssignment parse error")
    }
}
impl Error for SectionAssignmentParseError {}

#[derive(Debug, PartialEq, Eq)]
pub struct Pair {
    first: SectionAssignment,
    second: SectionAssignment,
}

impl Pair {
    pub fn fully_overlapping(&self) -> Option<SectionAssignment> {
        if self.first.contains(&self.second) {
            Some(self.first)
        } else if self.second.contains(&self.first) {
            Some(self.second)
        } else {
            None
        }
    }

    pub fn partial_overlapping(&self) -> Option<SectionAssignment> {
        if self.first.partially_contains(&self.second) {
            Some(self.first)
        } else if self.second.partially_contains(&self.first) {
            Some(self.second)
        } else {
            None
        }
    }
}

impl FromStr for Pair {
    type Err = PairParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut assignments = s.split(',');
        let first = assignments
            .next()
            .ok_or(PairParseError {})?
            .parse::<SectionAssignment>()
            .map_err(|_| PairParseError {})?;
        let second = assignments
            .next()
            .ok_or(PairParseError {})?
            .parse::<SectionAssignment>()
            .map_err(|_| PairParseError {})?;
        Ok(Pair { first, second })
    }
}

#[derive(Debug)]
pub struct PairParseError {}

impl Display for PairParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pair parse error")
    }
}
impl Error for PairParseError {}
