use aoc::{day01, day02, day03, day04, load_input};

fn main() -> std::io::Result<()> {
    println!("Hello Advent of code\n");

    let input_day01 = load_input(1)?;
    println!("Day 01 - Part 1: {}", day01::part1::solve(&input_day01));
    println!("Day 01 - Part 2: {}", day01::part2::solve(&input_day01));

    let input_day02 = load_input(2)?;
    println!("Day 02 - Part 1: {}", day02::part1::solve(&input_day02));
    println!("Day 02 - Part 2: {}", day02::part2::solve(&input_day02));

    let input_day03 = load_input(3)?;
    println!("Day 03 - Part 1: {}", day03::part1::solve(&input_day03));
    println!("Day 03 - Part 2: {}", day03::part2::solve(&input_day03));

    let input_day04 = load_input(4)?;
    println!("Day 04 - Part 1: {}", day04::part1::solve(&input_day04));
    println!("Day 04 - Part 2: {}", day04::part2::solve(&input_day04));

    Ok(())
}
