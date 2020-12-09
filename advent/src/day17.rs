use std::collections::{HashMap, HashSet};

use itertools::Itertools;

type Pos = Vec<i8>;

pub fn solve(input: &str, dimensions: u8) -> usize {
    let mut cubes = HashSet::new();
    for (x, line) in input.lines().enumerate() {
        for (y, char) in line.chars().enumerate() {
            if char == '#' {
                let mut vec = vec![0; dimensions as usize];
                vec[0] = x as i8;
                vec[1] = y as i8;
                cubes.insert(vec);
            }
        }
    }

    cubes = run_simulation(cubes, 6, dimensions);
    cubes.len()
}

fn run_simulation(mut cubes: HashSet<Pos>, mut iters: usize, dimensions: u8) -> HashSet<Pos> {
    let neighbor_dirs = get_neighbors(dimensions);

    while iters > 0 {
        let mut neighbor_map = HashMap::new();
        for cube in &cubes {
            for delta in &neighbor_dirs {
                let neighbor = cube
                    .iter()
                    .zip(delta.iter())
                    .map(|(c, d)| c + d)
                    .collect_vec();
                *neighbor_map.entry(neighbor).or_insert(0) += 1;
            }
        }

        // Retain active cubes that remain active in the next step
        let mut new_cubes = HashSet::new();
        for cube in &cubes {
            let neighbors = *neighbor_map.get(cube).unwrap_or(&0);
            if neighbors == 2 || neighbors == 3 {
                new_cubes.insert(cube.clone());
            }
        }

        // Add previously inactive cubes that will be active next step
        for (activated_cube, _) in neighbor_map
            .into_iter()
            .filter(|(k, v)| *v == 3 && !cubes.contains(k))
        {
            new_cubes.insert(activated_cube);
        }

        cubes = new_cubes;
        iters -= 1;
    }
    cubes
}

fn get_neighbors(dimensions: u8) -> Vec<Pos> {
    (0..dimensions)
        .map(|_| -1..=1)
        .multi_cartesian_product()
        .filter(|vec| vec.iter().any(|val| *val != 0))
        .collect_vec()
}

#[test]
fn test_solve() {
    let input = ".#.
..#
###";
    assert_eq!(solve(input, 3), 112);
    assert_eq!(solve(input, 4), 848);
}
