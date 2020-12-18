use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone)]
enum Tile {
    Seat{ occupied: bool },
    Floor
}

impl FromStr for Tile {
    type Err = TileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Seat{ false })
            "." => Ok(Floor)
            _ => Err(TileParseError{ string: s.clone() })
        }
}

#[derive(Debug)]
pub struct TileParseError {
    string: str
}
impl fmt::Display for TileParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Tile parse error")
    }
}
impl Error for TileParseError {}
