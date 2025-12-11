pub static TEST: &str = "..@@.@@@@.
@@@.@.@.@@
@@@@@.@.@@
@.@@@@..@.
@@.@@@@.@@
.@@@@@@@.@
.@.@.@.@@@
@.@@@.@@@@
.@@@@@@@@.
@.@.@@@.@.";

mod common {
    pub fn parse(input: &str) -> Vec<&str> {
        input.lines().collect()
    }
}

pub mod part1 {
    const AROUND: [(i8, i8); 8] = [
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    fn check_cell(coordinates: (usize, usize), map: &[&str], size: (usize, usize)) -> i32 {
        AROUND
            .map(|(x_c, y_c)| {
                let new_x = coordinates.0 as isize + x_c as isize;
                let new_y = coordinates.1 as isize + y_c as isize;
                if new_x >= 0 && new_x < size.0 as isize && new_y >= 0 && new_y < size.1 as isize {
                    let cell = map[new_y as usize].as_bytes()[new_x as usize];
                    if cell == b'@' {
                        return 1;
                    }
                }
                0
            })
            .iter()
            .sum()
    }

    pub fn solve(input: &str) -> i32 {
        let parsed = crate::day04::common::parse(input);
        let len = parsed.len();
        let mut count = 0;
        for (y, line) in parsed.iter().enumerate() {
            let line_len = line.len();
            for (x, char) in line.as_bytes().iter().enumerate() {
                if *char == b'@' {
                    count += if check_cell((x, y), &parsed, (line_len, len)) < 4 {
                        1
                    } else {
                        0
                    };
                }
            }
        }
        count
    }
}

pub mod part2 {
    pub fn solve(input: &str) -> i32 {
        -1
    }
}
