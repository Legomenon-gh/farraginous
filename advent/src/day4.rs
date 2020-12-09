pub fn part2(input: &str) -> isize {
    let passports = input.split("\n\n").collect::<Vec<_>>();
    passports
        .iter()
        .fold(0, |acc, el| if check_passport(el) { acc + 1 } else { acc })
}

fn check_passport(passport: &str) -> bool {
    let mut entries = passport.split_whitespace();
    if ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .all(|field| passport.contains(field))
    {
        entries.all(|entry| check_entry(entry))
    } else {
        false
    }
}

fn check_entry(entry: &str) -> bool {
    let split = entry.split(":").collect::<Vec<_>>();
    let field = split[0];
    let value = split[1].replace("#", "");

    match field {
        "byr" => (1920..2003).contains(&value.parse::<i32>().unwrap()),
        "iyr" => (2010..2021).contains(&value.parse::<i32>().unwrap()),
        "eyr" => (2020..2031).contains(&value.parse::<i32>().unwrap()),
        "hgt" => is_valid_height(&value),
        "hcl" => split[1].contains("#") && usize::from_str_radix(&value, 16).is_ok(),
        "ecl" => ["blu", "amb", "brn", "gry", "grn", "hzl", "oth"].contains(&value.as_str()),
        "pid" => value.len() == 9,
        "cid" => true,
        _ => false,
    }
}

fn is_valid_height(input: &str) -> bool {
    let value = input
        .replace("cm", "")
        .replace("in", "")
        .parse::<i32>()
        .unwrap();
    if input.contains("cm") {
        (150..194).contains(&value)
    } else if input.contains("in") {
        (59..77).contains(&value)
    } else {
        false
    }
}

#[test]
fn check_valid_passports() {
    let mut input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f";
    assert!(check_passport(input));

    input = "eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm";
    assert!(check_passport(input));

    input = "hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";
    assert!(check_passport(input));

    input = "iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719";
    assert!(check_passport(input));
}

#[test]
fn check_invalid_passports() {
    let mut input = "pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980";
    assert!(!check_passport(input));

    input = "eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926";
    assert!(!check_passport(input));

    input = "iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946";
    assert!(!check_passport(input));

    input = "hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277";
    assert!(!check_passport(input));

    input = "hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007";
    assert!(!check_passport(input));

    input = "hcl:#888785
hgt:164in byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022";
    assert!(!check_passport(input));
}
