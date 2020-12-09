use std::collections::HashMap;

pub fn solve(input: &str) -> usize {
    let mut nums = input
        .split_terminator("\n")
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    nums.sort();
    nums.insert(0, 0);
    nums.push(nums.last().unwrap() + 3);

    let mut counts = HashMap::new();
    counts.insert(0, 1);
    for i in 1..nums.len() {
        for j in i.saturating_sub(3)..i {
            if nums[i] - nums[j] <= 3 {
                *counts.entry(i).or_insert(0) += *counts.get(&j).unwrap()
            }
        }
    }

    *counts.values().max().unwrap()
}

#[test]
fn test_solve() {
    let input = "28
33
18
42
31
14
46
20
48
47
24
23
49
45
19
38
39
11
1
32
25
35
8
17
7
9
4
2
34
10
3";
    assert_eq!(solve(input), 19208);
}
