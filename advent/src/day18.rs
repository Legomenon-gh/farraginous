// Rust doesn't allow redefining operators or their precedence :(
// Parsing crate supports custom precedence :)
peg::parser! {
    grammar parser() for str {
        rule num() -> usize
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule part_1() -> usize = precedence!{
            x:(@) "+" y:@ { x + y }
            x:(@) "*" y:@ { x * y }
            "(" e:part_1() ")" { e }
            n:num() { n }
        }

        pub rule part_2() -> usize = precedence!{
            x:(@) "*" y:@ { x * y }
            --
            x:(@) "+" y:@ { x + y }
            --
            "(" e:part_2() ")" { e }
            n:num() { n }
        }
    }
}

pub fn part_1(input: &str) -> usize {
    solve(input, |line| parser::part_1(&line).unwrap())
}

pub fn part_2(input: &str) -> usize {
    solve(input, |line| parser::part_2(&line).unwrap())
}

pub fn solve<F: Fn(&str) -> usize>(input: &str, parser: F) -> usize {
    let strip = |line: &str| -> String { line.chars().filter(|c| !c.is_whitespace()).collect() };

    input.lines().map(|line| parser(&strip(line))).sum()
}

#[test]
fn test_part_1() {
    let mut input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(part_1(input), 12240);

    input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(part_1(input), 13632);
}

#[test]
fn test_part_2() {
    let mut input = "5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))";
    assert_eq!(part_2(input), 669060);

    input = "((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
    assert_eq!(part_2(input), 23340);
}
