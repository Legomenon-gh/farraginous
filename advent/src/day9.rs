use itertools::Itertools;

pub fn part1(numbers: &Vec<usize>, preamble: usize) -> usize {
    // Brute force is sadly sufficient here, go through all possible sums every time
    let validator = |window: &[usize]| {
        window[..preamble]
            .iter()
            .tuple_combinations()
            .all(|(a, b)| a + b != window[preamble])
    };

    numbers
        .windows(preamble + 1)
        .find(|window| validator(window))
        .map(|window| window[preamble])
        .unwrap()
}

pub fn part2(input: &str, preamble: usize) -> usize {
    let numbers = input
        .split_terminator("\n")
        .map(|l| l.parse::<usize>().unwrap())
        .collect::<Vec<_>>();
    let target = part1(&numbers, preamble);

    let mut start = 0;
    let mut end = 0;
    let mut sum = 0;

    while sum != target {
        if sum > target {
            sum -= numbers[start];
            start += 1;
        } else {
            sum += numbers[end];
            end += 1;
        }
    }

    let range = numbers[start..end].to_vec();
    range.iter().min().unwrap() + range.iter().max().unwrap()
}

#[test]
fn test_solve() {
    let input = "35
20
15
25
47
40
62
55
65
95
102
117
150
182
127
219
299
277
309
576";
    assert_eq!(part2(input, 5), 62);
}
