pub fn part_1(input: &str) -> isize {
    let lines = input.split_terminator("\n").collect::<Vec<_>>();
    let timestamp = lines[0].parse::<isize>().unwrap();
    let ids = lines[1]
        .split(",")
        .filter_map(|c| c.parse::<isize>().ok())
        .collect::<Vec<_>>();

    let min = ids
        .iter()
        .map(|id| (id, id - timestamp % id))
        .min_by(|a, b| a.1.cmp(&b.1))
        .unwrap();

    *min.0 * min.1
}

pub fn part_2(input: &str) -> isize {
    let mut total = 0;
    let mut step = 1;

    let constraints: Vec<(usize, isize)> = input
        .split(",")
        .enumerate()
        .filter_map(|(index, c)| c.parse::<isize>().ok().map(|id| (index, id)))
        .collect();

    // Boring but sufficiently fast given the small input size
    for (modulus, cofactor) in constraints {
        while (total + modulus as isize) % cofactor != 0 {
            total += step;
        }
        step *= cofactor;
    }

    total
}

#[test]
fn test_part_1() {
    let input = "939
7,13,x,x,59,x,31,19";
    assert_eq!(part_1(input), 295);
}

#[test]
fn test_part_2() {
    let mut input = "17,x,13,19";
    assert_eq!(part_2(input), 3417);

    input = "7,13,x,x,59,x,31,19";
    assert_eq!(part_2(input), 1068781);
}
