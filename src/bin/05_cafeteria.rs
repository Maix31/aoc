use aoc::cafeteria;

fn main() {
    let input = include_str!("../05_cafeteria/input");

    println!("part_1: {}", cafeteria::part_1(input)); // 739
    println!("part_2: {}", cafeteria::part_2(input)); // 344486348901788
}
