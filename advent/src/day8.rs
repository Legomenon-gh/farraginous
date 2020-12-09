use std::collections::HashSet;

pub fn solve(input: &str) -> isize {
    let mut result = None;
    let program = input.lines().collect::<Vec<_>>();

    for (index, line) in program.iter().enumerate() {
        if line.contains("acc") {
            continue;
        }

        result = run_program(&program, index as isize);

        if result.is_some() {
            break;
        }
    }

    result.unwrap()
}

fn run_program(program: &Vec<&str>, replace_idx: isize) -> Option<isize> {
    let mut visited = HashSet::new();
    let mut index = 0;
    let mut acc = 0;

    loop {
        if index as usize >= program.len() {
            return Some(acc);
        }

        if visited.contains(&index) {
            return None; // Infinite loop
        }

        visited.insert(index);

        let split: Vec<_> = program[index as usize].split(" ").collect();
        let mut command = split[0];
        let argument = split[1].parse::<isize>().unwrap();

        if index == replace_idx {
            command = if command == "nop" { "jmp" } else { "nop" };
        }

        match command {
            "acc" => acc += argument,
            "jmp" => index += argument - 1,
            "nop" => {}
            _ => panic!(),
        }

        index += 1;
    }
}

#[test]
fn test_solve() {
    let input = "nop +0
acc +1
jmp +4
acc +3
jmp -3
acc -99
acc +1
jmp -4
acc +6";
    assert_eq!(solve(input), 8);
}
