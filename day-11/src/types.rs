use std::cmp::{max, min};
use std::error::Error;
use std::fmt;
use std::fmt::Display;
use std::str::FromStr;

#[derive(Copy, Clone, Debug, Eq)]
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

    pub fn next_frame(&self, mut neighbors: impl Iterator<Item = Tile>, max_occupied: usize) -> Tile {
        match *self {
            a @ Tile::Floor => a,
            Tile::Seat { occupied } => match occupied {
                true => {
                    let num_occupied = neighbors.filter(|t| t.is_occupied()).count();
                    if num_occupied >= max_occupied {
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

    fn to_char(&self) -> char {
        match self {
            Tile::Floor => '.',
            Tile::Seat{ occupied } => {
                match occupied {
                    true => '#',
                    false => 'L',
                }
            }
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

impl PartialEq for Tile {
    fn eq(&self, other: &Self) -> bool {
        match self {
            Tile::Seat { occupied } => match other {
                Tile::Seat {
                    occupied: o_occupied,
                } => occupied == o_occupied,
                Tile::Floor => false,
            },
            Tile::Floor => matches!(other, Tile::Floor),
        }
    }
}

impl Display for Tile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
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

#[derive(Clone, Debug, Eq)]
pub struct SeatingChart {
    seats: Vec<Vec<Tile>>,
    max_occupied: usize,
    neighbors: Fn(Vec<Vec<Tile>>, usize, usize) -> impl Iterator<Item = Tile>
}

impl SeatingChart {
    pub fn from_lines(lines: impl Iterator<Item = String>, max_occupied: usize) -> SeatingChart {
        let seats = lines
            .map(|line| {
                line.chars()
                    .filter_map(|c| c.to_string().parse::<Tile>().ok())
                    .collect()
            })
            .collect();
        SeatingChart { seats, max_occupied }
    }

    fn neighbors(&self, row: usize, col: usize) -> impl Iterator<Item = Tile> + '_ {
        let r_min: usize = max(row as i8 - 1, 0) as usize;
        let r_max: usize = min(row + 1, self.seats.len() - 1) as usize;

        let c_min: usize = max(col as i8 - 1, 0) as usize;
        let c_max: usize = min(col + 1, self.seats[row].len() - 1) as usize;

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
                        .map(|(c, tile)| tile.next_frame(self.neighbors(r, c), self.max_occupied))
                        .collect()
                })
                .collect(),
            max_occupied: self.max_occupied,
        }
    }

    pub fn count_occupied(&self) -> usize {
        self.seats.iter()
            .flat_map(|row| row.iter())
            .filter(|tile| tile.is_occupied())
            .count()
    }
}

impl Iterator for SeatingChart {
    type Item = SeatingChart;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.seats.clone();
        let next = self.next_frame();
        self.seats = next.seats;

        Some(SeatingChart { seats: current, max_occupied: self.max_occupied })
    }
}

impl PartialEq for SeatingChart {
    fn eq(&self, other: &Self) -> bool {
        let a = &self.seats;
        let b = &other.seats;
        *a == *b
    }
}

impl Display for SeatingChart {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = self.seats.iter()
            .map(|row| {
                row.iter()
                    .map(|tile| tile.to_char())
                    .collect::<String>()
            })
            .collect::<Vec<String>>()
            .join("\n");
        write!(f, "{}", s)
    }
}
