#![feature(str_split_once)]

mod day20;

use std::time::Instant;
use std::{fs, path::Path};

pub fn read_file(filename: impl AsRef<Path>) -> String {
    fs::read_to_string(filename).expect("Something went wrong reading the file")
}

fn main() {
    let input = read_file("D:/D/Rust/Advent/input/day20.txt");
    let now = Instant::now();

    println!("{:?}", day20::part_1(&input));
    println!("{:?}", day20::part_2(&input));

    println!("Elapsed:{} microseconds", now.elapsed().as_micros());
}
