use crate::gift_shop::{find_invalid_part_1, find_invalid_part_2, Range};

#[test]
fn test_find_invalid_part_1() {
    let cases: [(Range, Vec<i64>); 11] = [
        (Range { min: 11, max: 22 }, vec![11, 22]),
        (Range { min: 95, max: 115 }, vec![99]),
        (
            Range {
                min: 998,
                max: 1012,
            },
            vec![1010],
        ),
        (
            Range {
                min: 1188511880,
                max: 1188511890,
            },
            vec![1188511885],
        ),
        (
            Range {
                min: 222220,
                max: 222224,
            },
            vec![222222],
        ),
        (
            Range {
                min: 1698522,
                max: 1698528,
            },
            vec![],
        ),
        (
            Range {
                min: 446443,
                max: 446449,
            },
            vec![446446],
        ),
        (
            Range {
                min: 38593856,
                max: 38593862,
            },
            vec![38593859],
        ),
        (
            Range {
                min: 565653,
                max: 565659,
            },
            vec![],
        ),
        (
            Range {
                min: 824824821,
                max: 824824827,
            },
            vec![],
        ),
        (
            Range {
                min: 2121212118,
                max: 2121212124,
            },
            vec![],
        ),
    ];

    let mut answer = 0;
    for (input, expected_output) in cases {
        let output = find_invalid_part_1(input).collect::<Vec<i64>>();
        assert_eq!(output, expected_output);
        answer = answer + output.into_iter().sum::<i64>();
    }
    assert_eq!(answer, 1227775554);
}

#[test]
fn test_find_invalid_part_2() {
    let cases: [(Range, Vec<i64>); 11] = [
        (Range { min: 11, max: 22 }, vec![11, 22]),
        (Range { min: 95, max: 115 }, vec![99, 111]),
        (
            Range {
                min: 998,
                max: 1012,
            },
            vec![999, 1010],
        ),
        (
            Range {
                min: 1188511880,
                max: 1188511890,
            },
            vec![1188511885],
        ),
        (
            Range {
                min: 222220,
                max: 222224,
            },
            vec![222222],
        ),
        (
            Range {
                min: 1698522,
                max: 1698528,
            },
            vec![],
        ),
        (
            Range {
                min: 446443,
                max: 446449,
            },
            vec![446446],
        ),
        (
            Range {
                min: 38593856,
                max: 38593862,
            },
            vec![38593859],
        ),
        (
            Range {
                min: 565653,
                max: 565659,
            },
            vec![565656],
        ),
        (
            Range {
                min: 824824821,
                max: 824824827,
            },
            vec![824824824],
        ),
        (
            Range {
                min: 2121212118,
                max: 2121212124,
            },
            vec![2121212121],
        ),
    ];

    let mut answer = 0;
    for (input, expected_output) in cases {
        let output = find_invalid_part_2(input).collect::<Vec<i64>>();
        assert_eq!(output, expected_output);
        answer = answer + output.into_iter().sum::<i64>();
    }
    assert_eq!(answer, 4174379265);
}
