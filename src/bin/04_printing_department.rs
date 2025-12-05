use aoc::printing_department;

fn main() {
    let input = include_str!("../04_printing_department/input");

    println!("part_1: {}", printing_department::part_1(input));
    println!("part_2: {}", printing_department::part_2(input));
}
