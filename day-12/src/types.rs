use std::f32::consts::PI;
use ndarray::{arr2, Array2};
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::str::FromStr;

fn to_rotation_matrix(angle: f32) -> Array2<f32> {
    let radians = (angle as f32) / 180.0 * PI;
    arr2(&[[radians.cos(), -radians.sin()], [radians.sin(), radians.cos()]])
}

pub enum Action {
    Rotate(Array2<f32>),
    Shift(Array2<f32>),
    Forward(f32),
}

impl FromStr for Action {
    type Err = ActionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let command = &s[0..1];
        let magnitude = s[1..].parse::<f32>().map_err(|_| ActionParseError {
            string: s.to_string(),
        })?;

        match command {
            "N" => Ok(Action::Shift(arr2(&[[0.0], [magnitude]]))),
            "S" => Ok(Action::Shift(arr2(&[[0.0], [-magnitude]]))),
            "E" => Ok(Action::Shift(arr2(&[[magnitude], [0.0]]))),
            "W" => Ok(Action::Shift(arr2(&[[-magnitude], [0.0]]))),
            "L" => Ok(Action::Rotate(to_rotation_matrix(magnitude))),
            "R" => Ok(Action::Rotate(to_rotation_matrix(- magnitude))),
            "F" => Ok(Action::Forward(magnitude)),
            _ => Err(ActionParseError {
                string: s.to_string(),
            }),
        }
    }
}

#[derive(Debug)]
pub struct ActionParseError {
    string: String,
}
impl fmt::Display for ActionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Cannot parse {}", self.string)
    }
}
impl Error for ActionParseError {}

pub struct Boat {
    direction: Array2<f32>,
    location: Array2<f32>,
}

impl Boat {
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::Rotate(m) => self.direction = m * &self.direction,
            Action::Shift(s) => self.location = s * &self.direction,
            Action::Forward(m) => self.location = &self.location + &(m * &self.direction),
        }
    }
}
