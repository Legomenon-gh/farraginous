pub fn solve(start: Vec<usize>, target: usize) -> usize {
    // Allocate at least 10 elements to support the unit tests for very small targets
    let mut history: Vec<u32> = vec![0; target.max(10)];
    for (i, num) in start.iter().enumerate() {
        history[*num] = i as u32 + 1;
    }

    let mut last_num = *start.last().unwrap();
    let mut previous_seen = 0;
    let mut turn = start.len() + 1;

    // This could be a fold but I folded instead
    while turn <= target {
        if previous_seen == 0 {
            last_num = 0;
        } else {
            last_num = turn - previous_seen - 1;
        }

        previous_seen = history[last_num] as usize;
        history[last_num] = turn as u32;
        turn += 1;
    }
    last_num
}

#[test]
fn test_part_1() {
    let vec = || vec![0, 3, 6];
    assert_eq!(solve(vec(), 4), 0);
    assert_eq!(solve(vec(), 5), 3);
    assert_eq!(solve(vec(), 6), 3);
    assert_eq!(solve(vec(), 9), 4);
    assert_eq!(solve(vec(), 2020), 436);
    assert_eq!(solve(vec![3, 1, 2], 2020), 1836);
}

#[test]
fn test_part_2() {
    assert_eq!(solve(vec![0, 3, 6], 30000000), 175594);
}
