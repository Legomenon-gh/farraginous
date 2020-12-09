use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

use itertools::Itertools;

fn ints_from_file(filename: impl AsRef<Path>) -> Vec<i32> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .map(|s| s.parse::<i32>().unwrap())
        .collect()
}

pub fn sum() -> i32 {
    let ints = ints_from_file("D:/D/Rust/Advent/day1.txt");
    let ((l, m), r) = ints
        .iter()
        .cartesian_product(&ints)
        .cartesian_product(&ints)
        .find(|((l, m), r)| *l + *m + *r == 2020)
        .expect("Not found");

    l * m * r
}
