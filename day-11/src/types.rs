use std::cmp::{max, min};
use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone, Debug)]
pub enum Tile {
    Seat { occupied: bool },
    Floor,
}

impl Tile {
    pub fn occupy(&self) -> Result<Self, OnlyImplementedForSeatError> {
        match self {
            Tile::Seat { occupied: _ } => Ok(Tile::Seat { occupied: true }),
            _ => Err(OnlyImplementedForSeatError),
        }
    }

    pub fn vacate(&self) -> Result<Self, OnlyImplementedForSeatError> {
        match self {
            Tile::Seat { occupied: _ } => Ok(Tile::Seat { occupied: false }),
            _ => Err(OnlyImplementedForSeatError),
        }
    }

    pub fn is_occupied(&self) -> bool {
        match self {
            Tile::Seat { occupied } => *occupied,
            _ => false,
        }
    }

    pub fn next_frame(&self, mut neighbors: impl Iterator<Item = Tile>) -> Tile {
        match *self {
            a @ Tile::Floor => a,
            Tile::Seat { occupied } => match occupied {
                true => {
                    let num_occupied = neighbors.filter(|t| t.is_occupied()).count();
                    if num_occupied >= 4 {
                        self.vacate().expect("Was not seat")
                    } else {
                        *self
                    }
                }
                false => {
                    if neighbors.any(|t| t.is_occupied()) {
                        *self
                    } else {
                        self.occupy().expect("Was not seat")
                    }
                }
            },
        }
    }
}

impl FromStr for Tile {
    type Err = TileParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "L" => Ok(Tile::Seat { occupied: false }),
            "." => Ok(Tile::Floor),
            _ => Err(TileParseError {
                string: s.to_string(),
            }),
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

#[derive(Clone, Debug)]
pub struct SeatingChart {
    seats: Vec<Vec<Tile>>,
}

impl SeatingChart {
    pub fn from_lines(lines: impl Iterator<Item = String>) -> SeatingChart {
        let seats = lines
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_string().parse::<Tile>().ok())
                    .collect()
            })
            .collect();
        SeatingChart { seats }
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = Tile> + '_ {
        let r_min: usize = max(row as i8 - 1, 0) as usize;
        let r_max: usize = min(row + 1, self.seats.len()) as usize;

        let c_min: usize = max(col as i8 - 1, 0) as usize;
        let c_max: usize = min(col + 1, self.seats[row].len()) as usize;

        (r_min..=r_max)
            .flat_map(move |r| {
                (c_min..=c_max)
                    .filter(move |&c| r != row || c != col)
                    .map(move |c| (r, c))
            })
            .map(move |(r, c)| self.seats[r][c])
    }

    fn next_frame(&self) -> SeatingChart {
        SeatingChart {
            seats: self
                .seats
                .iter()
                .enumerate()
                .map(|(r, row)| {
                    row.iter()
                        .enumerate()
                        .map(|(c, tile)| tile.next_frame(self.neighbors(r, c)))
                        .collect()
                })
                .collect(),
        }
    }
}

impl Iterator for SeatingChart {
    type Item = SeatingChart;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.seats.clone();
        let next = self.next_frame();
        self.seats = next.seats;

        Some(SeatingChart{ seats: current })
    }
}
