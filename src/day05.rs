pub static TEST: &str = "3-5
10-14
16-20
12-18

1
5
8
11
17
32";

mod common {
    #[derive(Debug)]
    pub struct Ingredients {
        pub ranges: Vec<(u64, u64)>,
        pub values: Vec<u64>,
    }

    pub fn parse(input: &str) -> Ingredients {
        let splited: Vec<&str> = input.split("\n\n").collect();

        let ranges: Vec<(u64, u64)> = splited
            .first()
            .unwrap()
            .lines()
            .map(|s| {
                let vs: Vec<_> = s.split("-").map(|v| v.parse().unwrap()).collect();
                (vs[0], vs[1])
            })
            .collect();
        let values: Vec<u64> = splited
            .get(1)
            .unwrap()
            .lines()
            .map(|v| v.parse().unwrap())
            .collect();

        Ingredients { ranges, values }
    }
}

pub mod part1 {
    use crate::day05::common;

    pub fn solve(input: &str) -> u64 {
        let ingredients = common::parse(input);
        ingredients
            .values
            .iter()
            .map(|value| {
                if ingredients
                    .ranges
                    .iter()
                    .any(|range| range.0 <= *value && *value <= range.1)
                {
                    return 1;
                }
                0
            })
            .sum()
    }
}

pub mod part2 {
    use crate::day05::common::parse;

    type RangeVector = Vec<(u64, u64)>;

    pub fn solve(input: &str) -> i64 {
        let mut ingredients = parse(input);
        ingredients.ranges.sort_by(|(a, _), (c, _)| a.cmp(c));

        let mut non_overlaping_ranges: RangeVector = Vec::new();
        if ingredients.ranges.is_empty() {
            return 0;
        }

        let (mut start, mut end) = ingredients.ranges[0];

        for &(next_start, next_end) in ingredients.ranges.iter().skip(1) {
            if next_start <= end {
                end = end.max(next_end);
            } else {
                non_overlaping_ranges.push((start, end));
                start = next_start;
                end = next_end;
            }
        }
        non_overlaping_ranges.push((start, end));
        non_overlaping_ranges
            .iter()
            .map(|pair| pair.1 - pair.0 + 1)
            .sum::<u64>() as i64
    }
}
