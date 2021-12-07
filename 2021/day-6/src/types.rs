use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

#[derive(Debug)]
pub enum Lanternfish {
    InitialFish { days: u8 },
    BornFish { birth_day: u8 },
}

impl FromStr for Lanternfish {
    type Err = FishParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let days = s.parse::<u8>();
        InitialFish {days: days}
    }
}

#[derive(Debug)]
pub struct FishParseError {}

impl Display for CommandParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fish parse error")
    }
}
impl Error for CommandParseError {}

impl Lanternfish {
    pub fn spawn(&self, last_day: &u8) -> u32 {
        match self {
            Lanternfish::InitialFish => {
                
            },
            Lanternfish::BornFish => {

            }
        }
    }
}