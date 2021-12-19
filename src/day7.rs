use aoc_runner_derive::{aoc, aoc_generator};

#[aoc_generator(day7)]
fn parse_crabs(input: &str) -> Vec<i64> {
    input.split(",").map(|t| t.parse().unwrap()).collect()
}

fn total_cost_to_align<F>(input: &[i64], position: i64, cost_func: F) -> i64 where F: Fn(i64, i64) -> i64 {
    input.iter().map(|c| cost_func(*c, position)).sum()
}

#[aoc(day7, part1)]
fn cost_to_cheapest_alignment(input: &[i64]) -> usize {
    let min_position = *input.iter().min().unwrap();
    let max_position = *input.iter().max().unwrap();

    let mut cheapest_alignment = (max_position - min_position) * (input.len() as i64);

    for pos in min_position..=max_position {
        let total_cost = total_cost_to_align(input, pos, |c, pos| (c - pos).abs());

        if total_cost < cheapest_alignment {
            cheapest_alignment = total_cost;
        }
    }

    cheapest_alignment.try_into().unwrap()
}

#[aoc(day7, part2)]
fn cost_to_cheapest_alignment_2(input: &[i64]) -> usize {
    let min_position = *input.iter().min().unwrap();
    let max_position = *input.iter().max().unwrap();

    let cost_func = |c: i64, pos: i64| {
        let diff = (c - pos).abs();

        diff * (diff + 1) / 2
    };

    let mut cheapest_alignment = cost_func(max_position, min_position) * (input.len() as i64);

    for pos in min_position..=max_position {
        let total_cost = total_cost_to_align(input, pos, cost_func);

        if total_cost < cheapest_alignment {
            cheapest_alignment = total_cost;
        }
    }

    cheapest_alignment.try_into().unwrap()
}
