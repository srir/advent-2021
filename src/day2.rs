use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};

#[derive(PartialEq, Debug)]
enum Cmd {
    Forward(u32),
    Up(u32),
    Down(u32)
}

impl FromStr for Cmd {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let cmd_pair = s.trim().split(" ").collect::<Vec<&str>>();

        match cmd_pair[..] {
            ["forward", n] => Ok(Cmd::Forward(n.parse().unwrap())),
            ["up", n] => Ok(Cmd::Up(n.parse().unwrap())),
            ["down", n] => Ok(Cmd::Down(n.parse().unwrap())),
            _ => Err("unrecognized cmd")
        }
    }
}

#[aoc_generator(day2)]
fn parse_cmds(input: &str) -> Vec<Cmd> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

struct Position {
    x: u32,
    y: u32
}

impl Position {
    fn init() -> Self {
        Position{ x: 0, y: 0 }
    }
}

#[aoc(day2, part1)]
fn final_position_product(cmds: &[Cmd]) -> u32 {
    let mut pos = Position::init();

    for cmd in cmds {
        match cmd {
            Cmd::Forward(k) => pos.x += k,
            Cmd::Down(k) => pos.y += k,
            Cmd::Up(k) => pos.y -= k
        }
    }

    pos.y * pos.x
}

struct Vector {
    x: u32,
    y: u32,
    aim: u32
}

impl Vector {
    fn init() -> Self {
        Vector{ x: 0, y: 0, aim: 0 }
    }

    fn make_move(&mut self, cmd: &Cmd) {
        match cmd {
            Cmd::Up(u) => self.aim -= u,
            Cmd::Down(d) => self.aim += d,
            Cmd::Forward(f) => {
                self.x += f;
                self.y += self.aim * f;
            }
        }
    }
}

#[aoc(day2, part2)]
fn final_position_product_with_aim(cmds: &[Cmd]) -> u32 {
    let mut vec = Vector::init();

    for cmd in cmds {
        vec.make_move(cmd);
    }

    vec.x * vec.y
}