use aoc::day01::{TEST, part1, part2};

#[test]
fn day01_part1_real_input() {
    let result = part1::solve(TEST);
    assert_eq!(result, 3)
}

#[test]
fn day01_part2_real_input() {
    let result = part2::solve(TEST);
    assert_eq!(result, 6);
}
