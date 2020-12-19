use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Tile {
    Seat{ occupied: bool },
    Floor
}

impl Tile {
    pub fn occupy(&self) -> Result<Self, OnlyImplementedForSeatError> {
        match self {
            Tile::Seat{ occupied: _ } => Ok(Tile::Seat{ occupied: true }),
            _ => Err(OnlyImplementedForSeatError),
        }
    }

    pub fn vacate(&self) -> Result<Self, OnlyImplementedForSeatError> {
        match self {
            Tile::Seat{ occupied: _ } => Ok(Tile::Seat{ occupied: false }),
            _ => Err(OnlyImplementedForSeatError),
        }
    }

    pub fn is_occupied(&self) -> bool {
        match self {
            Tile::Seat{ occupied } => *occupied,
            _ => false
        }
    }

    pub fn next_frame(&self, mut neighbors: impl Iterator<Item = Tile>) -> Tile {
        match *self {
            a @ Tile::Floor => a,
            Tile::Seat{occupied} => {
                match occupied {
                    true => {
                        let num_occupied = neighbors
                            .filter(|t| t.is_occupied())
                            .count();
                        if num_occupied >= 4 {
                            self.vacate().expect("Was not seat")
                        } else {
                            *self
                        }
                    },
                    false => {
                        if neighbors.any(|t| t.is_occupied()) {
                            *self
                        } else {
                            self.occupy().expect("Was not seat")
                        }
                    },
                }
            }
        }
    }
}

impl FromStr for Tile {
    type Err = TileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Tile::Seat{ occupied: false }),
            "." => Ok(Tile::Floor),
            _ => Err( TileParseError{ string: s.to_string() }),
        }
    }
}

#[derive(Debug)]
pub struct TileParseError {
    string: String,
}
impl fmt::Display for TileParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot parse {}", self.string)
    }
}
impl Error for TileParseError {}

#[derive(Debug)]
pub struct OnlyImplementedForSeatError;
impl fmt::Display for OnlyImplementedForSeatError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "This method is only implemented for Seat")
    }
}
impl Error for OnlyImplementedForSeatError {}
