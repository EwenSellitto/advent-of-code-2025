use aoc::{day01, day02, day03, day04, day05, day06, load_input};
use std::env;
use std::time::Instant;

fn main() -> std::io::Result<()> {
    println!("Hello Advent of code\n");

    let day = env::args().nth(1).and_then(|arg| arg.parse::<u8>().ok());

    macro_rules! solve {
        ($day:expr, $part:expr, $call:expr) => {
            if day.is_none_or(|selected_day| selected_day == $day) {
                let start = Instant::now();
                let result = $call;
                let duration = start.elapsed();
                println!(
                    "Day {:02} - Part {}: {:<25} ({:?})",
                    $day, $part, result, duration
                );
            }
        };
    }

    let input_day01 = load_input(1)?;
    solve!(1, 1, day01::part1::solve(&input_day01));
    solve!(1, 2, day01::part2::solve(&input_day01));

    let input_day02 = load_input(2)?;
    solve!(2, 1, day02::part1::solve(&input_day02));
    solve!(2, 2, day02::part2::solve(&input_day02));

    let input_day03 = load_input(3)?;
    solve!(3, 1, day03::part1::solve(&input_day03));
    solve!(3, 2, day03::part2::solve(&input_day03));

    let input_day04 = load_input(4)?;
    solve!(4, 1, day04::part1::solve(&input_day04));
    solve!(4, 2, day04::part2::solve(&input_day04));

    let input_day05 = load_input(5)?;
    solve!(5, 1, day05::part1::solve(&input_day05));
    solve!(5, 2, day05::part2::solve(&input_day05));

    let input_day06 = load_input(6)?;
    solve!(6, 1, day06::part1::solve(&input_day06));
    solve!(6, 2, day06::part2::solve(&input_day06));

    Ok(())
}
