use std::collections::HashMap;
use crate::lobby;

#[test]
fn test_find_invalid_part_1() {
    assert_eq!(lobby::find_max_joltage_part_1("111111123456789"), 89);

    let cases = [
        ("987654321111111", 98),
        ("811111111111119", 89),
        ("234234234234278", 78),
        ("818181911112111", 92),
    ];

    let mut answer = 0;
    for (input, expected_output) in cases {
        let output = lobby::find_max_joltage_part_1(input);
        assert_eq!(output, expected_output);
        answer = answer + output;
    }
    assert_eq!(answer, 357);
}

#[test]
fn test_find_invalid_part_2() {
    let mut map = &mut HashMap::<(&str, u32), u64>::new();
    assert_eq!(lobby::find_max_joltage_part_2("111111123456789", 2, map), 89);
    
    let cases = [
        ("987654321111111", 98),
        ("811111111111119", 89),
        ("234234234234278", 78),
        ("818181911112111", 92),
    ];
    
    let mut answer = 0;
    for (input, expected_output) in cases {
        let output = lobby::find_max_joltage_part_2(input, 2, map);
        assert_eq!(output, expected_output);
        answer = answer + output;
    }
    assert_eq!(answer, 357);

    assert_eq!(lobby::find_max_joltage_part_2("1234", 3, map), 234);
    assert_eq!(lobby::find_max_joltage_part_2("4321", 3, map), 432);
    

    let cases = [
        ("987654321111111", 987654321111),
        ("811111111111119", 811111111119),
        ("234234234234278", 434234234278),
        ("818181911112111", 888911112111),
    ];

    let mut answer = 0;
    for (input, expected_output) in cases {
        let output = lobby::find_max_joltage_part_2(input, 12, map);
        assert_eq!(output, expected_output);
        answer = answer + output;
    }
    assert_eq!(answer, 3121910778619);
}
