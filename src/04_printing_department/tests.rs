use crate::printing_department;

#[test]
fn test_find_accessible_rows_of_paper_part_1() {
    let input = include_str!("example_input");
    let result = printing_department::find_accessible_toilet_papers_part_1(input);
    assert_eq!(result, 13);
}

#[test]
fn test_find_invalid_part_2() {
    let input = include_str!("example_input");
    let result = printing_department::find_accessible_toilet_papers_part_2(input);
    assert_eq!(result, 43);
}