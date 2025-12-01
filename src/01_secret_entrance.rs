use std::fmt::Debug;
use std::str::FromStr;

enum Direction {
    Left,
    Right,
}

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
    let new_dial_before_modulo = match rotation.direction {
        Direction::Left => *dial - rotation.amount,
        Direction::Right => *dial + rotation.amount,
    };
    let new_dial = new_dial_before_modulo.rem_euclid(n);

    let times_passed_zero = (new_dial_before_modulo - *dial) / 100;
    // let went_from_positive_to_positive = dial.is_positive() && new_dial_before_modulo.is_negative();
    let went_from_positive_to_positive = false;
    let ended_at_zero = new_dial_before_modulo == 0;

    let times_passed_zero = times_passed_zero + if went_from_positive_to_positive { 1 } else { 0 } + if ended_at_zero { 1 } else { 0 };

    println!(
        "{} -> {} ({}) {:?}",
        *dial, new_dial, times_passed_zero, rotation
    );

    assert!(!(*dial).is_negative());
    assert!(!times_passed_zero.is_negative());

    *dial = new_dial;
    times_passed_zero
}

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
        },
        Direction::Right => {
            for _ in 0..rotation.amount {
                *dial = (*dial + 1).rem_euclid(n);
                if *dial == 0 {
                    answer = answer + 1;
                }
            }
        },
    }
    answer
}

fn main() {
    let input_str = include_str!("../input/01_secret_entrance");
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
        })
        .collect::<Vec<_>>();

    let n: i32 = 100;
    let mut dial = 50;
    let mut answer = 0;
    for rotation in input {
        // answer = answer + rotate(&mut dial, rotation, n);
        answer = answer + naive_rotate(&mut dial, rotation, n);
    }

    println!("answer {:?}", answer);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    const N: i32 = 100;

    #[test]
    fn rotate_over_n_by_amount_less_than_n() {
        assert_eq!(rotate(&mut (N - 1), Rotation { direction: Direction::Right, amount: 2 }, N), 1);
    }

    #[test]
    fn rotate_under_n_by_amount_less_than_n() {
        assert_eq!(rotate(&mut 1, Rotation { direction: Direction::Left, amount: 2 }, N), 1);
    }

    #[test]
    fn rotate_multiple_times_n() {
        assert_eq!(rotate(&mut (N / 2), Rotation { direction: Direction::Right, amount: N * 10 }, N), 10);
    }

    #[test]
    fn rotate_less_than_n_to_left() {
        assert_eq!(rotate(&mut 1, Rotation { direction: Direction::Left, amount: 1 }, N), 1);
    }

    #[test]
    fn rotate_less_than_n_to_right() {
        assert_eq!(rotate(&mut (N - 1), Rotation { direction: Direction::Right, amount: N }, N), 1);
    }

    #[test]
    fn rotate_more_than_n_to_right() {
        assert_eq!(rotate(&mut (N - 1), Rotation { direction: Direction::Right, amount: N + 1 }, N), 2);
    }

    #[test]
    fn rotate_less_than_n_but_not_over_zero() {
        assert_eq!(rotate(&mut 0, Rotation { direction: Direction::Right, amount: 1 }, N), 0);
    }
}