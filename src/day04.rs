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

    pub fn parse_vec(input: &str) -> Vec<Vec<u8>> {
        input
            .lines()
            .map(|value| value.as_bytes().to_vec())
            .collect()
    }

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

    pub enum Map<'a> {
        S(&'a [&'a str]),
        V(&'a Vec<Vec<u8>>),
    }

    fn get_cell_content(map: &Map, x: usize, y: usize) -> u8 {
        match map {
            Map::S(value) => value[y].as_bytes()[x],
            Map::V(value) => value[y][x],
        }
    }

    pub fn check_cell(coordinates: (usize, usize), map: &Map, size: (usize, usize)) -> i32 {
        AROUND
            .map(|(x_c, y_c)| {
                let new_x = coordinates.0 as isize + x_c as isize;
                let new_y = coordinates.1 as isize + y_c as isize;
                if new_x >= 0 && new_x < size.0 as isize && new_y >= 0 && new_y < size.1 as isize {
                    let cell = get_cell_content(map, new_x as usize, new_y as usize);
                    if cell == b'@' {
                        return 1;
                    }
                }
                0
            })
            .iter()
            .sum()
    }
}

pub mod part1 {

    pub fn solve(input: &str) -> i32 {
        let parsed = crate::day04::common::parse(input);
        let len = parsed.len();
        let mut count = 0;
        for (y, line) in parsed.iter().enumerate() {
            let line_len = line.len();
            for (x, char) in line.as_bytes().iter().enumerate() {
                if *char == b'@' {
                    count += if crate::day04::common::check_cell(
                        (x, y),
                        &crate::day04::common::Map::S(&parsed),
                        (line_len, len),
                    ) < 4
                    {
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
    fn clean_map(map: &mut [Vec<u8>], cleanup_coordinates: &[(usize, usize)]) {
        cleanup_coordinates
            .iter()
            .for_each(|(x, y)| map[*y][*x] = b'.');
    }

    pub fn solve(input: &str) -> i32 {
        let mut parsed = crate::day04::common::parse_vec(input);
        let len_y = parsed.len();
        let len_x = parsed[0].len();
        let mut count = 0;

        loop {
            let mut cleanup_coordinates: Vec<(usize, usize)> = Vec::new();
            for y in 0..len_y {
                for x in 0..len_x {
                    if parsed[y][x] == b'@'
                        && crate::day04::common::check_cell(
                            (x, y),
                            &crate::day04::common::Map::V(&parsed),
                            (len_x, len_y),
                        ) < 4
                    {
                        cleanup_coordinates.push((x, y));
                    }
                }
            }

            if cleanup_coordinates.is_empty() {
                break;
            }

            count += cleanup_coordinates.len() as i32;
            clean_map(&mut parsed, &cleanup_coordinates);
        }

        count
    }
}
