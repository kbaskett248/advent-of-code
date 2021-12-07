use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Forward { dist: u32 },
    Up { dist: u32 },
    Down { dist: u32 },
}

impl FromStr for Command {
    type Err = CommandParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let com = parts.next().ok_or(CommandParseError {})?.trim();
        let value = parts
            .next()
            .ok_or(CommandParseError {})?
            .trim()
            .parse::<u32>()
            .map_err(|_| CommandParseError {})?;
        match com {
            "forward" => Ok(Command::Forward { dist: value }),
            "down" => Ok(Command::Down { dist: value }),
            "up" => Ok(Command::Up {dist: value }),
            _ => Err(CommandParseError {}),
        }
    }
}

#[derive(Debug)]
pub struct CommandParseError {}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Command parse error")
    }
}
impl Error for CommandParseError {}

#[derive(Debug)]
pub struct State {
    depth: u32,
    distance: u32,
    aim: u32,
}

impl State {
    pub fn new() -> State {
        State {
            depth: 0,
            distance: 0,
            aim: 0,
        }
    }

    pub fn process(&mut self, command: &Command) {
        match command {
            Command::Forward { dist } => self.distance += dist,
            Command::Up { dist } => self.depth -= dist,
            Command::Down { dist } => self.depth += dist,
        };
    }

    pub fn process2(&mut self, command: &Command) {
        match command {
            Command::Forward { dist } => {
                self.distance += dist;
                self.depth += self.aim * dist
            },
            Command::Up { dist } => self.aim -= dist,
            Command::Down { dist } => self.aim += dist,
        };
    }

    pub fn product(&self) -> u32 {
        self.depth * self.distance
    }
}

