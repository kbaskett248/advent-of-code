use regex::Regex;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug)]
pub struct Stacks {
    stacks: Vec<Vec<char>>,
}

impl Stacks {
    pub fn from_lines(lines: Vec<String>) -> Stacks {
        lazy_static! {
            static ref CRATE_MATCHER: Regex = Regex::new(r"(   |\[(?P<id>[A-Z])\]) ?").unwrap();
        }

        let mut stacks: Vec<Vec<char>> = Vec::with_capacity(10);
        for line in lines {
            for (i, cap) in CRATE_MATCHER.captures_iter(&line).enumerate() {
                while i + 1 > stacks.len() {
                    stacks.push(Vec::new())
                }
                if let Some(m) = cap.get(2) {
                    if let Some(v) = stacks.get_mut(i) {
                        if let Some(c) = m.as_str().chars().next() {
                            v.insert(0, c);
                        }
                    }
                }
            }
        }
        Stacks { stacks }
    }

    pub fn move_crates(&mut self, m: &Move) {
        let mut temp_stack = Vec::with_capacity(m.quantity);
        if let Some(stack_start) = self.stacks.get_mut(m.source - 1) {
            for _ in 0..m.quantity {
                if let Some(c) = stack_start.pop() {
                    temp_stack.push(c);
                }
            }
        }
        if let Some(stack_end) = self.stacks.get_mut(m.destination - 1) {
            stack_end.append(&mut temp_stack);
        }
    }

    pub fn top_crates(&self) -> Vec<char> {
        self.stacks
            .iter()
            .filter_map(|stack| stack.get(stack.len() - 1))
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
pub struct Move {
    quantity: usize,
    source: usize,
    destination: usize,
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        lazy_static! {
            static ref MOVE_MATCHER: Regex =
                Regex::new(r"^move (\d+) from (\d+) to (\d+)$").unwrap();
        }
        if let Some(caps) = MOVE_MATCHER.captures(s) {
            let quantity = caps[1].parse::<usize>().unwrap();
            let source = caps[2].parse::<usize>().unwrap();
            let destination = caps[3].parse::<usize>().unwrap();
            Ok(Move {
                quantity,
                source,
                destination,
            })
        } else {
            let input = s.to_string();
            Err(MoveParseError { input })
        }
    }
}

#[derive(Debug)]
pub struct MoveParseError {
    input: String,
}

impl Display for MoveParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Failed to parse input: {}", self.input)
    }
}
impl Error for MoveParseError {}
