use crate::printing_department;

#[test]
fn test_find_accessible_rows_of_paper_part_1() {
    let input = include_str!("example_input");
    // let expected_output = [
    //     "..xx.xx@x.",
    //     "x@@.@.@.@@",
    //     "@@@@@.x.@@",
    //     "@.@@@@..@.",
    //     "x@.@@@@.@x",
    //     ".@@@@@@@.@",
    //     ".@.@.@.@@@",
    //     "x.@@@.@@@@",
    //     ".@@@@@@@@.",
    //     "x.x.@@@.x.",
    // ];

    let result = printing_department::find_printing_department_part_1(input);
    assert_eq!(result, 13);
}

#[test]
fn test_find_invalid_part_2() {}
