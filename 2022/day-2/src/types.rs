use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq)]
pub enum Move {
    Rock,
    Paper,
    Scissors,
}

impl Move {
    pub fn score(&self) -> u8 {
        match self {
            Move::Rock => 1,
            Move::Paper => 2,
            Move::Scissors => 3,
        }
    }
}

impl FromStr for Move {
    type Err = MoveParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Move::Rock),
            "B" => Ok(Move::Paper),
            "C" => Ok(Move::Scissors),
            "X" => Ok(Move::Rock),
            "Y" => Ok(Move::Paper),
            "Z" => Ok(Move::Scissors),
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

#[derive(Debug)]
pub struct Round {
    pub opponent_move: Move,
    pub my_move: Move,
}

impl Round {
    pub fn outcome(&self) -> Outcome {
        if self.opponent_move == self.my_move {
            Outcome::Draw
        } else if (self.opponent_move.score() + 1) % 3 == self.my_move.score() % 3 {
            Outcome::Win
        } else {
            Outcome::Lose
        }
    }

    pub fn score(&self) -> u8 {
        self.my_move.score() + self.outcome().score()
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
        let my_move = moves
            .next()
            .ok_or(RoundParseError {})?
            .parse::<Move>()
            .map_err(|_| RoundParseError {})?;
        Ok(Round {
            opponent_move,
            my_move,
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
