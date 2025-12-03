use aoc::gift_shop;

fn main() {
    let input = include_str!("../02_gift_shop/input");

    println!("part_1: {}", gift_shop::part_1(input));
    println!("part_2: {}", gift_shop::part_2(input));
}
