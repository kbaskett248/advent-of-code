use std::error::Error;
use std::fmt;
use std::mem::replace;
use std::str::FromStr;

use mylib::read_lines;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }
}

fn main() {
    // let p1 = part_1(read_lines("input.txt").expect("read_lines failed"));
    // println!("PART 1: {:?}", p1);
    // let p2 = part_2(read_lines("input.txt").expect("read_lines failed"));
    // println!("PART 2: {:?}", p2);
    let val = "-10".parse::<i8>().unwrap();
    println!("{}", val)
}

fn part_1(lines: impl Iterator<Item = String>) -> () {}

fn part_2(lines: impl Iterator<Item = String>) -> () {}

#[derive(Copy, Clone)]
enum Instruction {
    Acc { value: i16, count: i8 },
    Jmp { value: i16, count: i8 },
    Nop { count: i8 },
}

impl Instruction {
    fn execute(&self, state: &State) -> State {
        match self {
            Instruction::Acc { value, .. } => State {
                acc: state.acc + value,
                inst: state.inst + 1,
            },
            Instruction::Jmp { value, .. } => State {
                acc: state.acc,
                inst: state.inst + value,
            },
            Instruction::Nop { .. } => State {
                acc: state.acc,
                inst: state.inst + 1,
            },
        }
    }

    fn increment_count(&self) -> Instruction {
        match self {
            Instruction::Acc { count, value } => Instruction::Acc {
                count: count + 1,
                value: value.clone(),
            },
            Instruction::Jmp { count, value } => Instruction::Jmp {
                count: count + 1,
                value: value.clone(),
            },
            Instruction::Nop { count } => Instruction::Nop {
                count: count + 1,
            }
        }
    }
}

#[derive(Copy, Clone)]
struct State {
    acc: i16,
    inst: i16,
}

impl State {
    fn new() -> State {
        State { acc: 0, inst: 0 }
    }
}

impl FromStr for Instruction {
    type Err = InstructionParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut parts = s.split(' ');
        let ins = parts.next().ok_or(InstructionParseError)?.trim();
        let value = parts
            .next()
            .ok_or(InstructionParseError)?
            .trim()
            .parse::<i16>()
            .map_err(|_| InstructionParseError)?;
        match ins {
            "acc" => Ok(Instruction::Acc { value, count: 0 }),
            "jmp" => Ok(Instruction::Jmp { value, count: 0 }),
            "nop" => Ok(Instruction::Nop { count: 0 }),
            _ => Err(InstructionParseError),
        }
    }
}

#[derive(Debug)]
struct InstructionParseError;
impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instruction parse error")
    }
}
impl Error for InstructionParseError {}

struct Program {
    instructions: Vec<Instruction>,
    state: State,
}

impl Program {
    fn new(instructions: Vec<Instruction>) -> Program {
        let state = State::new();
        Program {
            instructions,
            state,
        }
    }
}

impl Iterator for Program {
    type Item = (State, Instruction);
    
    fn next(&mut self) -> Option<Self::Item> {
        let state = self.state;
        let instruction = self.instructions.get(self.state.inst as usize)?.clone();
        let new_state = instruction.execute(&state);
        let new_instruction = instruction.increment_count();
        self.instructions[state.inst as usize] = new_instruction;
        self.state = new_state;
        Some((state, instruction))
    }
}
