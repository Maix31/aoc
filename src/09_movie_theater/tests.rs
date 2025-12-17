use crate::movie_theater;

#[test]
fn test_part_1() {
    let input = include_str!("example_input");
    let result = movie_theater::part_1(input);
    assert_eq!(result, 50);
}

#[test]
fn test_part_2() {
    let input = include_str!("example_input");
    let result = movie_theater::part_2(input);
    assert_eq!(result, 40);
}