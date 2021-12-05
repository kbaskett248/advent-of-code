use mylib::{parse_lines, read_lines};
use std::ops::RangeInclusive;
use std::time::Instant;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        assert_eq!(
            part_1(read_lines("input.txt").expect("read_lines failed")),
            2484
        );
    }

    #[test]
    fn test_part_2() {
        assert_eq!(
            part_2(read_lines("input.txt").expect("read_lines failed")),
            15790581481472
        );
    }
}

fn main() {
    let mut start = Instant::now();
    let p1 = part_1(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 1: {:?} ({:?})", p1, start.elapsed());

    start = Instant::now();
    let p2 = part_2(read_lines("input.txt").expect("read_lines failed"));
    println!("PART 2: {:?} ({:?})", p2, start.elapsed());
}

// Find a chain that uses all of your adapters to connect the charging outlet
// to your device's built-in adapter and count the joltage differences between
// the charging outlet, the adapters, and your device. What is the number of
// 1-jolt differences multiplied by the number of 3-jolt differences?
fn part_1(lines: impl Iterator<Item = String>) -> usize {
    let mut nums: Vec<u8> = parse_lines(lines).collect();
    nums.sort_unstable();
    let (ones, threes) = nums
        .iter()
        .scan(0, |last_jolt, &n| {
            let difference = n - *last_jolt;
            *last_jolt += difference;
            Some(difference)
        })
        .partition::<Vec<u8>, _>(|&n| n == 1);
    // Add one here to account for the device, which is 3 jolts higher
    ones.len() * (threes.len() + 1)
}

// What is the total number of distinct ways you can arrange the adapters to 
// connect the charging outlet to your device?
fn part_2(lines: impl Iterator<Item = String>) -> usize {
    let mut nums: Vec<u8> = parse_lines(lines).collect();
    nums.push(0);
    nums.sort_unstable();
    nums.push((*nums.iter().last().unwrap() as u8) + 3);

    let mut cache = Vec::with_capacity(nums.len());
    cache.push(1);

    (1..nums.len()).for_each(|index| {
        let num = nums[index];

        let r: RangeInclusive<i8> = ((index as i8) - 3)..=((index as i8) - 1);

        let sum: usize = r
            .skip_while(|x| *x < 0)
            .skip_while(|&x| num - nums[x as usize] > 3)
            .map(|x| cache[x as usize])
            .sum();

        cache.push(sum);
    });
    *cache.last().expect("Cache is empty")
}
