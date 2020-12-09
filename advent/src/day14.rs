use std::collections::HashMap;

pub fn part_1(input: &str) -> isize {
    let lines = input.split_terminator('\n').collect::<Vec<_>>();
    let mut mem: HashMap<usize, isize> = HashMap::new();
    let (mut and_mask, mut or_mask) = (0, 0);

    for line in lines {
        let (op, arg) = line.split_once(" = ").unwrap();
        if op == "mask" {
            and_mask = 0;
            or_mask = 0;

            for (i, char) in arg.chars().rev().enumerate() {
                match char {
                    '0' => and_mask |= 1 << i,
                    '1' => or_mask |= 1 << i,
                    _ => {}
                }
            }
        } else {
            let index = op
                .chars()
                .skip(4)
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();

            let value = (arg.parse::<isize>().unwrap() & !and_mask) | or_mask;
            mem.insert(index, value);
        }
    }

    mem.values().sum()
}

pub fn part_2(input: &str) -> isize {
    let lines = input.split_terminator('\n').collect::<Vec<_>>();
    let mut mem: HashMap<usize, isize> = HashMap::new();
    let mut float_bits = Vec::new();
    let mut or_mask = 1;

    for line in lines {
        let (op, arg) = line.split_once(" = ").unwrap();
        if op == "mask" {
            or_mask = 0;
            float_bits = Vec::new();

            for (i, char) in arg.chars().rev().enumerate() {
                match char {
                    '1' => or_mask |= 1 << i,
                    'X' => float_bits.push(i),
                    _ => {}
                }
            }
        } else {
            let address = op
                .chars()
                .skip(4)
                .take_while(|c| c.is_numeric())
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            let value = arg.parse::<isize>().unwrap();

            let addresses = get_addresses(address, &float_bits);
            for address in addresses {
                mem.insert(address | or_mask, value);
            }
        }
    }

    mem.values().sum()
}

fn get_addresses(base_address: usize, float_bits: &Vec<usize>) -> Vec<usize> {
    let mut addresses = Vec::new();
    addresses.push(base_address);

    for bit in float_bits {
        let old_addresses = addresses.clone();
        addresses.clear();
        for address in old_addresses {
            addresses.push(address ^ (1 << bit));
            addresses.push(address ^ (0 << bit));
        }
    }

    addresses
}

#[test]
fn test_part_1() {
    let input = "mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX1XXXX0X
mem[8] = 11
mem[7] = 101
mem[8] = 0
mask = XXXXXXXXXXXXXXXXXXXXXXXXXXXXX0XXXX1X
mem[9] = 0";
    assert_eq!(part_1(input), 167);
}

#[test]
fn test_part_2() {
    let input = "mask = 000000000000000000000000000000X1001X
mem[42] = 100
mask = 00000000000000000000000000000000X0XX
mem[26] = 1";
    assert_eq!(part_2(input), 208);
}
