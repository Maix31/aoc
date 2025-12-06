use crate::trash_compactor;

#[test]
fn test_find_accessible_rows_of_paper_part_1() {
    let input = include_str!("example_input");
    let result = trash_compactor::part_1(input);
    assert_eq!(result, 3);
}

#[test]
fn test_find_invalid_part_2() {
    let input = include_str!("example_input");
    let result = trash_compactor::part_2(input);
    assert_eq!(result, 14);
}