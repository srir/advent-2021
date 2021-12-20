use std::collections::HashMap;
use std::str::FromStr;
use aoc_runner_derive::aoc;
use itertools::Itertools;

#[derive(Debug, Clone)]
struct PolymerInstrs {
    template: Vec<char>,
    pair_rules: HashMap<(char, char), char>
}

impl FromStr for PolymerInstrs {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (tpl, rules) = s.split_once("\n\n").ok_or("couldn't split template")?;

        let template = tpl.chars().collect::<Vec<_>>();
        let pair_rules = rules.lines().map(|line| {
            let (pair, result) = line.split_once(" -> ").ok_or("couldn't split rule")?;

            let mut chars = pair.chars();

            Ok(
                (
                    (chars.next().ok_or("first of pair")?, chars.next().ok_or("second of pair")?),
                    result.chars().next().ok_or("result")?
                )
            )
        }).collect::<Result<Vec<((char, char),char)>, Self::Err>>();

        pair_rules.map(|rules| {
            PolymerInstrs {
                template,
                pair_rules: HashMap::from_iter(rules)
            }
        })
    }
}

impl PolymerInstrs {
    fn step(&mut self) {
        let new_entries = self.template.windows(2).map(|window| {
            match window {
                &[a, b] => {
                    self.pair_rules.get(&(a, b)).unwrap()
                },
                _ => panic!("unexpected window size")
            }
        });

        let new_data = self.template.iter()
            .interleave(new_entries)
            .cloned()
            .collect::<Vec<_>>();


        self.template = new_data;
    }

    fn occurrences(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for e in &self.template {
            *counts.entry(*e).or_insert(0) += 1;
        }

        counts
    }
}

#[aoc(day14, part1)]
fn answer_1(input: &str) -> usize {
    let mut instrs = input.parse::<PolymerInstrs>().unwrap();

    for _ in 0..10 {
        instrs.step();
    }

    let occurrences = instrs.occurrences();

    occurrences.values().max().unwrap() - occurrences.values().min().unwrap()
}

#[derive(Debug, Clone)]
struct PolymerInstrs2 {
    template: Vec<char>,
    pair_rules: HashMap<(char, char), char>,
    pair_counts: HashMap<(char, char), usize>
}

impl PolymerInstrs2 {
    fn new(input: &str) -> Self {
        let instrs = input.parse::<PolymerInstrs>().unwrap();

        let pair_counts = instrs.template.windows(2).map(|window| {
            match window {
                &[a, b] => ((a, b), 1),
                _ => panic!("unexpected window size")
            }
        }).collect::<HashMap<(char, char), usize>>();

        PolymerInstrs2 {
            template: instrs.template,
            pair_rules: instrs.pair_rules,
            pair_counts
        }
    }

    fn step(&mut self) {
        let mut new_counts = HashMap::new();

        for (&(a, b), &count) in self.pair_counts.iter() {
            let &result = self.pair_rules.get(&(a, b)).unwrap();

            *new_counts.entry((a, result)).or_insert(0) += count;
            *new_counts.entry((result, b)).or_insert(0) += count;
        }

        self.pair_counts = new_counts;
    }

    fn occurrences(&self) -> HashMap<char, usize> {
        let mut counts = HashMap::new();

        for (&(a, _), &count) in self.pair_counts.iter() {
            *counts.entry(a).or_insert(0) += count;
        }

        *counts.entry(*self.template.last().unwrap()).or_insert(0) += 1;

        counts
    }
}

#[aoc(day14, part2)]
fn answer_2(input: &str) -> usize {
    let mut instrs = PolymerInstrs2::new(input);

    for n in 0..40 {
        println!("iteration {}", n);
        instrs.step();
    }

    let occurrences = instrs.occurrences();

    occurrences.values().max().unwrap() - occurrences.values().min().unwrap()
}
