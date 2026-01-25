use std::fs;
use std::path::PathBuf;

pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn load_input(day: u8) -> std::io::Result<String> {
    let mut path = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    path.push("inputs");
    path.push(format!("day{day:02}.txt"));
    fs::read_to_string(path)
}
