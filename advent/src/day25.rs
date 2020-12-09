pub fn solve((card, door): (usize, usize)) -> usize {
    let div = 20201227;
    let mut start = 1;
    let mut loop_size = 0;
    while start != door {
        start = start * 7 % div;
        loop_size += 1;
    }
    (0..loop_size).fold(1, |acc, _| acc * card % div)
}

#[test]
fn test_solve() {
    let input = (5764801, 17807724);
    assert_eq!(solve(input), 14897079);
}
