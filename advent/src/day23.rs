type List = Vec<usize>;

pub fn part_1(deck: List, turns: usize) -> String {
    let mut current_item = deck[0];
    let mut list = make_list(deck);

    for _ in 0..turns {
        run_step(&mut list, &mut current_item);
    }

    let mut output = String::new();
    let mut next = list[1];
    loop {
        if next == 1 {
            break;
        }
        output.push_str(&format!("{}", next));
        next = list[next];
    }
    output
}

pub fn part_2(mut deck: List) -> usize {
    let mut current_item = deck[0];
    deck.extend(10..=1_000_000);
    let mut list = make_list(deck);

    for _ in 0..10_000_000 {
        run_step(&mut list, &mut current_item);
    }

    let next = list[1];
    next * list[next]
}

fn run_step(list: &mut List, current_item: &mut usize) {
    // This code made me crawl under my turtle shell to avoid ever having to look at it again
    let pick_up_1 = list[*current_item];
    let pick_up_2 = list[pick_up_1];
    let pick_up_3 = list[pick_up_2];

    let mut target = get_target(*current_item, list.len());
    while pick_up_1 == target || pick_up_2 == target || pick_up_3 == target {
        target = get_target(target, list.len());
    }

    list[*current_item] = list[pick_up_3];
    list[pick_up_3] = list[target];
    list[target] = pick_up_1;
    *current_item = list[*current_item];
}

fn make_list(deck: List) -> List {
    // Store a list of array[item_value] => next neighbor
    let mut list = vec![0; deck.len() + 1];
    for (idx, element) in deck.iter().enumerate() {
        let neighbor = deck[(idx + 1) % deck.len()];
        list[*element] = neighbor;
    }
    list
}

fn get_target(target: usize, max: usize) -> usize {
    if target <= 1 {
        max - 1
    } else {
        target - 1
    }
}

#[test]
fn test_part_1() {
    let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    assert_eq!(part_1(input.clone(), 10), "92658374");
    assert_eq!(part_1(input, 100), "67384529");
}

#[test]
fn test_part_2() {
    let input = vec![3, 8, 9, 1, 2, 5, 4, 6, 7];
    assert_eq!(part_2(input), 149245887792);
}
