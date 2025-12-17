use aoc::playground;

fn main() {
    let input = include_str!("../08_playground/input");

    println!("part_1: {}", playground::part_1(input)); // 175440
    println!("part_2: {}", playground::part_2(input)); // 3200955921
}
