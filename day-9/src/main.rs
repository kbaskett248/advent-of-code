use std::iter::{repeat, repeat_with};

use mylib::read_lines;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            0
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            ()
        );
    }

    #[test]
    fn test_combinations_2() {
        let c: Vec<(u64, u64)> = combinations(&[1, 2]).collect();
        assert_eq!(c.as_slice(), [(1, 2)]);
    }

    #[test]
    fn test_combinations_3() {
        let c: Vec<(u64, u64)> = combinations(&[1, 2, 3]).collect();
        assert_eq!(c.as_slice(), [(1, 2), (1, 3), (2, 3)]);
    }

    #[test]
    fn test_combinations_4() {
        let c: Vec<(u64, u64)> = combinations(&[1, 2, 3, 4]).collect();
        assert_eq!(
            c.as_slice(),
            [(1, 2), (1, 3), (1, 4), (2, 3), (2, 4), (3, 4)]
        );
    }
}

fn main() {
    let p1 = part_1(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?}", p1);
    let p2 = part_2(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 2: {:?}", p2);
}

fn part_1(lines: impl Iterator<Item = String>) -> u64 {
    let numbers: Vec<u64> = to_numbers(lines).collect();
    if let Some((num, _)) = repeat(numbers)
        .enumerate()
        .skip(25)
        .map(|(index, nums)| {
            let num = nums[index];
            let ns = nums[(index - 26)..(index - 1)].to_vec();
            (num, ns)
        })
        .find(|(n, nums)| is_sum(*n, &nums))
    {
        num.clone()
    } else {
        0
    }
}

fn is_sum(num: u64, preceding: &'static Vec<u64>) -> bool {
    combinations(&preceding)
        .find(|(a, b)| a + b == num)
        .is_some()
}

fn combinations<'a>(numbers: &'a [u64]) -> impl Iterator<Item = (u64, u64)> {
    repeat(numbers)
        .enumerate()
        .take(numbers.len() - 1)
        .flat_map(|(index, nums)| {
            let num = nums[index];
            nums[(index + 1)..].iter().map(move |n| (num, *n))
        })
}

fn part_2(lines: impl Iterator<Item = String>) {}

fn to_numbers(lines: impl Iterator<Item = String>) -> impl Iterator<Item = u64> {
    lines.filter_map(|line| line.parse::<u64>().ok())
}
