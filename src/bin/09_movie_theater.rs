use aoc::movie_theater;

fn main() {
    let input = include_str!("../09_movie_theater/input");

    println!("part_1: {}", movie_theater::part_1(input)); // 4782896435
    println!("part_2: {}", movie_theater::part_2(input)); // 1540060480
}
