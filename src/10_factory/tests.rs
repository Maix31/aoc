use crate::factory;

#[test]
fn test_part_1() {
    let input = include_str!("example_input");
    let result = factory::part_1(input);
    assert_eq!(result, 7);
}

#[test]
fn test_part_2() {
    let input = include_str!("example_input");
    let result = factory::part_2(input);
    assert_eq!(result, 40);
}