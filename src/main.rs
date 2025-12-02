use aoc::{day01, load_input};

fn main() -> std::io::Result<()> {
    println!("Hello Advent of code\n");

    let input_day01 = load_input(1)?;
    let answer_part1 = day01::part1::solve(&input_day01);
    println!("Day 01 - Part 1: {}", answer_part1);

    let answer_part2 = day01::part2::solve(&input_day01);
    println!("Day 01 - Part 2: {}", answer_part2);

    Ok(())
}
