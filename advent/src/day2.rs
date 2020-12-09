use std::{
    fs::File,
    io::{prelude::*, BufReader},
    path::Path,
};

pub fn part1() -> i32 {
    let lines = lines_from_file("D:/D/Rust/Advent/day2.txt");
    let mut total = 0;

    for line in lines {
        let first: Vec<&str> = line.split("-").collect();
        let min = first[0].parse::<usize>().unwrap();
        let rest: Vec<&str> = first[1].split(" ").collect();
        let max = rest[0].parse::<usize>().unwrap();
        let letter = rest[1].as_bytes()[0];

        let mut copy = String::from(rest[2]);
        copy.retain(|c| c == letter as char);
        let count = copy.len();

        if count >= min && count <= max {
            total += 1;
        }
    }

    total
}

pub fn part2() -> i32 {
    let lines = lines_from_file("D:/D/Rust/Advent/day2.txt");
    let mut total = 0;

    for line in lines {
        if check_line(&line) {
            total += 1;
        }
    }

    total
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}

fn check_line(line: &str) -> bool {
    let first = line.split("-").collect::<Vec<_>>();
    let idx1 = first[0].parse::<usize>().unwrap() - 1;
    let rest: Vec<&str> = first[1].split(" ").collect();
    let idx2 = rest[0].parse::<usize>().unwrap() - 1;
    let letter = rest[1].as_bytes()[0];

    let mut count = 0;
    let password = rest[2].as_bytes();

    if password[idx1] == letter {
        count += 1;
    }
    if password[idx2] == letter {
        count += 1;
    }

    count == 1
}

#[test]
fn checks_line_works() {
    let mut input = "1-3 a: abcde";
    assert!(check_line(input));

    input = "1-3 a: cbade";
    assert!(check_line(input));

    input = "1-3 b: cdefg";
    assert_eq!(false, check_line(input));

    input = "2-9 c: ccccccccc";
    assert_eq!(false, check_line(input));
}
