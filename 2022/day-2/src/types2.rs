use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn index(&self) -> u8 {
        match self {
            Move::Rock => 0,
            Move::Paper => 1,
            Move::Scissors => 2,
        }
    }

    pub fn from_index(index: u8) -> Move {
        match index % 3 {
            0 => Move::Rock,
            1 => Move::Paper,
            _ => Move::Scissors,
        }
    }

    pub fn score(&self) -> u8 {
        self.index() + 1
    }
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            _ => Err(MoveParseError {}),
        }
    }
}

#[derive(Debug)]
pub struct MoveParseError {}

impl Display for MoveParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Move parse error")
    }
}
impl Error for MoveParseError {}

#[derive(Debug)]
pub enum Outcome {
    Win,
    Lose,
    Draw,
}

impl Outcome {
    pub fn score(&self) -> u8 {
        match self {
            Outcome::Lose => 0,
            Outcome::Draw => 3,
            Outcome::Win => 6,
        }
    }
}

impl FromStr for Outcome {
    type Err = OutcomeParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "X" => Ok(Outcome::Lose),
            "Y" => Ok(Outcome::Draw),
            "Z" => Ok(Outcome::Win),
            _ => Err(OutcomeParseError {}),
        }
    }
}

#[derive(Debug)]
pub struct OutcomeParseError {}

impl Display for OutcomeParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Outcome parse error")
    }
}
impl Error for OutcomeParseError {}

#[derive(Debug)]
pub struct Round {
    pub opponent_move: Move,
    pub outcome: Outcome,
}

impl Round {
    pub fn my_move(&self) -> Move {
        match self.outcome {
            Outcome::Draw => self.opponent_move,
            Outcome::Lose => Move::from_index((self.opponent_move.index() + 2) % 3),
            Outcome::Win => Move::from_index((self.opponent_move.index() + 1) % 3),
        }
    }

    pub fn score(&self) -> u8 {
        self.my_move().score() + self.outcome.score()
    }
}

impl FromStr for Round {
    type Err = RoundParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut moves = s.split(' ');
        let opponent_move = moves
            .next()
            .ok_or(RoundParseError {})?
            .parse::<Move>()
            .map_err(|_| RoundParseError {})?;
        let outcome = moves
            .next()
            .ok_or(RoundParseError {})?
            .parse::<Outcome>()
            .map_err(|_| RoundParseError {})?;
        Ok(Round {
            opponent_move,
            outcome,
        })
    }
}

#[derive(Debug)]
pub struct RoundParseError {}

impl Display for RoundParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Round parse error")
    }
}
impl Error for RoundParseError {}
