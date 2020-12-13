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
            ()
        );
    }
}

fn main() {
    let p1 = part_1(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?}", p1);
    let p2 = part_2(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 2: {:?}", p2);
}

fn part_1(lines: impl Iterator<Item = String>) -> i16 {
    let instructions: Vec<types::Instruction> = lines
        .filter_map(|line| line.parse::<types::Instruction>().ok())
        .collect();
    let program = types::Program::new(instructions);
    if let Some((state, _)) = program.take_while(|(_, instruction)| instruction.count() == &0).last() {
        state.acc
    } else {
        0
    }

}

fn part_2(lines: impl Iterator<Item = String>) {}
