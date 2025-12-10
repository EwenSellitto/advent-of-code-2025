pub static TEST: &str = "987654321111111
811111111111119
234234234234278
818181911112111";

mod common {
    pub fn parse(input: &str) -> Vec<&str> {
        input.lines().collect()
    }

    pub fn parseV2(input: &str) -> Vec<Vec<u8>> {
        input
            .lines()
            .map(|line| line.chars().map(|c| (c as u8) - 48u8).collect())
            .collect()
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
    fn best_k(arr: &[u8], k: usize, memo: &mut std::collections::HashMap<(usize, usize), i128>) -> i128 {
        if k == 0 {
            return 0;
        }
        if let Some(&v) = memo.get(&(arr.as_ptr() as usize, k)) {
            return v;
        }

        let mut best = 0i128;
        for i in 0..=arr.len() - k {
            let digit_value = arr[i] as i128 * 10i128.pow((k - 1) as u32);
            let rest = best_k(&arr[i + 1..], k - 1, memo);
            best = best.max(digit_value + rest);
        }

        memo.insert((arr.as_ptr() as usize, k), best);
        best
    }

    pub fn solve(string: &str) -> i128 {
        let parsed = crate::day03::common::parseV2(string);
        parsed.into_iter().map(|v| {
            let mut memo = std::collections::HashMap::new();
            best_k(&v, 12, &mut memo)
        }).sum()
    }
}
