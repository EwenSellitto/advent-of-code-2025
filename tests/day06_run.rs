use aoc::day06::{TEST, part1, part2};

#[test]
fn day06_part1_real_input() {
    let result = part1::solve(TEST);
    assert_eq!(result, 4277556);
}

#[test]
fn day06_part2_real_input() {
    let result = part2::solve(TEST);
    assert_eq!(result, 3263827);
}
