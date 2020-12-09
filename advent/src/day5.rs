pub fn part1(input: &str) -> isize {
    let strs = input.split_terminator("\n");
    let mut ids = strs.map(|id| boarding_id(id)).collect::<Vec<_>>();

    ids.sort();
    let range = (75..865).collect::<Vec<_>>();
    *range.iter().find(|id| !ids.contains(id)).unwrap()
}

fn boarding_id(str: &str) -> isize {
    let binary = str
        .chars()
        .map(|c| match c {
            'F' => "0",
            'B' => "1",
            'L' => "0",
            'R' => "1",
            _ => "",
        })
        .collect::<String>();
    isize::from_str_radix(&binary, 2).unwrap()
}

#[test]
fn valid_boarding_id() {
    let mut str = "BFFFBBFRRR";
    assert_eq!(boarding_id(str), 567);

    str = "FFFBBBFRRR";
    assert_eq!(boarding_id(str), 119);
}
