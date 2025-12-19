use crate::reactor;

#[test]
fn test_part_1() {
    let input = include_str!("example_input");
    let result = reactor::part_1(input);
    assert_eq!(result, 5);
}

#[test]
fn test_part_2() {
    let input = include_str!("example_input_2");
    let result = reactor::part_2(input);
    assert_eq!(result, 2);
}