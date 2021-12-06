use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug)]
pub enum Command {
    Forward { dist: u8 },
    Up { dist: u8 },
    Down { dist: u8 },
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
            .parse::<u8>()
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

