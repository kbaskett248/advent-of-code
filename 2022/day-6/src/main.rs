use mylib::read_lines;
use std::collections::VecDeque;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1_example_1() {
        assert_eq!(part_1("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 7);
    }

    #[test]
    fn test_part_1_example_2() {
        assert_eq!(part_1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
    }

    #[test]
    fn test_part_1_example_3() {
        assert_eq!(part_1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
    }

    #[test]
    fn test_part_1_example_4() {
        assert_eq!(part_1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
    }

    #[test]
    fn test_part_1_example_5() {
        assert_eq!(part_1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn test_part_1() {
        let signal = read_lines("input.txt")
            .expect("read_lines failed")
            .next()
            .expect("no content in file");
        assert_eq!(part_1(&signal), 1093);
    }

    #[test]
    fn test_part_2_example_1() {
        assert_eq!(part_2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
    }

    #[test]
    fn test_part_2_example_2() {
        assert_eq!(part_2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
    }

    #[test]
    fn test_part_2_example_3() {
        assert_eq!(part_2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
    }

    #[test]
    fn test_part_2_example_4() {
        assert_eq!(part_2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
    }

    #[test]
    fn test_part_2_example_5() {
        assert_eq!(part_2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }

    #[test]
    fn test_part_2() {
        let signal = read_lines("input.txt")
            .expect("read_lines failed")
            .next()
            .expect("no content in file");
        assert_eq!(part_2(&signal), 3534);
    }
}

fn main() {
    let mut start = Instant::now();
    let signal = read_lines("example.txt")
        .expect("read_lines failed")
        .next()
        .expect("no content in file");
    let p1 = part_1(&signal);
    println!("PART 1: {:?} ({:?})", p1, start.elapsed());

    start = Instant::now();
    let p2 = part_2(&signal);
    println!("PART 2: {:?} ({:?})", p2, start.elapsed());
}

fn part_1(signal: &str) -> usize {
    let mut sop: VecDeque<char> = VecDeque::with_capacity(3);
    for (i, c) in signal.chars().enumerate() {
        if let Some(j) = sop.iter().position(|q| q == &c) {
            for _ in 0..(j + 1) {
                sop.pop_front();
            }
            sop.push_back(c);
        } else if sop.len() < 3 {
            sop.push_back(c);
        } else {
            return i + 1;
        }
    }
    0
}

fn part_2(signal: &str) -> usize {
    let mut sop: VecDeque<char> = VecDeque::with_capacity(13);
    for (i, c) in signal.chars().enumerate() {
        if let Some(j) = sop.iter().position(|q| q == &c) {
            for _ in 0..(j + 1) {
                sop.pop_front();
            }
            sop.push_back(c);
        } else if sop.len() < 13 {
            sop.push_back(c);
        } else {
            return i + 1;
        }
    }
    0
}
