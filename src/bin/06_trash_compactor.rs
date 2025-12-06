use aoc::trash_compactor;

fn main() {
    let input = include_str!("../06_trash_compactor/input");

    println!("part_1: {}", trash_compactor::part_1(input)); // 6725216329103
    println!("part_2: {}", trash_compactor::part_2(input)); // 10600728112865
}
