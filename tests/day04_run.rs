use aoc::day04::{TEST, part1, part2};

#[test]
fn day04_part1_real_input() {
    let result = part1::solve(TEST);
    assert_eq!(result, 13)
}

#[test]
fn day04_part2_real_input() {
    let result = part2::solve(TEST);
    assert_eq!(result, 43);
}
