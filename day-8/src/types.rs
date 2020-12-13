use std::error::Error;
use std::fmt;
use std::str::FromStr;

#[derive(Copy, Clone)]
pub enum Instruction {
    Acc { value: i16, count: i8 },
    Jmp { value: i16, count: i8 },
    Nop { value: i16, count: i8 },
}

impl Instruction {
    fn execute(&self, state: &State) -> State {
        match self {
            Instruction::Acc { value, .. } => State {
                acc: state.acc + (*value as i64),
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
                value: *value,
            },
            Instruction::Jmp { count, value } => Instruction::Jmp {
                count: count + 1,
                value: *value,
            },
            Instruction::Nop { value, count } => Instruction::Nop {
                count: count + 1,
                value: *value,
            },
        }
    }

    pub fn count(&self) -> &i8 {
        match self {
            Instruction::Acc { count, .. } => count,
            Instruction::Jmp { count, .. } => count,
            Instruction::Nop { count, .. } => count,
        }
    }

    pub fn flip(&self) -> Option<Instruction> {
        match self {
            Instruction::Jmp { count, value } => Some(Instruction::Nop {
                count: *count,
                value: *value,
            }),
            Instruction::Nop { count, value } => Some(Instruction::Jmp {
                count: *count,
                value: *value,
            }),
            _ => None,
        }
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
            "nop" => Ok(Instruction::Nop { value, count: 0 }),
            _ => Err(InstructionParseError),
        }
    }
}

#[derive(Copy, Clone)]
pub struct State {
    pub acc: i64,
    pub inst: i16,
}

impl State {
    fn new() -> State {
        State { acc: 0, inst: 0 }
    }
}

#[derive(Debug)]
pub struct InstructionParseError;
impl fmt::Display for InstructionParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Instruction parse error")
    }
}
impl Error for InstructionParseError {}

pub struct Program {
    instructions: Vec<Instruction>,
    state: State,
}

impl Program {
    pub fn new(instructions: Vec<Instruction>) -> Program {
        let state = State::new();
        Program {
            instructions,
            state,
        }
    }
}

impl Iterator for Program {
    type Item = (State, Option<Instruction>);

    fn next(&mut self) -> Option<Self::Item> {
        let state = self.state;
        // If at the end of the program, return the final state without an instruction.
        // Increment the instruction counter one more time so we don't hit this again.
        if state.inst == (self.instructions.len() as i16) {
            let new_state = State { inst: state.inst + 1, ..state };
            self.state = new_state;
            return Some((state, None));
        }

        let instruction = *self.instructions.get(self.state.inst as usize)?;
        // Assume that if we've executed the same instruction 127 times,
        // it's infinite looping
        if *instruction.count() == 127 {
            return None;
        }
        let new_state = instruction.execute(&state);
        let new_instruction = instruction.increment_count();
        self.instructions[state.inst as usize] = new_instruction;
        self.state = new_state;
        Some((state, Some(instruction)))
    }
}
