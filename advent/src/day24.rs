use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use regex::Regex;

use strum::IntoEnumIterator;
use strum_macros::EnumIter;
use strum_macros::EnumString;

#[derive(Debug, EnumIter, EnumString)]
#[strum(serialize_all = "snake_case")]
enum Dir {
    W,
    E,
    NW,
    NE,
    SW,
    SE,
}

type Pos = (isize, isize);

pub fn solve(input: &str, updates: usize) -> usize {
    let mut black_tiles = setup(input);
    for _ in 0..updates {
        let mut neighbor_map = HashMap::new();
        for black_tile in &black_tiles {
            let neighbors = Dir::iter().map(|dir| apply_dir(*black_tile, dir));
            for neighbor in neighbors {
                *neighbor_map.entry(neighbor).or_insert(0) += 1;
            }
        }

        let mut new_black_tiles = HashSet::new();
        for (neighbor, count) in neighbor_map {
            if count == 2 || (count == 1 && black_tiles.contains(&neighbor)) {
                new_black_tiles.insert(neighbor);
            }
        }
        black_tiles = new_black_tiles;
    }
    black_tiles.len()
}

fn setup(input: &str) -> HashSet<Pos> {
    let dirs = parse_input(input);
    let mut black_tiles = HashSet::new();
    for dir in dirs {
        let final_tile = dir.into_iter().fold((0, 0), apply_dir);
        if !black_tiles.insert(final_tile) {
            black_tiles.remove(&final_tile);
        }
    }
    black_tiles
}

fn apply_dir(mut pos: Pos, dir: Dir) -> Pos {
    match dir {
        Dir::W => pos.0 -= 2,
        Dir::E => pos.0 += 2,
        Dir::NW => pos = (pos.0 - 1, pos.1 - 1),
        Dir::NE => pos = (pos.0 + 1, pos.1 - 1),
        Dir::SW => pos = (pos.0 - 1, pos.1 + 1),
        Dir::SE => pos = (pos.0 + 1, pos.1 + 1),
    }
    pos
}

fn parse_input(input: &str) -> Vec<Vec<Dir>> {
    let re = Regex::new(r"nw|ne|sw|se|w|e").unwrap();
    input
        .lines()
        .map(|line| {
            re.captures_iter(line)
                .map(|cap| Dir::from_str(&cap[0]).unwrap())
                .collect_vec()
        })
        .collect_vec()
}

#[test]
fn test_solve() {
    let input = "sesenwnenenewseeswwswswwnenewsewsw
neeenesenwnwwswnenewnwwsewnenwseswesw
seswneswswsenwwnwse
nwnwneseeswswnenewneswwnewseswneseene
swweswneswnenwsewnwneneseenw
eesenwseswswnenwswnwnwsewwnwsene
sewnenenenesenwsewnenwwwse
wenwwweseeeweswwwnwwe
wsweesenenewnwwnwsenewsenwwsesesenwne
neeswseenwwswnwswswnw
nenwswwsewswnenenewsenwsenwnesesenew
enewnwewneswsewnwswenweswnenwsenwsw
sweneswneswneneenwnewenewwneswswnese
swwesenesewenwneswnwwneseswwne
enesenwswwswneneswsenwnewswseenwsese
wnwnesenesenenwwnenwsewesewsesesew
nenewswnwewswnenesenwnesewesw
eneswnwswnwsenenwnwnwwseeswneewsenese
neswnwewnwnwseenwseesewsenwsweewe
wseweeenwnesenwwwswnew";
    assert_eq!(solve(&input, 0), 10);
    assert_eq!(solve(&input, 1), 15);
    assert_eq!(solve(&input, 10), 37);
    assert_eq!(solve(&input, 100), 2208);
}
