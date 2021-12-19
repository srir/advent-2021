use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum OctoState {
    EnergyLevel(usize),
    Flash
}

#[derive(Debug, Clone)]
struct OctoGrid {
    grid: Vec<Vec<OctoState>>,
    flash_count: usize,
    step_count: usize
}

impl OctoGrid {
    fn new(grid: Vec<Vec<OctoState>>) -> Self {
        OctoGrid { grid, flash_count: 0, step_count: 0 }
    }

    fn size(&self) -> usize {
        let height = self.grid.len();
        let width = self.grid.first().unwrap().len();

        height * width
    }

    fn adjacent_coordinates(width: usize, height: usize, x: usize, y: usize) -> Vec<(usize, usize)> {
        let x = x as i32;
        let y = y as i32;

        let coords = vec![
            (x-1, y-1), (x, y-1), (x+1, y-1),
            (x-1, y),  /*(x, y),*/ (x+1, y),
            (x-1, y+1), (x, y+1), (x+1, y+1),
        ];

        coords.iter()
            .filter(|&&(x,y)| 0 <= x && x < (width as i32) && 0 <= y && y < (height as i32))
            .map(|&(x, y)| (x as usize, y as usize))
            .collect()
    }

    fn maybe_flash(grid: &mut Vec<Vec<OctoState>>, x: usize, y: usize) {
        let height = grid.len();
        let width = grid.first().unwrap().len();
        let val = grid[y][x];

        let adjacent_coordinates = OctoGrid::adjacent_coordinates(width, height, x, y);

        let mut flashed_coords = vec![];

        match val {
            OctoState::EnergyLevel(e) if e > 9 => {
                // mark as flashed
                grid[y][x] = OctoState::Flash;

                // increase surrounding energy levels by 1
                for &(x1, y1) in &adjacent_coordinates {
                    let v = grid[y1][x1];
                    let value = match v {
                        OctoState::EnergyLevel(e) => {
                            if e >= 9 {
                                flashed_coords.push((x1, y1));
                            }

                            OctoState::EnergyLevel(e + 1)
                        },
                        _ => v
                    };

                    grid[y1][x1] = value;
                }
            }
            _ => {}
        }

        for (x, y) in flashed_coords {
            OctoGrid::maybe_flash(grid, x, y);
        }
    }

    fn step(&mut self) {
        let height = self.grid.len();
        let width = self.grid.first().unwrap().len();
        let mut new_grid = self.grid.clone();

        for y in 0..height {
            for x in 0..width {
                let val = new_grid[y][x];
                new_grid[y][x] = match val {
                    OctoState::EnergyLevel(e) => OctoState::EnergyLevel(e+1),
                    _ => val
                }
            }
        }

        for y in 0..height {
            for x in 0..width {
                OctoGrid::maybe_flash(&mut new_grid, x, y)
            }
        }

        let mut flash_count = 0;
        for y in 0..height {
            for x in 0..width {
                let val = new_grid[y][x];
                new_grid[y][x] = match val {
                    OctoState::Flash => {
                        flash_count += 1;
                        OctoState::EnergyLevel(0)
                    },
                    _ => val
                }
            }
        }

        self.flash_count += flash_count;
        self.step_count += 1;
        self.grid = new_grid;
    }
}

impl FromStr for OctoGrid {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(OctoGrid::new(s.lines().map(|line| {
            line.chars().filter_map(|c| {
                c.to_digit(10).map(|d| OctoState::EnergyLevel(d as usize))
            }).collect()
        }).collect()))
    }
}

#[aoc_generator(day11)]
fn parse_input(input: &str) -> OctoGrid {
    input.parse().unwrap()
}

#[aoc(day11, part1)]
fn total_flashes(input: &OctoGrid) -> usize {
    let mut grid = input.clone();

    for _ in 0..100 {
        grid.step();
    }

    grid.flash_count
}

#[aoc(day11, part2)]
fn first_synced_flash_step(input: &OctoGrid) -> usize {
    let mut grid = input.clone();

    loop {
        let old_flash_count = grid.flash_count;

        grid.step();

        if grid.flash_count - old_flash_count == grid.size() {
            break;
        }
    }

    grid.step_count
}