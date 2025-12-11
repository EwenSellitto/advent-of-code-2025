pub static TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

mod common {
    pub fn parse(input: &str) -> Vec<&str> {
        input.lines().collect()
    }
}

pub mod part1 {
    pub fn solve(string: &str) -> i32 {
        crate::day03::common::parse(string)
            .iter()
            .map(|&line| {
                let mut numbers = [0u8; 100];
                let mut max = (0, 0);
                let mut len = 0;
                for (index, char) in line.chars().enumerate() {
                    let value = char as u8 - 48;
                    numbers[index] = value;
                    max = if value > max.0 {
                        (value, index)
                    } else {
                        (max.0, max.1)
                    };
                    len += 1;
                }
                if max.1 == len - 1 {
                    return (*numbers[..len - 1].iter().max().unwrap() as i32) * 10 + max.0 as i32;
                }
                max.0 as i32 * 10 + (*numbers[max.1 + 1..].iter().max().unwrap() as i32)
            })
            .sum()
    }
}

pub mod part2 {
    const K: usize = 12;

    pub fn solve(string: &str) -> i128 {
        let mut pow10 = [0i64; K + 1];
        pow10[0] = 1;
        for i in 1..=K {
            pow10[i] = pow10[i - 1] * 10;
        }

        let mut total: i128 = 0;

        for line in string.lines() {
            let bytes = line.as_bytes();
            let n = bytes.len();
            let mut dp = [0i64; K + 1];

            for i in (0..n).rev() {
                let digit = (bytes[i] - b'0') as i64;
                let remaining = n - i;
                let max_k = remaining.min(K);
                for j in (1..=max_k).rev() {
                    let take = digit * pow10[j - 1] + dp[j - 1];
                    if remaining > j {
                        dp[j] = dp[j].max(take);
                    } else {
                        dp[j] = take;
                    }
                }
            }

            total += dp[K] as i128;
        }

        total
    }
}
