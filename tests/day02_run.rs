use aoc::day02::{TEST, part1, part2};

#[test]
fn day01_part1_real_input() {
    let result = part1::solve(TEST);
    assert_eq!(result, 1227775554)
}

#[test]
fn day01_part2_real_input() {
    let result = part2::solve(TEST);
    assert_eq!(result, 4174379265);
}
