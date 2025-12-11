use aoc::laboratories;

fn main() {
    let input = include_str!("../07_laboratories/input");

    println!("part_1: {}", laboratories::part_1(input)); // 1573
    println!("part_2: {}", laboratories::part_2(input)); // 15093663987272
}