use crate::laboratories;

#[test]
fn test_part_1() {
    let input = include_str!("example_input");
    let result = laboratories::part_1(input);
    assert_eq!(result, 21);
}

#[test]
fn test_part_2() {
    let input = include_str!("example_input");
    let result = laboratories::part_2(input);
    assert_eq!(result, 40);
}