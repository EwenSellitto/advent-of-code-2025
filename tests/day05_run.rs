use aoc::day05::{TEST, part1, part2};

#[test]
fn day05_part1_real_input() {
    let result = part1::solve(TEST);
    assert_eq!(result, 3)
}

#[test]
fn day05_part2_real_input() {
    let result = part2::solve(TEST);
    assert_eq!(result, 14);
}
