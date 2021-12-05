use std::cmp::Ordering;
use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug)]
struct Step {
    rise: i64,
    run: i64
}

#[derive(Debug)]
struct Segment {
    start: (i64, i64),
    length: u64,
    step: Step
}

fn cmp<T: Ord>(x: T, y: T) -> i64 {
    match x.cmp(&y) {
        Ordering::Less => -1,
        Ordering::Equal => 0,
        Ordering::Greater => 1
    }
}

impl FromStr for Segment {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts = s
            .split(" -> ")
            .map(|part| {
                part
                    .split(",")
                    .map(|n| n.parse().unwrap())
                    .collect::<Vec<i64>>()
            })
            .collect::<Vec<Vec<i64>>>();

        if let (&[start_x, start_y], &[end_x, end_y]) =
                (&parts.first().unwrap()[..], &parts.last().unwrap()[..]) {
            let x_diff = (end_x - start_x).abs();
            let y_diff = (end_y - start_y).abs();

            let length = std::cmp::max(x_diff, y_diff) + 1;

            let step = Step {
                rise: cmp(end_y, start_y),
                run: cmp(end_x, start_x)
            };

            Ok(Segment {
                start: (start_x, start_y),
                length: length as u64,
                step
            })
        } else {
            Err("can't match")
        }
    }
}

#[aoc_generator(day5)]
fn parse_inputs(input: &str) -> Vec<Segment> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

struct Grid(Vec<Vec<u64>>);

impl Grid {
    fn new() -> Self {
        Grid(vec![vec![0; 1000]; 1000])
    }

    fn paint(&mut self, segment: &Segment) {
        let (mut curr_x, mut curr_y) = segment.start;

        for _ in 0..segment.length {
            *self.0
                .get_mut(curr_y as usize).unwrap()
                .get_mut(curr_x as usize).unwrap() += 1;

            curr_y += segment.step.rise;
            curr_x += segment.step.run;
        }
    }

    fn paint_horiz_vert(&mut self, segments: &[Segment]) {
        for segment in segments {
            if segment.step.rise == 0 || segment.step.run == 0 {
                self.paint(segment);
            }
        }
    }

    fn paint_all(&mut self, segments: &[Segment]) {
        for segment in segments {
            self.paint(segment);
        }
    }

    fn count_overlapping(&self) -> u64 {
        self.0.iter()
            .map(|row| {
                row.iter().filter(|c| **c > 1).count() as u64
            })
            .sum()
    }
}

#[aoc(day5, part1)]
fn count_overlaps(segments: &[Segment]) -> u64 {
    let mut grid = Grid::new();

    grid.paint_horiz_vert(segments);
    grid.count_overlapping()
}

#[aoc(day5, part2)]
fn count_overlaps_2(segments: &[Segment]) -> u64 {
    let mut grid = Grid::new();

    grid.paint_all(segments);
    grid.count_overlapping()
}
