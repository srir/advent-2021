use std::collections::{HashMap, HashSet};
use aoc_runner_derive::{aoc, aoc_generator};

struct Signal {
    patterns: Vec<String>,
    output: Vec<String>
}

#[aoc_generator(day8)]
fn parse_input(input: &str) -> Vec<Signal> {
    input.lines().filter_map(|line| {
        line.split_once(" | ").map(|(pats, outs)| {
            Signal {
                patterns: pats.split_whitespace().map(|s| {
                    let mut r = s.chars().collect::<Vec<_>>();
                    r.sort();
                    r.into_iter().collect()
                }).collect(),
                output: outs.split_whitespace().map(|s| {
                    let mut r = s.chars().collect::<Vec<_>>();
                    r.sort();
                    r.into_iter().collect()
                }).collect()
            }
        })
    }).collect::<Vec<_>>()
}

#[aoc(day8, part1)]
fn count_digits_1478(signals: &[Signal]) -> usize {
    signals.iter().fold(0, |acc, row| {
        row.output.iter().map(|segment| {
            match segment.len() {
                2|3|4|7 => 1,
                _ => 0
            }
        }).sum::<usize>() + acc
    })
}

fn create_mapping(signal: &Signal) -> HashMap<String, usize> {
    let mut all_patterns: HashSet<String> = HashSet::from_iter(signal.patterns.iter().cloned());
    let mut pattern_map = HashMap::new();

    let one_pattern = all_patterns.iter().find(|p| p.len() == 2).unwrap().clone();
    let right_wires: Vec<_> = one_pattern.chars().collect();
    all_patterns.remove(&one_pattern);
    pattern_map.insert(one_pattern, 1);

    let seven_pattern = all_patterns.iter().find(|p| p.len() == 3).unwrap().clone();
    all_patterns.remove(&seven_pattern);
    pattern_map.insert(seven_pattern, 7);

    let four_pattern = all_patterns.iter().find(|p| p.len() == 4).unwrap().clone();
    all_patterns.remove(&four_pattern);
    pattern_map.insert(four_pattern, 4);

    let eight_pattern = all_patterns.iter().find(|p| p.len() == 7).unwrap().clone();
    all_patterns.remove(&eight_pattern);
    pattern_map.insert(eight_pattern, 8);

    let three_pattern = all_patterns.iter().find(|p| {
        p.len() == 5 && right_wires.iter().all(|wire| p.contains(*wire))
    }).unwrap().clone();
    all_patterns.remove(&three_pattern);

    let six_pattern = all_patterns.iter().find(|p| {
        p.len() == 6 && right_wires.iter().filter(|&&wire| p.contains(wire)).count() == 1
    }).unwrap().clone();
    let right_bottom_wire = six_pattern.chars().find(|wire| right_wires.contains(wire)).unwrap();
    all_patterns.remove(&six_pattern);
    pattern_map.insert(six_pattern, 6);

    let right_top_wire = *right_wires.iter()
        .filter(|&&w| w != right_bottom_wire)
        .next()
        .unwrap();

    let two_pattern = all_patterns.iter().find(|p| {
        p.len() == 5 && p.contains(right_top_wire)
    }).unwrap().clone();
    let left_bottom_wire = two_pattern.chars()
        .filter(|&w| {
            !three_pattern.contains(w)
        })
        .next()
        .unwrap();
    all_patterns.remove(&two_pattern);
    pattern_map.insert(two_pattern, 2);
    pattern_map.insert(three_pattern, 3);

    let five_pattern = all_patterns.iter().find(|p| {
        p.len() == 5 && p.contains(right_bottom_wire)
    }).unwrap().clone();
    all_patterns.remove(&five_pattern);
    pattern_map.insert(five_pattern, 5);

    let zero_pattern = all_patterns.iter().find(|p| {
        p.len() == 6 && p.contains(left_bottom_wire)
    }).unwrap().clone();
    all_patterns.remove(&zero_pattern);
    pattern_map.insert(zero_pattern, 0);

    let nine_pattern = all_patterns.iter().find(|p| { p.len() == 6 }).unwrap().clone();
    all_patterns.remove(&nine_pattern);
    pattern_map.insert(nine_pattern, 9);

    pattern_map
}

fn create_mapping_and_calculate(signal: &Signal) -> usize {
    let mapping = create_mapping(signal);

    let mut place_value = 1;
    let mut total = 0usize;

    for digit in signal.output.iter().rev().map(|digit| mapping.get(digit).unwrap()) {
        let value = digit * place_value;
        total += value;
        place_value *= 10;
    }

    total
}

#[aoc(day8, part2)]
fn sum_output_values(signals: &[Signal]) -> usize {
    signals.iter().map(|s| create_mapping_and_calculate(s)).sum()
}