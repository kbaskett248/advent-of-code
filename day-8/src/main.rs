// https://adventofcode.com/2020/day/8
use mylib::read_lines;

mod types;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            1586
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            703
        );
    }
}

fn main() {
    let p1 = part_1(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?}", p1);
    let p2 = part_2(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 2: {:?}", p2);
}

// Run your copy of the boot code. Immediately before any instruction
// is executed a second time, what value is in the accumulator?
fn part_1(lines: impl Iterator<Item = String>) -> i64 {
    let instructions = read_instructions(lines);
    let program = types::Program::new(instructions);
    if let Some((state, _)) = program
        .take_while(|(_, op_instruction)| match op_instruction {
            Some(instruction) => instruction.count() == &0,
            None => false,
        })
        .last()
    {
        state.acc
    } else {
        0
    }
}

// Fix the program so that it terminates normally by changing exactly
// one jmp (to nop) or nop (to jmp). What is the value of the accumulator
// after the program terminates?
fn part_2(lines: impl Iterator<Item = String>) -> i64 {
    let instructions = read_instructions(lines);
    let length = instructions.len();
    let modified_programs = (0..length)
        .filter_map::<Vec<types::Instruction>, _>(|index: usize| {
            mutate_instructions(&instructions, index)
        })
        .map(types::Program::new);
    if let Some((state, _)) = modified_programs
        .filter_map(|program| program.last())
        .find(|(state, _)| state.inst == (length as i16))
    {
        state.acc
    } else {
        0
    }
}

fn read_instructions(lines: impl Iterator<Item = String>) -> Vec<types::Instruction> {
    lines
        .filter_map(|line| line.parse::<types::Instruction>().ok())
        .collect()
}

// Mutate the instructions list by flipping the instruction at the specified index.
// This returns an Option because not all instructions can be flipped.
fn mutate_instructions(
    instructions: &[types::Instruction],
    index: usize,
) -> Option<Vec<types::Instruction>> {
    let instruction = *instructions.get(index)?;
    let new_instruction = instruction.flip()?;
    let mut new_instructions = instructions.to_vec();
    new_instructions[index] = new_instruction;
    Some(new_instructions)
}
