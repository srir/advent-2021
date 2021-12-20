use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone)]
enum Instr {
    FoldX(usize),
    FoldY(usize)
}

impl FromStr for Instr {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match &s[11..12] {
            "x" => Ok(Instr::FoldX(*&s[13..].parse::<usize>().unwrap())),
            "y" => Ok(Instr::FoldY(*&s[13..].parse::<usize>().unwrap())),
            _ => Err("couldn't parse")
        }
    }
}

#[derive(Debug, Clone)]
struct Manual {
    grid: Vec<Vec<bool>>,
    instructions: Vec<Instr>
}

#[aoc_generator(day13)]
fn parse_input(input: &str) -> Manual {
    let (coords, instrs) = input.split_once("\n\n").unwrap();

    let coords = coords.lines().map(|l| {
        let (x, y) = l.split_once(",").unwrap();
        (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
    }).collect::<Vec<_>>();

    let max_x = coords.iter().map(|&(x, _)| x).max().unwrap();
    let max_y = coords.iter().map(|&(_, y)| y).max().unwrap();

    let mut grid = vec![vec![false; max_x+1]; max_y+1];

    for (x, y) in coords {
        grid[y][x] = true;
    }

    let instructions = instrs.lines().filter_map(|l| {
        l.parse::<Instr>().ok()
    }).collect::<Vec<_>>();

    Manual { grid, instructions }
}

impl Manual {
    fn do_fold(&mut self) {
        let instr = self.instructions.remove(0);

        match instr {
            Instr::FoldY(split) => {
                let mut top = self.grid[..split].to_vec();
                let bottom = self.grid[split+1..].to_vec();

                for (y, row) in bottom.iter().rev().enumerate() {
                    for x in 0..row.len() {
                        top[y][x] |= row[x];
                    }
                }

                self.grid = top;
            },
            Instr::FoldX(split) => {
                let (mut left, right): (Vec<Vec<bool>>, Vec<Vec<bool>>) = self.grid.iter().map(|row| {
                    let (l, r) = row.split_at(split);
                    let r = &r[1..];

                    (l.to_vec(), r.to_vec())
                }).unzip();

                let right_rev = right.iter().map(|row| {
                    row.iter().rev().cloned().collect()
                }).collect::<Vec<Vec<_>>>();

                let max_x = right_rev.first().unwrap().len();

                for y in 0..left.len() {
                    for x in 0..max_x {
                        left[y][x] |= right_rev[y][x];
                    }
                }

                self.grid = left;
            }
        }
    }

    fn do_all_folds(&mut self) {
        while !self.instructions.is_empty() {
            self.do_fold()
        }
    }

    fn count_dots(&self) -> usize {
        self.grid.iter().map(|r| r.iter().filter(|&&e| e).count()).sum()
    }

    fn print(&self) {
        for row in &self.grid {
            for elem in row {
                if *elem {
                    print!("#");
                } else {
                    print!(".");
                }
            }
            println!("");
        }
    }
}

#[aoc(day13, part1)]
fn count_dots_after_first_fold(input: &Manual) -> usize {
    let mut manual = input.clone();
    manual.do_fold();
    manual.count_dots()
}

#[aoc(day13, part2)]
fn part2(input: &Manual) -> usize {
    let mut manual = input.clone();

    manual.do_all_folds();

    manual.print();

    0
}