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

fn part_1(lines: impl Iterator<Item = String>) -> i64 {
    let instructions = read_instructions(lines);
    let program = types::Program::new(instructions);
    if let Some((state, _)) = program.take_while(|(_, instruction)| instruction.count() == &0).last() {
        state.acc
    } else {
        0
    }

}

fn part_2(lines: impl Iterator<Item = String>) -> i64 {
    let instructions = read_instructions(lines);
    let length = instructions.len();
    let modified_programs = (0..length)
        .filter_map::<Vec<types::Instruction>, _>(|index: usize| {
            let instruction = *instructions.get(index)?;
            match instruction {
                types::Instruction::Jmp{ count, value } => {
                    let mut new_instructions = instructions.clone();
                    let nop = types::Instruction::Nop { count, value };
                    new_instructions[index] = nop;
                    Some(new_instructions)
                },
                types::Instruction::Nop{ count, value } => {
                    let mut new_instructions = instructions.clone();
                    let nop = types::Instruction::Jmp { count, value };
                    new_instructions[index] = nop;
                    Some(new_instructions)
                },
                _ => None
            }
        })
        .map(|instructions| types::Program::new(instructions));
    if let Some((state, _)) = modified_programs
        .filter_map(|program| program.last())
        .find(|(state, _)| state.inst == ((length - 1) as i16)) {
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
