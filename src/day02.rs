pub static TEST: &str = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

mod common {
    pub fn parse(input: &str) -> Vec<(i64, i64)> {
        input
            .trim()
            .split(',')
            .filter_map(|pair| {
                let pair = pair.trim();

                if let Some((start, end)) = pair.split_once('-') {
                    let start: i64 = start.trim().parse().ok()?;
                    let end: i64 = end.trim().parse().ok()?;
                    Some((start, end))
                } else {
                    let n: i64 = pair.parse().ok()?;
                    Some((n, n))
                }
            })
            .collect()
    }
}

pub mod part1 {
    fn evaluate(string: &str) -> bool {
        let length = string.len();
        if !length.is_multiple_of(2) {
            return false;
        }
        let half = length / 2;
        string[..half] == string[half..]
    }

    pub fn solve(input: &str) -> i64 {
        let mut sum = 0;
        let parsed = crate::day02::common::parse(input);
        for (a, b) in parsed {
            for i in a..=b {
                if evaluate(&format!("{}", i)) {
                    sum += i;
                }
            }
        }
        sum
    }
}

// FIXME : Compare as number using powers (Function is really slow)
pub mod part2 {
    fn evaluate(s: &str) -> bool {
        let chars: Vec<char> = s.chars().collect();
        let char_len = s.chars().count();
        if char_len <= 1 {
            return false;
        }
        let half = char_len / 2;

        (1..=half).any(|n| {
            if !char_len.is_multiple_of(n) {
                return false;
            }
            let pattern = &chars[..n];
            chars.chunks(n).all(|chunk| chunk == pattern)
        })
    }

    pub fn solve(input: &str) -> i64 {
        let mut sum = 0;
        let parsed = crate::day02::common::parse(input);
        for (a, b) in parsed {
            for i in a..=b {
                if evaluate(&format!("{}", i)) {
                    sum += i;
                }
            }
        }
        sum
    }
}
