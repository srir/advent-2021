use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day3)]
fn parse_bool_vecs(input: &str) -> Vec<Vec<bool>> {
    input.lines().map(|line| {
        line.chars().map(|char| match char {
            '0' => false,
            '1' => true,
            _ => panic!("unexpected char")
        }).collect()
    }).collect()
}

struct BitCounts {
    zero_counts: HashMap<usize, u32>,
    one_counts: HashMap<usize, u32>
}

fn bit_counts(input: &[Vec<bool>]) -> BitCounts {
    let mut count_zeroes: HashMap<usize, u32> = HashMap::new();
    let mut count_ones: HashMap<usize, u32> = HashMap::new();

    for row in input {
        for (i , &elem) in row.iter().enumerate() {
            let entry = if elem {
                count_ones.entry(i).or_insert(0)
            } else {
                count_zeroes.entry(i).or_insert(0)
            };

            *entry += 1;
        }
    }

    BitCounts{ zero_counts: count_zeroes, one_counts: count_ones }
}

fn shift_and_combine(v: &[u32]) -> u32 {
    let mut acc = 0;
    for (i, &v) in v.iter().rev().enumerate() {
        acc += v * (1 << i)
    }

    acc
}

fn boolvec_to_int(bv: &[bool]) -> u32 {
    shift_and_combine(&bv.iter().map(|&b| if b { 1 } else { 0 }).collect::<Vec<u32>>())
}

#[aoc(day3, part1)]
fn power_consumption(input: &[Vec<bool>]) -> u32 {
    let counts = bit_counts(input);
    let row_len = input.first().unwrap().len();

    let mut most_common: Vec<u32> = Vec::new();
    for i in 0..row_len {
        if counts.one_counts.get(&i).unwrap() > counts.zero_counts.get(&i).unwrap() {
            most_common.push(1);
        } else {
            most_common.push(0);
        }
    }

    let mut least_common: Vec<u32> = Vec::new();
    for i in 0..row_len {
        if counts.one_counts.get(&i).unwrap() > counts.zero_counts.get(&i).unwrap() {
            least_common.push(0);
        } else {
            least_common.push(1);
        }
    }

    shift_and_combine(&most_common[..]) * shift_and_combine(&least_common[..])
}

fn filter_common(most_common: bool, input: &[Vec<bool>], i: usize) -> Vec<Vec<bool>> {
    let mut count_zeroes = 0;
    let mut count_ones = 0;

    for row in input {
        if *row.get(i).unwrap() {
            count_ones += 1;
        } else {
            count_zeroes += 1;
        }
    }

    let looking_for = most_common == (count_ones >= count_zeroes);

    input.iter().filter(|&row| row.get(i).unwrap() == &looking_for).cloned().collect()
}

#[aoc(day3, part2)]
fn life_support_rating(input: &[Vec<bool>]) -> u32 {
    let row_len = input.first().unwrap().len();

    let mut i = 0;
    let mut most_common = input.to_vec();
    while i < row_len && most_common.len() > 1 {
        most_common = filter_common(true, &most_common, i);
        i += 1;
    }
    let most = boolvec_to_int(most_common.first().unwrap().as_slice());

    let mut j = 0;
    let mut least_common = input.to_vec();
    while j < row_len && least_common.len() > 1 {
        least_common = filter_common(false, &least_common, j);
        j += 1;
    }
    let least = boolvec_to_int(least_common.first().unwrap().as_slice());

    most * least
}