use crate::playground;

#[test]
fn test_part_1() {
    let input = include_str!("example_input");
    let result = playground::solve_k(input, 10);
    assert_eq!(result, 40);
}

#[test]
fn test_part_2() {
    let input = include_str!("example_input");
    let result = playground::part_2(input);
    assert_eq!(result, 40);
}