use num_complex::Complex;

pub fn solve(input: &str) -> isize {
    let lines = input.split_terminator("\n").collect::<Vec<_>>();

    let mut waypoint = Complex::new(10, 1);
    let mut pos = Complex::new(0, 0);

    for line in lines {
        let dir = line.chars().nth(0).unwrap();
        let arg = line[1..].parse::<isize>().unwrap();

        match dir {
            'N' => waypoint.im += arg,
            'S' => waypoint.im -= arg,
            'E' => waypoint += arg,
            'W' => waypoint -= arg,
            'R' => {
                for _ in 0..arg / 90 {
                    waypoint *= Complex::new(0, -1);
                }
            }
            'L' => {
                for _ in 0..arg / 90 {
                    waypoint *= Complex::new(0, 1);
                }
            }
            'F' => pos += waypoint * arg,
            _ => unreachable!(),
        }
    }
    (pos.re as isize).abs() + (pos.im as isize).abs()
}

#[test]
fn test_solve() {
    let input = "F10
N3
F7
R90
F11";
    assert_eq!(solve(input), 286);
}
