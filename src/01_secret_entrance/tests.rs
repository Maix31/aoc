use crate::secret_entrance::{rotate, Direction, Rotation};

const N: i32 = 100;
#[test]
fn rotate_over_n_by_amount_less_than_n() {
    assert_eq!(
        rotate(
            &mut (N - 1),
            Rotation {
                direction: Direction::Right,
                amount: 2
            },
            N
        ),
        1
    );
}

#[test]
fn rotate_under_n_by_amount_less_than_n() {
    assert_eq!(
        rotate(
            &mut 1,
            Rotation {
                direction: Direction::Left,
                amount: 2
            },
            N
        ),
        1
    );
}

#[test]
fn rotate_multiple_times_n() {
    assert_eq!(
        rotate(
            &mut (N / 2),
            Rotation {
                direction: Direction::Right,
                amount: N * 10
            },
            N
        ),
        10
    );
}

#[test]
fn rotate_less_than_n_to_left() {
    assert_eq!(
        rotate(
            &mut 1,
            Rotation {
                direction: Direction::Left,
                amount: 1
            },
            N
        ),
        1
    );
}

#[test]
fn rotate_less_than_n_to_right() {
    assert_eq!(
        rotate(
            &mut (N - 1),
            Rotation {
                direction: Direction::Right,
                amount: N
            },
            N
        ),
        1
    );
}

#[test]
fn rotate_more_than_n_to_right() {
    assert_eq!(
        rotate(
            &mut (N - 1),
            Rotation {
                direction: Direction::Right,
                amount: N + 1
            },
            N
        ),
        2
    );
}

#[test]
fn rotate_less_than_n_but_not_over_zero() {
    assert_eq!(
        rotate(
            &mut 0,
            Rotation {
                direction: Direction::Right,
                amount: 1
            },
            N
        ),
        0
    );
}

#[test]
fn _0() {
    assert_eq!(
        rotate(
            &mut 24,
            Rotation {
                direction: Direction::Left,
                amount: 30
            },
            N
        ),
        1
    );
}
