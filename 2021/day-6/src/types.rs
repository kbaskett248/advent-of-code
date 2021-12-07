use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug)]
pub enum Lanternfish {
    InitialFish { days: u16 },
    BornFish { birth_day: u16 },
}

impl FromStr for Lanternfish {
    type Err = FishParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<u16>() {
            Ok(days) => Ok(Lanternfish::InitialFish {days: days}),
            Err(_) => Err(FishParseError {}),
        }
    }
}

#[derive(Debug)]
pub struct FishParseError {}

impl Display for FishParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fish parse error")
    }
}
impl Error for FishParseError {}

impl Lanternfish {
    pub fn spawn(&self, last_day: u16) -> u64 {
        match self {
            Lanternfish::InitialFish { days } => {
                let mut num_fish = 1;
                let mut current_day = days + 1 as u16;
                while current_day <= last_day {
                    num_fish += Lanternfish::BornFish { birth_day: current_day }.spawn(last_day);
                    current_day += 7;
                };
                num_fish
            },
            Lanternfish::BornFish { birth_day } => {
                let mut num_fish = 1;
                let mut current_day = birth_day + 9 as u16;
                while current_day <= last_day {
                    num_fish += Lanternfish::BornFish { birth_day: current_day }.spawn(last_day);
                    current_day += 7;
                }
                num_fish
            }
        }
    }
}