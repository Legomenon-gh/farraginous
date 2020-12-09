use std::collections::{HashSet, VecDeque};

type Deck = VecDeque<u8>;

pub fn part_1(input: &str) -> usize {
    let (mut p1_deck, mut p2_deck) = parse_input(input);

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();

        if p1_card > p2_card {
            p1_deck.extend(&[p1_card, p2_card]);
        } else {
            p2_deck.extend(&[p2_card, p1_card]);
        }
    }

    get_score(p1_deck, p2_deck)
}

pub fn part_2(input: &str) -> usize {
    let (p1_deck, p2_deck) = parse_input(input);

    let decks = play_game(p1_deck, p2_deck);
    get_score(decks.0, decks.1)
}

fn play_game(mut p1_deck: Deck, mut p2_deck: Deck) -> (Deck, Deck) {
    let mut p1_states = HashSet::new();
    let mut p2_states = HashSet::new();

    let max_p1_card = p1_deck.iter().max().unwrap();
    let max_p2_card = p2_deck.iter().max().unwrap();

    if max_p1_card > max_p2_card && max_p1_card > &((p1_deck.len() + p2_deck.len()) as u8) {
        // Shortcircuit recursion game if P2 cannot force a win or a subgame
        p2_deck.clear();
    }

    while !p1_deck.is_empty() && !p2_deck.is_empty() {
        let p1_card = p1_deck.pop_front().unwrap();
        let p2_card = p2_deck.pop_front().unwrap();
        let is_new_state = p1_states.insert(hash(&p1_deck)) || p2_states.insert(hash(&p2_deck));

        if !is_new_state {
            p2_deck.clear();
            break;
        }

        let mut p1_winner = p1_card > p2_card;

        if p1_deck.len() as u8 >= p1_card && p2_deck.len() as u8 >= p2_card {
            let (_, subgame_p2_deck) = play_game(
                p1_deck.iter().take(p1_card as usize).copied().collect(),
                p2_deck.iter().take(p2_card as usize).copied().collect(),
            );

            p1_winner = subgame_p2_deck.is_empty();
        }

        if p1_winner {
            p1_deck.extend(&[p1_card, p2_card]);
        } else {
            p2_deck.extend(&[p2_card, p1_card]);
        }
    }

    (p1_deck, p2_deck)
}

fn parse_input(input: &str) -> (Deck, Deck) {
    let (p1, p2) = &input.split_once("\n\n").unwrap();
    let parse_deck = |input: &str| {
        input
            .lines()
            .skip(1)
            .map(|line| line.parse().unwrap())
            .collect()
    };
    (parse_deck(p1), parse_deck(p2))
}

fn get_score(p1_deck: Deck, p2_deck: Deck) -> usize {
    p1_deck
        .iter()
        .chain(p2_deck.iter())
        .rev()
        .enumerate()
        .map(|(idx, card)| *card as usize * (idx + 1))
        .sum()
}

fn hash(deck: &Deck) -> usize {
    // The maximum number in my decks is 50 but go a little higher for safety
    deck.iter()
        .enumerate()
        .fold(0, |acc, (idx, num)| acc + ((idx + 1) * 60 * *num as usize))
}

#[test]
fn test_solve() {
    let input = "Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
    assert_eq!(part_1(&input), 306);
    assert_eq!(part_2(&input), 291);
}

#[test]
fn test_part_2_puzzle_input() {
    let input = crate::read_file("D:/D/Rust/Advent/input/day22.txt");
    assert_eq!(part_2(&input), 32769);
}
