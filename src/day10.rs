use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use Delimiter::*;

#[derive(Debug, Copy, Clone, PartialOrd, PartialEq, Eq)]
enum Delimiter {
    OpenParen,
    OpenBracket,
    OpenCurly,
    OpenAngle,
    CloseParen,
    CloseBracket,
    CloseCurly,
    CloseAngle
}

impl Delimiter {
    fn from_char(c: char) -> Option<Delimiter> {
        match c {
            '(' => Some(OpenParen),
            '[' => Some(OpenBracket),
            '{' => Some(OpenCurly),
            '<' => Some(OpenAngle),
            ')' => Some(CloseParen),
            ']' => Some(CloseBracket),
            '}' => Some(CloseCurly),
            '>' => Some(CloseAngle),
            _ => None
        }
    }

    fn matching(&self) -> Delimiter {
        match self {
            OpenParen => CloseParen,
            OpenBracket => CloseBracket,
            OpenCurly => CloseCurly,
            OpenAngle => CloseAngle,
            CloseParen => OpenParen,
            CloseBracket => OpenBracket,
            CloseCurly => OpenCurly,
            CloseAngle => OpenAngle,
        }
    }

    fn error_score(&self) -> Option<usize> {
        match self {
            OpenParen | OpenBracket | OpenCurly | OpenAngle => None,
            CloseParen => Some(3),
            CloseBracket => Some(57),
            CloseCurly => Some(1197),
            CloseAngle => Some(25137)
        }
    }

    fn autocomplete_score(&self) -> Option<usize> {
        match self {
            OpenParen | OpenBracket | OpenCurly | OpenAngle => None,
            CloseParen => Some(1),
            CloseBracket => Some(2),
            CloseCurly => Some(3),
            CloseAngle => Some(4)
        }
    }
}

#[aoc_generator(day10)]
fn parse_input(input: &str) -> Vec<Vec<Delimiter>> {
    input.lines().map(|line| {
        line.chars().filter_map(|c| Delimiter::from_char(c)).collect()
    }).collect()
}

fn corrupted_error_score(line: &[Delimiter]) -> Option<usize> {
    let mut expected_delims = vec![];

    for &delim in line {
        match delim {
            OpenParen | OpenBracket | OpenCurly | OpenAngle => {
                expected_delims.push(delim.matching());
            },
            _ => {
                let expected = expected_delims.pop();

                if expected.is_none() || expected != Some(delim) {
                    return delim.error_score()
                }
            }
        }
    }

    None
}


#[aoc(day10, part1)]
fn total_syntax_error_score(input: &[Vec<Delimiter>]) -> usize {
    input.iter().filter_map(|line| corrupted_error_score(line)).sum()
}

fn autocomplete_score(line: &[Delimiter]) -> Option<usize> {
    let mut expected_delims = vec![];

    for &delim in line {
        match delim {
            OpenParen | OpenBracket | OpenCurly | OpenAngle => {
                expected_delims.push(delim.matching());
            },
            _ => {
                expected_delims.pop();
            }
        }
    }

    let mut score = 0;
    for delim in expected_delims.iter().rev() {
        score *= 5;
        score += delim.autocomplete_score().unwrap();
    }

    Some(score)
}

#[aoc(day10, part2)]
fn middle_autocomplete_score(input: &[Vec<Delimiter>]) -> usize {
    let scores = input.iter()
        .filter(|&line| corrupted_error_score(line).is_none())
        .filter_map(|line| autocomplete_score(line))
        .sorted()
        .collect::<Vec<_>>();

    *scores.get(scores.len() / 2).unwrap()
}
