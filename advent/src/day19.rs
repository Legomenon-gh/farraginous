use itertools::Itertools;
#[derive(Clone, Debug)]
enum Rule {
    Char(char),
    Sequence(Vec<u8>),
    Or(Vec<u8>, Vec<u8>),
}

pub fn part_1(input: &str) -> Vec<&str> {
    solve(input, std::convert::identity)
}

pub fn part_2(input: &str) -> Vec<&str> {
    let modify = |mut rules: Vec<Rule>| {
        rules[8] = Rule::Or(vec![42], vec![8, 42]);
        rules[11] = Rule::Or(vec![31, 42], vec![31, 11, 42]);
        rules
    };

    solve(input, modify)
}

fn solve<F: Fn(Vec<Rule>) -> Vec<Rule>>(input: &str, setup: F) -> Vec<&str> {
    let (rules_input, messages_input) = input.split_once("\n\n").unwrap();
    let rules = setup(parse_rules(rules_input));
    let messages = messages_input.lines().collect_vec();

    messages
        .into_iter()
        .filter(|msg| is_match(msg, &rules, rules[0..1].to_vec()))
        .collect_vec()
}

fn is_match(message: &str, all_rules: &[Rule], mut next_rules: Vec<Rule>) -> bool {
    match next_rules.pop() {
        None => message.is_empty(),
        Some(Rule::Char(x)) => {
            if message.starts_with(x) {
                is_match(&message[1..], all_rules, next_rules)
            } else {
                false
            }
        }
        Some(Rule::Sequence(vec)) => {
            let mut new_rules = vec
                .iter()
                .map(|v| all_rules[*v as usize].clone())
                .collect_vec();
            next_rules.append(&mut new_rules);
            is_match(message, all_rules, next_rules)
        }
        Some(Rule::Or(lhs, rhs)) => {
            let mut lhs_vec = next_rules.clone();
            lhs_vec.push(Rule::Sequence(lhs));
            next_rules.push(Rule::Sequence(rhs));

            is_match(message, all_rules, lhs_vec) || is_match(message, all_rules, next_rules)
        }
    }
}

fn parse_rules(input: &str) -> Vec<Rule> {
    let parse_nums = |str: &str| {
        str.split(' ')
            .map(|s| s.parse::<u8>().unwrap())
            .rev()
            .collect_vec()
    };

    let mut vec = vec![Rule::Char('!'); 140];

    for line in input.lines() {
        let (idx, op) = line.split_once(": ").unwrap();
        let index = idx.parse::<usize>().unwrap();

        if op.contains('"') {
            vec[index] = Rule::Char(op.chars().nth(1).unwrap());
        } else if op.contains('|') {
            let (lhs, rhs) = op.split_once(" | ").unwrap();
            vec[index] = Rule::Or(parse_nums(lhs), parse_nums(rhs));
        } else {
            vec[index] = Rule::Sequence(parse_nums(op));
        }
    }
    vec
}

#[test]
fn test_part_1() {
    let mut input = "0: 4 1 5
2: 4 4 | 5 5
3: 4 5 | 5 4
1: 2 3 | 3 2
4: \"a\"
5: \"b\"

ababbb
bababa
abbbab
aaabbb
aaaabbb
ababb";
    assert_eq!(part_1(input), vec!["ababbb", "abbbab"]);

    input = "0: 4 1 5
1: 2 3 | 3 2
2: 4 4 | 5 5
3: 4 5 | 5 4
4: \"a\"
5: \"b\";

aaaabb
aaabab
abbabb
abbbab
aabaab
aabbbb
abaaab
ababbb";
    assert_eq!(part_1(input).iter().count(), 8);
}

#[test]
fn test_part_2() {
    let input = "42: 9 14 | 10 1
9: 14 27 | 1 26
10: 23 14 | 28 1
1: \"a\"
11: 42 31
5: 1 14 | 15 1
19: 14 1 | 14 14
12: 24 14 | 19 1
16: 15 1 | 14 14
31: 14 17 | 1 13
6: 14 14 | 1 14
2: 1 24 | 14 4
0: 8 11
13: 14 3 | 1 12
15: 1 | 14
17: 14 2 | 1 7
23: 25 1 | 22 14
28: 16 1
4: 1 1
20: 14 14 | 1 15
3: 5 14 | 16 1
27: 1 6 | 14 18
14: \"b\"
21: 14 1 | 1 14
25: 1 1 | 1 14
22: 14 14
8: 42
26: 14 22 | 1 20
18: 15 15
7: 14 5 | 1 21
24: 14 1

abbbbbabbbaaaababbaabbbbabababbbabbbbbbabaaaa
bbabbbbaabaabba
babbbbaabbbbbabbbbbbaabaaabaaa
aaabbbbbbaaaabaababaabababbabaaabbababababaaa
bbbbbbbaaaabbbbaaabbabaaa
bbbababbbbaaaaaaaabbababaaababaabab
ababaaaaaabaaab
ababaaaaabbbaba
baabbaaaabbaaaababbaababb
abbbbabbbbaaaababbbbbbaaaababb
aaaaabbaabaaaaababaa
aaaabbaaaabbaaa
aaaabbaabbaaaaaaabbbabbbaaabbaabaaa
babaaabbbaaabaababbaabababaaab
aabbbbbaabbbaaaaaabbbbbababaaaaabbaaabba";
    assert_eq!(part_1(input).iter().count(), 3);
    assert_eq!(part_2(input).iter().count(), 12);
}
