use itertools::Itertools;

pub fn part1(input: &str) -> isize {
    let groups = input.split_terminator("\n\n");
    groups.fold(0, |acc, group| acc + count_group(group))
}

fn count_group(group: &str) -> isize {
    let group_size = group.split_terminator("\n").collect::<Vec<_>>().len();
    let chars = group.replace("\n", "").chars().sorted().join("");
    let unique_chars = chars.chars().into_iter().unique().collect::<Vec<_>>();

    let mut total = 0;
    for char in unique_chars {
        let matcher = char.to_string().repeat(group_size);
        if chars.contains(&matcher) {
            total += 1;
        }
    }
    total
}

#[test]
fn count_all_answers() {
    let mut input = "abc";
    assert_eq!(count_group(input), 3);

    input = "a
b
c";
    assert_eq!(count_group(input), 0);

    input = "adb
caf";
    assert_eq!(count_group(input), 1);
}
