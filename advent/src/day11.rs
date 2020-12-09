type Grid = Vec<Vec<char>>;
struct Solver {
    max_steps: isize,
    max_neighbors: usize,
}

const DIRS: [(isize, isize); 8] = [
    (0, 1),
    (0, -1),
    (1, 1),
    (1, -1),
    (-1, 1),
    (-1, -1),
    (1, 0),
    (-1, 0),
];

pub fn solve_part_1(input: &str) -> usize {
    solve(
        input,
        Solver {
            max_steps: 1,
            max_neighbors: 4,
        },
    )
}

pub fn solve_part_2(input: &str) -> usize {
    solve(
        input,
        Solver {
            max_steps: isize::MAX,
            max_neighbors: 5,
        },
    )
}

fn solve(input: &str, solver: Solver) -> usize {
    let mut grid: Grid = input
        .split_terminator("\n")
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect();

    loop {
        let (new_grid, has_changed) = run_step(&grid, &solver);
        if has_changed {
            grid = new_grid;
        } else {
            return new_grid
                .iter()
                .map(|row| row.iter().filter(|c| **c == '#').count())
                .sum();
        }
    }
}

fn run_step(grid: &Grid, solver: &Solver) -> (Grid, bool) {
    let mut new_grid = grid.clone();
    let mut has_changed = false;

    for row in 0..grid.len() {
        for column in 0..grid[0].len() {
            let tile = grid[row][column];
            let mut new_tile = None;

            let neighbors = count_neighbors(row as isize, column as isize, &grid, solver.max_steps);
            if tile == 'L' && neighbors == 0 {
                new_tile = Some('#');
            } else if tile == '#' && neighbors >= solver.max_neighbors {
                new_tile = Some('L');
            }

            if new_tile.is_some() {
                new_grid[row][column] = new_tile.unwrap();
                has_changed = true;
            }
        }
    }

    (new_grid, has_changed)
}

fn get_tile(x: isize, y: isize, grid: &Grid) -> Option<char> {
    let xrange = 0..grid.len() as isize;
    let yrange = 0..grid[0].len() as isize;
    if xrange.contains(&x) && yrange.contains(&y) {
        Some(grid[x as usize][y as usize])
    } else {
        None
    }
}

fn count_neighbors(x: isize, y: isize, grid: &Grid, max_steps: isize) -> usize {
    let mut sum = 0;
    for (dx, dy) in DIRS.iter() {
        let mut step = 1;
        while step <= max_steps {
            let tile = get_tile(x + (dx * step), y + (dy * step), grid);
            match tile {
                Some('#') => {
                    sum += 1;
                    break;
                }
                Some('L') => break,
                Some('.') => step += 1,
                None => break,
                _ => unreachable!(),
            }
        }
    }
    sum
}

#[test]
fn test_solve() {
    let input = "#.##.##.##
#######.##
#.#.#..#..
####.##.##
#.##.##.##
#.#####.##
..#.#.....
##########
#.######.#
#.#####.##";
    assert_eq!(solve_part_1(input), 37);
    assert_eq!(solve_part_2(input), 26);
}
