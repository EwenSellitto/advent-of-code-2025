pub static TEST: &str = "L68
L30
R48
L5
R60
L55
L1
L99
R14
L82";

mod common {
    pub fn parse(input: &str) -> Vec<i32> {
        let direction = input.bytes().filter(|&b| b.is_ascii_uppercase());
        let numbers: Vec<i32> = input
            .lines()
            .map(|s| {
                let digits: String = s
                    .bytes()
                    .filter(|&b| b.is_ascii_digit())
                    .map(char::from)
                    .collect();
                digits.parse().unwrap()
            })
            .collect();
        let parsed: Vec<i32> = direction
            .zip(numbers)
            .map(|(d, n)| if d == b'L' { -n } else { n })
            .collect();
        parsed
    }
}

pub mod part1 {
    pub fn solve(input: &str) -> i32 {
        let mut count = 50;
        let mut password = 0;
        let parsed = crate::day01::common::parse(input);

        parsed.iter().for_each(|&n| {
            count += n;
            password += i32::from(count % 100 == 0);
        });
        password
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> i32 {
        let mut count = 50;
        let mut password = 0;
        let parsed = crate::day01::common::parse(input);

        parsed.iter().for_each(|&n| {
            let pos = ((count % 100) + 100) % 100;
            let steps = n.abs();

            if steps > 0 {
                let first_hit = if n > 0 {
                    if pos == 0 { 100 } else { 100 - pos }
                } else {
                    if pos == 0 { 100 } else { pos }
                };

                if first_hit <= steps {
                    password += 1 + (steps - first_hit) / 100;
                }
            }
            count += n;
        });

        password
    }
}
