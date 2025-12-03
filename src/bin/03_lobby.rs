use aoc::lobby;

fn main() {
    let input = include_str!("../03_lobby/input");

    println!("part_1: {}", lobby::part_1(input));
    println!("part_2: {}", lobby::part_2(input));
}
