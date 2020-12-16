use std::iter::repeat;

use mylib::read_lines;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            14360655
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

// The first step of attacking the weakness in the XMAS data is to find the 
// first number in the list (after the preamble) which is not the sum of two 
// of the 25 numbers before it. What is the first number that does not have 
// this property?
fn part_1(lines: impl Iterator<Item = String>) -> u64 {
    let numbers: Vec<u64> = to_numbers(lines).collect();
    if let Some((num, _)) = repeat(numbers)
        .enumerate()
        .skip(25)
        .map(|(index, nums)| {
            let num = nums[index];
            let ns = nums[(index - 25)..index].to_vec();
            (num, ns)
        })
        .find(|(n, nums)| !is_sum(*n, &nums))
    {
        num.clone()
    } else {
        0
    }
}

fn is_sum(num: u64, preceding: &Vec<u64>) -> bool {
    combinations(&preceding)
        .find(|(a, b)| a + b == num)
        .is_some()
}

fn combinations(numbers: &[u64]) -> impl Iterator<Item = (u64, u64)> {
    let nums = numbers.to_vec();
    let len = nums.len() - 1;
    repeat(nums)
        .enumerate()
        .take(len)
        .map(|(index, ns)| (ns[index], ns[(index + 1)..].to_vec()))
        .flat_map(|(n, ns)| repeat(n).zip(ns))
}

fn part_2(lines: impl Iterator<Item = String>) {}

fn to_numbers(lines: impl Iterator<Item = String>) -> impl Iterator<Item = u64> {
    lines.filter_map(|line| line.parse::<u64>().ok())
}
