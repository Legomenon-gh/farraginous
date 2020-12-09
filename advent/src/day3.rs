pub fn part2(input: &str) -> isize {
    count_trees(&input, 1, 1)
        * count_trees(&input, 3, 1)
        * count_trees(&input, 5, 1)
        * count_trees(&input, 7, 1)
        * count_trees(&input, 1, 2)
}

fn count_trees(input: &str, right_step: usize, down_step: usize) -> isize {
    let rows = input.split("\n").collect::<Vec<_>>();
    let width = rows[0].len();

    let mut x = 0;
    let mut total = 0;

    for row in rows.iter().step_by(down_step) {
        if row.as_bytes()[x] == b'#' {
            total += 1;
        }
        x = (x + right_step) % width;
    }

    total
}
#[test]
fn tree_counting() {
    let input = "..##.......
#...#...#..
.#....#..#.
..#.#...#.#
.#...##..#.
..#.##.....
.#.#.#....#
.#........#
#.##...#...
#...##....#
.#..#...#.#";
    assert_eq!(count_trees(input, 1, 1), 2);
    assert_eq!(count_trees(input, 3, 1), 7);
    assert_eq!(count_trees(input, 5, 1), 3);
    assert_eq!(count_trees(input, 7, 1), 4);
    assert_eq!(count_trees(input, 1, 2), 2);
}
