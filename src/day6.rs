use std::collections::HashMap;
use std::str::FromStr;
use aoc_runner_derive::{aoc};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
struct Lanternfish(u32);

impl FromStr for Lanternfish {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        input.parse::<u32>().map(|i| Lanternfish(i)).map_err(|_| "couldn't parse timer")
    }
}

impl Lanternfish {
    fn spawn() -> Self {
        Lanternfish(8)
    }

    fn step(&self) -> (Self, Option<Self>) {
        match self.0 {
            0 => (Self(6), Some(Self::spawn())),
            n => (Self(n - 1), None)
        }
    }

    fn step_mut(&mut self) -> Option<Self> {
        match self.0 {
            0 => {
                self.0 = 6;
                Some(Self::spawn())
            }
            _ => {
                self.0 -= 1;
                None
            }
        }
    }
}

#[derive(Debug, Clone)]
struct School(Vec<Lanternfish>);

impl School {
    fn step(&mut self) {
        let (existing, maybe_new): (Vec<Lanternfish>, Vec<Option<Lanternfish>>) = self.0.iter()
            .map(|f| f.step())
            .unzip();

        let new = maybe_new.iter().cloned().flatten().collect::<Vec<Lanternfish>>();

        self.0 = existing.iter().chain(new.iter()).cloned().collect();
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for School {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let timers = input.split(",")
            .map(|t| t.parse::<Lanternfish>())
            .collect::<Result<Vec<_>, _>>();

        match timers {
            Ok(ts) => Ok(School(ts)),
            Err(_) => Err("could not parse school")
        }
    }
}

#[aoc(day6, part1, immutable)]
fn lanternfish_count_80(input: &str) -> usize {
    let mut school = input.parse::<School>().unwrap();

    for _ in 0..80 {
        school.step();
    }

    school.count()
}

struct InPlaceSchool(Vec<Lanternfish>);

impl InPlaceSchool {
    fn step_mut(&mut self) {
        let mut new: Vec<Lanternfish> = vec![];

        for lf in self.0.iter_mut() {
            if let Some(new_fish) = lf.step_mut() {
                new.push(new_fish);
            }
        }

        self.0.append(&mut new);
    }

    fn count(&self) -> usize {
        self.0.len()
    }
}

impl FromStr for InPlaceSchool {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let timers = input.split(",")
            .map(|t| t.parse::<Lanternfish>())
            .collect::<Result<Vec<_>, _>>();

        match timers {
            Ok(ts) => Ok(InPlaceSchool(ts)),
            Err(_) => Err("could not parse school")
        }
    }
}


#[aoc(day6, part1, mutable)]
fn lanternfish_count_80_2(input: &str) -> usize {
    let mut school = input.parse::<InPlaceSchool>().unwrap();

    for _ in 0..80 {
        school.step_mut();
    }

    school.count()
}

struct CountingSchool(HashMap<Lanternfish, usize>);

impl CountingSchool {
    fn step_mut(&mut self) {
        let mut new: HashMap<Lanternfish, usize> = HashMap::new();

        for (lf, count) in self.0.iter() {
            match lf.0 {
                0 => {
                    *new.entry(Lanternfish(6)).or_insert(0) += *count;
                    *new.entry(Lanternfish::spawn()).or_insert(0) += *count;
                },
                k => {
                    *new.entry(Lanternfish(k-1)).or_insert(0) += *count;
                }
            }
        }

        self.0 = new;
    }

    fn count(&self) -> usize {
        self.0.values().sum::<usize>()
    }
}

impl FromStr for CountingSchool {
    type Err = &'static str;

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let timers = input.split(",")
            .map(|t| t.parse::<Lanternfish>())
            .collect::<Result<Vec<_>, _>>();

        match timers {
            Ok(ts) => {
                let map = ts.iter().fold(HashMap::new(), |mut acc, &lf| {
                    *acc.entry(lf).or_insert(0) += 1;
                    acc
                });

                Ok(CountingSchool(map))
            },
            Err(_) => Err("could not parse school")
        }
    }
}


#[aoc(day6, part2)]
fn lanternfish_count_256(input: &str) -> usize {
    let mut school = input.parse::<CountingSchool>().unwrap();

    for _ in 0..256 {
        school.step_mut();
    }

    school.count()
}