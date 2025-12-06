pub static TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

mod common {
    pub fn parse(input: &str) -> Vec<(i32, i32)> {
        input.split(',').filter_map(|pair| {
            let mut splited = pair.split('-');
            let start: i32 = splited.next()?.parse().ok()?;
            let end: i32 = splited.next()?.parse().ok()?;
             Some((start, end))
        }).collect()
    }
}

pub mod part1 {
    pub fn solve(input: &str) -> i32 {
        let mut sum = 0;
        let parsed = crate::day02::common::parse(input);
        for (a, b) in parsed {
            let count = 0;
            for
        }
         -1
    }
}

pub mod part2 {
    pub fn solve(input: &str) ->i32 {
        -1
    }
}
