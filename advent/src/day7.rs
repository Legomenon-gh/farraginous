use regex::Regex;
use std::collections::HashMap;

type Contained = (isize, String);
type Rules = HashMap<String, Vec<Contained>>;

pub fn solve(input: &str) -> isize {
    let rules = create_rules(input);
    let init = (1, "shiny gold".to_string());
    bags_required(&rules, &init) - 1
}

fn bags_required(rules: &Rules, contained: &Contained) -> isize {
    let (weight, bag) = contained;
    let contents = rules.get(bag);

    if contents.is_none() {
        return *weight;
    }

    let sum_of_bags: isize = contents
        .unwrap()
        .iter()
        .map(|content| bags_required(rules, content))
        .sum();

    weight + weight * sum_of_bags
}

fn create_rules(input: &str) -> Rules {
    let start_re = Regex::new(r"^(?P<color>\w+ \w+) bags contain ").unwrap();
    let bags_re = Regex::new(r"(?P<count>\d+) (?P<color>\w+ \w+) bags?").unwrap();
    let mut rules: Rules = HashMap::new();

    for line in input.lines() {
        let start = start_re.captures(line).unwrap();
        let container = start["color"].to_string();
        let contained = bags_re
            .captures_iter(line)
            .map(|cap| (cap["count"].parse().unwrap(), cap["color"].to_string()));

        for (weight, bag) in contained {
            rules
                .entry(container.clone())
                .or_insert_with(Vec::new)
                .push((weight, bag));
        }
    }

    rules
}

#[test]
fn test_solve() {
    let input = "light red bags contain 1 bright white bag, 2 muted yellow bags.
dark orange bags contain 3 bright white bags, 4 muted yellow bags.
bright white bags contain 1 shiny gold bag.
muted yellow bags contain 2 shiny gold bags, 9 faded blue bags.
shiny gold bags contain 1 dark olive bag, 2 vibrant plum bags.
dark olive bags contain 3 faded blue bags, 4 dotted black bags.
vibrant plum bags contain 5 faded blue bags, 6 dotted black bags.
faded blue bags contain no other bags.
dotted black bags contain no other bags.";
    assert_eq!(solve(input), 32);
}
