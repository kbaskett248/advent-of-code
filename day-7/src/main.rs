mod lib;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(lib::read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(lib::read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }
}

fn main() {
    let p1 = part_1(lib::read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?}", p1);
    let p2 = part_2(lib::read_lines("input.txt").expect("read_lines failed"));
    println!("PART 2: {:?}", p2);
}

fn part_1(lines: impl Iterator<Item = String>) -> () {
    
}

fn part_2(lines: impl Iterator<Item = String>) -> () {
    
}
