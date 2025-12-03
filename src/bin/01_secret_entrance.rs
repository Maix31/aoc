use aoc::secret_entrance;

fn main() {
    let input = include_str!("../01_secret_entrance/input");

    println!("part_1: {}", secret_entrance::part_1(input));
    println!("part_2: {}", secret_entrance::part_2(input));
}
