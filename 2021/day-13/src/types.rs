use std::collections::HashSet;
use std::error::Error;
use std::fmt;
use std::fmt::{Debug, Display};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Clone, Debug, Eq, Hash)]
pub struct Point {
    pub x: u32,
    pub y: u32,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y
    }
}

impl FromStr for Point {
    type Err = PointParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split(',');
        let x: u32 = i.next().ok_or(PointParseError {})?.parse::<u32>()?;
        let y: u32 = i.next().ok_or(PointParseError {})?.parse::<u32>()?;
        Ok(Point { x, y })
    }
}

#[derive(Debug)]
pub struct PointParseError {}

impl Display for PointParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Point parse error")
    }
}
impl Error for PointParseError {}

impl From<ParseIntError> for PointParseError {
    fn from(e: ParseIntError) -> PointParseError {
        PointParseError {}
    }
}

#[derive(Clone, Debug)]
pub enum Fold {
    VerticalFold { x: u32 },
    HorizontalFold { y: u32 },
}

impl FromStr for Fold {
    type Err = FoldParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut i = s.split('=');
        let prefix: &str = i.next().ok_or(FoldParseError {})?;
        let offset: u32 = i.next().ok_or(FoldParseError {})?.parse::<u32>()?;
        match prefix {
            "fold along x" => Ok(Fold::VerticalFold { x: offset }),
            "fold along y" => Ok(Fold::HorizontalFold { y: offset }),
            _ => Err(FoldParseError {}),
        }
    }
}

#[derive(Debug)]
pub struct FoldParseError {}

impl Display for FoldParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Fold parse error")
    }
}
impl Error for FoldParseError {}

impl From<ParseIntError> for FoldParseError {
    fn from(e: ParseIntError) -> FoldParseError {
        FoldParseError {}
    }
}

pub struct TransparentPaper {
    pub dots: HashSet<Point>,
    pub instructions: Vec<Fold>,
}

impl TransparentPaper {
    pub fn from_lines(lines: impl Iterator<Item = String>) -> TransparentPaper {
        let mut dots: HashSet<Point> = HashSet::new();
        let mut instructions: Vec<Fold> = Vec::new();

        for line in lines {
            match line.parse::<Point>() {
                Ok(point) => {
                    dots.insert(point);
                }
                Err(_) => match line.parse::<Fold>() {
                    Ok(fold) => {
                        instructions.push(fold);
                    }
                    Err(_) => (),
                },
            }
        }
        TransparentPaper { dots, instructions }
    }

    pub fn apply_fold(&self, fold: &Fold) -> TransparentPaper {
        match fold {
            Fold::VerticalFold { x } => {
                let dots: HashSet<Point> = self
                    .dots
                    .iter()
                    .filter_map(|point| {
                        if point.x > *x {
                            Some(Point {
                                x: 2 * x - point.x,
                                y: point.y,
                            })
                        } else if point.x == *x {
                            None
                        } else {
                            Some(point.clone())
                        }
                    })
                    .collect();
                TransparentPaper {
                    dots,
                    instructions: self.instructions.clone(),
                }
            }
            Fold::HorizontalFold { y } => {
                let dots: HashSet<Point> = self
                    .dots
                    .iter()
                    .filter_map(|point| {
                        if point.y > *y {
                            Some(Point {
                                x: point.x,
                                y: 2 * y - point.y,
                            })
                        } else if point.y == *y {
                            None
                        } else {
                            Some(point.clone())
                        }
                    })
                    .collect();
                TransparentPaper {
                    dots,
                    instructions: self.instructions.clone(),
                }
            }
        }
    }

    pub fn count_dots(&self) -> u32 {
        self.dots.len() as u32
    }
}
