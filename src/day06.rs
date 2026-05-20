pub static TEST: &str = "123 328  51 64
 45 64  387 23
  6 98  215 314
*   +   *   +  ";

pub mod common {
    pub type Number = u64;
    pub type Operator = fn(Number, Number) -> Number;
    pub type Operations = (Vec<Vec<Number>>, Vec<Operator>);

    pub fn to_columns<T>(rows: Vec<Vec<T>>) -> Vec<Vec<T>> {
        assert!(!rows.is_empty());
        let r = rows.len();
        let c = rows[0].len();
        assert!(rows.iter().all(|row| row.len() == c)); // ensure rectangular

        let mut cols: Vec<Vec<T>> = (0..c).map(|_| Vec::with_capacity(r)).collect();

        for row in rows {
            for (j, v) in row.into_iter().enumerate() {
                cols[j].push(v);
            }
        }
        cols
    }

    pub fn get_operator(op_str: &str) -> Operator {
        match op_str {
            "*" => |a: Number, b: Number| a * b,
            "+" => |a: Number, b: Number| a + b,
            _ => panic!("The operator is not supported and should not exists"),
        }
    }
}

pub mod part1 {
    use crate::day06::common::{Number, Operations, Operator, get_operator, to_columns};

    pub fn parse(input: &str) -> Operations {
        let mut lines: Vec<&str> = input.lines().collect();
        let last = lines.pop().unwrap();
        let line_matrix: Vec<Vec<Number>> = lines
            .iter()
            .map(|line| {
                line.split_whitespace()
                    .map(|v| v.parse::<Number>().unwrap())
                    .collect()
            })
            .collect();
        let operations: Vec<Operator> = last.split_whitespace().map(get_operator).collect();
        (to_columns(line_matrix), operations)
    }

    pub fn solve(input: &str) -> Number {
        let parsed = parse(input);
        parsed
            .0
            .iter()
            .enumerate()
            .map(|(index, value)| {
                let mut remind: Number = *value.first().unwrap();
                let values = value.iter().skip(1);
                let operator = parsed.1[index];
                values.for_each(|v| {
                    remind = operator(remind, *v);
                });
                remind
            })
            .sum()
    }
}

pub mod part2 {
    use crate::day06::common::Number;

    pub fn solve(input: &str) -> Number {
        let mut lines: Vec<&str> = input.lines().collect();
        let last = lines.pop().unwrap();
        let operations: Vec<crate::day06::common::Operator> = last
            .split_whitespace()
            .map(crate::day06::common::get_operator)
            .collect();
        let mut start = 0;
        let mut res: u64 = 0;
        last.split(['*', '+'])
            .skip(1)
            .enumerate()
            .for_each(|(index, spaces)| {
                let mut col_res: Number = 0;
                let op = operations.get(index).unwrap();
                let length = spaces.len();
                for i in start..=(start + length) {
                    let mut num = String::new();
                    lines.iter().for_each(|line| {
                        match line.chars().nth(i) {
                            Some(' ') | None => {}
                            Some(x) => num.push(x),
                        };
                    });
                    if num.is_empty() {
                        continue;
                    };
                    let value: Number = num.parse().unwrap();
                    if col_res == 0 {
                        col_res = value
                    } else {
                        col_res = op(value, col_res);
                    }
                }
                start += length + 1;
                res += col_res;
            });
        res
    }
}
