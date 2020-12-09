use std::{collections::HashMap, ops::Range};

type Rules = HashMap<String, (Rule, Rule)>;
type Rule = Range<usize>;

pub fn solve(input: &str) -> usize {
    // I'll file a ticket to clean this up
    let sections: Vec<Vec<&str>> = input
        .split_terminator("\n\n")
        .map(|sec| sec.split_terminator('\n').collect())
        .collect();
    let rules = parse_rules(&sections[0]);
    let tickets = parse_valid_tickets(&sections[2], &rules);
    let values = tickets_to_values(tickets);

    let my_ticket = sections[1][1]
        .split(',')
        .map(|val| val.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    let assignment = match_rules(values, rules);

    let mut prod = 1;
    for (index, field) in assignment.iter().enumerate() {
        if field.starts_with("departure") {
            prod *= my_ticket[index];
        }
    }

    prod
}

fn match_rules(mut values: Vec<Vec<usize>>, mut rules: Rules) -> Vec<String> {
    let mut fields = vec!["".into(); rules.len()];

    while !rules.is_empty() {
        let mut candidates = HashMap::new();
        for (idx, values) in values.iter().enumerate() {
            if values.is_empty() {
                continue;
            }

            for (rule_name, (r1, r2)) in &rules {
                let valid = values
                    .iter()
                    .all(|val| r1.contains(val) || r2.contains(val));
                if valid {
                    candidates
                        .entry(rule_name.clone())
                        .or_insert_with(Vec::new)
                        .push(idx);
                }
            }
        }

        for (rule_name, options) in candidates {
            if options.len() == 1 {
                fields[options[0]] = rule_name.clone();
                rules.remove(&rule_name);
                values[options[0]] = Vec::new();
            }
        }
    }

    fields
}

fn tickets_to_values(tickets: Vec<Vec<usize>>) -> Vec<Vec<usize>> {
    (0..tickets[0].len())
        .map(|index| tickets.iter().map(|tix| tix[index]).collect())
        .collect()
}

fn parse_rules(lines: &[&str]) -> Rules {
    let mut map: Rules = HashMap::new();

    let parse_rule = |rule: &str| {
        let (min, max) = rule.split_once("-").unwrap();
        min.parse::<usize>().unwrap()..max.parse::<usize>().unwrap() + 1
    };

    for line in lines {
        let (field, rules) = line.split_once(": ").unwrap();
        let (r1, r2) = rules.split_once(" or ").unwrap();
        map.insert(field.into(), (parse_rule(r1), parse_rule(r2)));
    }

    map
}

fn parse_valid_tickets(lines: &[&str], rules: &Rules) -> Vec<Vec<usize>> {
    let tickets = lines
        .iter()
        .skip(1)
        .map(|line| {
            line.split(',')
                .map(|s| s.parse::<usize>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<usize>>>();

    let all_rules = rules.values().fold(Vec::new(), |mut acc, (r1, r2)| {
        acc.push(r1);
        acc.push(r2);
        acc
    });

    let mut valid_tickets = Vec::new();
    for ticket in tickets {
        if ticket
            .iter()
            .all(|val| all_rules.iter().any(|rule| rule.contains(val)))
        {
            valid_tickets.push(ticket);
        }
    }
    valid_tickets
}
