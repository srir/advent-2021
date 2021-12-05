use std::str::FromStr;
use aoc_runner_derive::{aoc, aoc_generator};

fn transpose<T>(v: Vec<Vec<T>>) -> Vec<Vec<T>> where T: Clone {
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect::<Vec<T>>())
        .collect()
}

#[derive(Clone, Copy, Debug, PartialOrd, PartialEq)]
enum Space {
    Unmarked(u64),
    Marked(u64)
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct BingoBoard {
    spaces: Vec<Vec<Space>>
}

impl BingoBoard {
    fn sum_unmarked(&self) -> u64 {
        self.spaces.iter().map(|row| {
            row.iter().map(|space| {
                match space {
                    Space::Unmarked(n) => *n,
                    Space::Marked(_) => 0
                }
            }).sum::<u64>()
        }).sum()
    }

    fn has_won(&self) -> bool {
        let any_row_win = self.spaces.iter().any(|row| {
            row.iter().all(|space| match space {
                Space::Marked(_) => true,
                _ => false
            })
        });

        let any_col_win = transpose(self.spaces.clone()).iter().any(|col| {
            col.iter().all(|space| match space {
                Space::Marked(_) => true,
                _ => false
            })
        });

        any_col_win || any_row_win
    }

    fn call(&mut self, num: &u64) {
        let new_spaces = self.spaces.iter().map(|row| {
            row.iter()
                .map(|space| {
                    match space {
                        Space::Unmarked(u) => {
                            if u == num {
                                Space::Marked(*u)
                            } else {
                                Space::Unmarked(*u)
                            }
                        }
                        x => *x
                    }
                })
                .collect()
        }).collect();

        self.spaces = new_spaces;
    }
}

impl FromStr for BingoBoard {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let spaces = s
            .lines()
            .map(|l| l
                .split_whitespace()
                .map(|n| {
                    Space::Unmarked(n.parse().unwrap())
                })
                .collect())
            .collect::<Vec<Vec<Space>>>();

        Ok(BingoBoard { spaces })
    }
}

#[derive(Clone, Debug, PartialOrd, PartialEq)]
struct BingoGame {
    boards: Vec<BingoBoard>,
    last_called: Option<u64>,
    most_recent_winner: Option<BingoBoard>
}

impl BingoGame {
    fn winning_board(&self) -> Option<&BingoBoard> {
        self.boards.iter().find(|board| board.has_won())
    }

    fn final_score(&self) -> Option<u64> {
        match (self.winning_board(), self.last_called) {
            (Some(b), Some(l)) => Some(b.sum_unmarked() * l),
            _ => None
        }
    }

    fn last_winner_final_score(&self) -> Option<u64> {
        match (&self.most_recent_winner, self.last_called) {
            (Some(b), Some(l)) => Some(b.sum_unmarked() * l),
            _ => None
        }
    }

    fn call(&mut self, num: &u64) {
        for board in self.boards.iter_mut() {
            let had_won = board.has_won();
            board.call(num);
            let has_won = board.has_won();

            if !had_won && has_won {
                self.most_recent_winner = Some(board.clone());
                self.last_called = Some(*num);
            }
        }
    }

    fn play_game(&mut self, numbers: &[u64]) {
        for number in numbers {
            self.call(number);

            if self.winning_board().is_some() {
                break;
            }
        }
    }

    fn play_until_end(&mut self, numbers: &[u64]) {
        for number in numbers {
            self.call(number);
        }
    }
}

#[aoc_generator(day4)]
fn parse_inputs(input: &str) -> (Vec<u64>, BingoGame) {
    let sections = input.split("\n\n").collect::<Vec<&str>>();

    let nums = sections.first().cloned().unwrap()
        .split(",")
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();

    let boards = sections[1..]
        .iter()
        .map(|b| b.parse().unwrap())
        .collect::<Vec<BingoBoard>>();

    (nums, BingoGame { boards, last_called: None, most_recent_winner: None })
}


#[aoc(day4, part1)]
fn final_score(input: &(Vec<u64>, BingoGame)) -> u64 {
    let (numbers, mut game) = input.clone();

    game.play_game(&numbers);

    game.final_score().unwrap()
}

#[aoc(day4, part2)]
fn last_winner_final_score(input: &(Vec<u64>, BingoGame)) -> u64 {
    let (numbers, mut game) = input.clone();

    game.play_until_end(&numbers);

    game.last_winner_final_score().unwrap()
}