use itertools::Itertools;
use std::collections::HashMap;
use std::collections::HashSet;
struct Item<'a> {
    foods: Vec<&'a str>,
    allergens: Vec<&'a str>,
}
type Allergens<'a> = HashMap<&'a str, HashSet<&'a str>>;

pub fn part_1(input: &str) -> usize {
    let items = parse_input(input);
    let allergen_map = get_allergen_map(&items);

    let mut food_count = HashMap::new();
    for foods in items.iter().map(|item| &item.foods) {
        for food in foods {
            *food_count.entry(*food).or_insert(0) += 1;
        }
    }

    let mut possible_allergens = HashSet::new();
    for foods in allergen_map.values() {
        possible_allergens.extend(foods);
    }

    let all_foods: HashSet<_> = food_count.keys().copied().collect();
    all_foods
        .difference(&possible_allergens)
        .fold(0, |acc, food| acc + food_count[*food])
}

pub fn part_2(input: &str) -> String {
    let items = parse_input(input);
    let mut allergen_map = get_allergen_map(&items);

    let mut known_allergens: HashMap<&str, &str> = HashMap::new();
    while !allergen_map.is_empty() {
        for (allergen, foods) in &allergen_map {
            if foods.len() == 1 {
                known_allergens.insert(allergen, *foods.iter().next().unwrap());
                continue;
            }
        }
        for (allergen, food) in &known_allergens {
            allergen_map.remove(allergen);
            for candidates in allergen_map.values_mut() {
                candidates.remove(food);
            }
        }
    }

    known_allergens
        .into_iter()
        .sorted_by_key(|entry| entry.0)
        .into_iter()
        .map(|entry| entry.1)
        .collect_vec()
        .join(",")
}

fn get_allergen_map<'a>(items: &'a [Item]) -> Allergens<'a> {
    let mut allergen_map: Allergens = HashMap::new();
    for Item { foods, allergens } in items {
        for allergen in allergens {
            let new_foods: HashSet<_> = foods.iter().copied().collect();

            if allergen_map.contains_key(allergen) {
                let old_foods = allergen_map.get_mut(allergen).unwrap();
                *old_foods = &*old_foods & &new_foods;
            } else {
                allergen_map.insert(allergen, new_foods);
            }
        }
    }

    allergen_map
}

fn parse_input(input: &str) -> Vec<Item> {
    input
        .lines()
        .map(|line| {
            let (lhs, rhs) = line.split_once(" (contains ").unwrap();
            let foods = lhs.split(' ').collect();
            let (split, _) = rhs.split_once(")").unwrap();
            let allergens = split.split(", ").collect();
            Item { foods, allergens }
        })
        .collect_vec()
}

#[test]
fn test_part_1() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    assert_eq!(part_1(&input), 5);
}

#[test]
fn test_part_2() {
    let input = "mxmxvkd kfcds sqjhc nhms (contains dairy, fish)
trh fvjkl sbzzf mxmxvkd (contains dairy)
sqjhc fvjkl (contains soy)
sqjhc mxmxvkd sbzzf (contains fish)";
    assert_eq!(part_2(&input), "mxmxvkd,sqjhc,fvjkl");
}
