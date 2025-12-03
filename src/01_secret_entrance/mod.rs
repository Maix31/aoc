use std::fmt::Debug;
use std::str::FromStr;

#[derive(Eq, PartialEq, Copy, Clone)]
enum Direction {
    Left,
    Right,
}

#[derive(Eq, PartialEq, Copy, Clone)]
struct Rotation {
    direction: Direction,
    amount: i32,
}

impl Debug for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            (match self {
                Direction::Left => 'L',
                Direction::Right => 'R',
            })
        )
    }
}

impl Debug for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}{}", self.direction, self.amount)
    }
}

fn rotate(dial: &mut i32, rotation: Rotation, n: i32) -> i32 {
    let remainder = rotation.amount % n;

    let mut new_dial = match rotation.direction {
        Direction::Right => *dial + remainder,
        Direction::Left => *dial - remainder,
    };

    let passer_over_zero = match rotation.direction {
        Direction::Right => *dial < 100 && 100 <= new_dial,
        Direction::Left => new_dial <= 0 && *dial != 0,
    };

    if new_dial.is_negative() {
        new_dial = new_dial + n;
    }

    new_dial = new_dial % n;

    *dial = new_dial;
    rotation.amount / 100 + if passer_over_zero { 1 } else { 0 }
}

#[allow(dead_code)]
fn naive_rotate(dial: &mut i32, rotation: Rotation, n: i32) -> i32 {
    let mut answer = 0;
    match rotation.direction {
        Direction::Left => {
            for _ in 0..rotation.amount {
                *dial = (*dial - 1).rem_euclid(n);
                if *dial == 0 {
                    answer = answer + 1;
                }
            }
        }
        Direction::Right => {
            for _ in 0..rotation.amount {
                *dial = (*dial + 1).rem_euclid(n);
                if *dial == 0 {
                    answer = answer + 1;
                }
            }
        }
    }
    answer
}

fn parse_input(input_str: &str) -> impl Iterator<Item = Rotation> + '_{
    let input = input_str
        .split_whitespace()
        .map(|s| {
            let mut chars = s.chars();
            let direction = chars.next().unwrap();
            let amount = chars.as_str();

            Rotation {
                direction: match direction {
                    'L' => Direction::Left,
                    'R' => Direction::Right,
                    _ => panic!("Unknown direction: {}", direction),
                },
                amount: i32::from_str(amount).unwrap(),
            }
        });

    input
}

pub fn part_1(str: &str) -> i32 {
    let input = parse_input(str);
    let n: i32 = 100;

    let mut dial = 50;
    let mut answer = 0;
    for rotation in input {
        dial = match rotation.direction {
            Direction::Right => dial + rotation.amount,
            Direction::Left => dial - rotation.amount,
        };

        dial = dial % n;

        if dial.is_negative() {
            dial = dial + n;
        }

        if dial == 0 {
            answer = answer + 1;
        }
    }

    answer
}

pub fn part_2(str: &str) -> i32 {
    let input = parse_input(str);
    let n: i32 = 100;

    let mut dial = 50;
    let mut answer = 0;
    for rotation in input {
        answer = answer + rotate(&mut dial, rotation, n);
    }

    answer
}

#[cfg(test)]
mod tests;