use itertools::Itertools;
use std::collections::HashMap;
use aoc_runner_derive::{aoc, aoc_generator};

type Point = (usize, usize);

struct HeightMap {
    locations: Vec<Vec<usize>>,
    low_points: Vec<Point>,
    height: usize,
    width: usize
}

impl HeightMap {
    fn find_low_points(locations: &[Vec<usize>]) -> Vec<Point> {
        let height = locations.len();
        let width = locations.first().unwrap().len();

        let mut low_points = vec![];

        for (y, row) in locations.iter().enumerate() {
            for (x, loc) in row.iter().enumerate() {
                if (x == 0 || row.get(x-1).unwrap() > loc) &&
                    (x == width - 1 || row.get(x+1).unwrap() > loc) &&
                    (y == 0 || locations.get(y-1).unwrap().get(x).unwrap() > loc) &&
                    (y == height - 1 || locations.get(y+1).unwrap().get(x).unwrap() > loc) {

                    low_points.push((x, y));
                }
            }
        }

        low_points
    }

    fn new(locations: Vec<Vec<usize>>) -> Self {
        let height = locations.len();
        let width = locations.first().unwrap().len();

        let low_points = HeightMap::find_low_points(&locations);

        HeightMap {
            locations,
            low_points,
            height,
            width
        }
    }

    fn get(&self, x: usize, y: usize) -> Option<usize> {
        self.locations.get(y).map(|row| row.get(x).cloned()).flatten()
    }

    fn adjacent_points(&self, x: usize, y: usize) -> Vec<Point> {
        let mut points = vec![];

        if x != 0 {
            points.push((x-1, y));
        }

        if y != 0 {
            points.push((x, y-1));
        }

        if x != self.width - 1 {
            points.push((x+1, y));
        }

        if y != self.height - 1 {
            points.push((x, y+1));
        }

        points
    }

    fn find_basin_low_point(&self, x: usize, y: usize) -> Option<Point> {
        if let Some(9) = self.get(x, y) {
            None
        } else if self.low_points.contains(&(x, y)) {
            Some((x, y))
        } else {
            let points = self.adjacent_points(x, y);

            let lowest_adjacent = *points.iter().sorted_by_key(|&&(x,y)| {
                self.get(x, y).unwrap()
            }).next().unwrap();

            self.find_basin_low_point(lowest_adjacent.0, lowest_adjacent.1)
        }
    }
}


#[aoc_generator(day9)]
fn parse_input(input: &str) -> HeightMap {
    HeightMap::new(input.lines().map(|line| {
        line.chars().map(|c| c.to_digit(10).unwrap() as usize).collect()
    }).collect())
}

#[aoc(day9, part1)]
fn sum_of_risk_levels(height_map: &HeightMap) -> usize {
    height_map.low_points.iter().filter_map(|&(x, y)| {
        height_map.get(x, y).map(|k| k+1)
    }).sum()
}

#[aoc(day9, part2)]
fn largest_basins_product(height_map: &HeightMap) -> usize {
    let mut basin_sizes: HashMap<(usize, usize), usize> = HashMap::new();

    for y in 0..height_map.height {
        for x in 0..height_map.width {
            if let Some(low_point) = height_map.find_basin_low_point(x, y) {
                *basin_sizes.entry(low_point).or_insert(0) += 1;
            }
        }
    }

    basin_sizes.values().sorted().rev().take(3).product()
}