use crate::playground;

#[test]
fn test_part_1() {
    let input = include_str!("example_input");
    let result = playground::part_1(input);
    assert_eq!(result, 21);
}

#[test]
fn test_part_2() {
    let input = include_str!("example_input");
    let result = playground::part_2(input);
    assert_eq!(result, 40);
}