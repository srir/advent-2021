use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day1)]
fn split_to_numbers(str: &str) -> Vec<u32> {
    str.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
fn count_increasing(nums: &[u32]) -> u32 {
    let mut last = None;
    let mut count = 0;

    for (_, &num) in nums.iter().enumerate() {
        if let Some(last) = last {
            if num > last {
                count += 1;
            }
        }

        last = Some(num);
    }

    count
}

#[aoc(day1, part2)]
fn count_windows(nums: &[u32]) -> u32 {
    let mut last: Option<u32> = None;
    let mut count = 0;

    for (_, window) in nums.windows(3).enumerate() {
        let sum = window.iter().sum();

        if let Some(last) = last {
            if sum > last {
                count += 1;
            }
        }

        last = Some(sum);
    }

    count
}
