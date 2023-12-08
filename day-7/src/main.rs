use nom::{
    bytes::complete::take,
    character::complete::{digit1, newline, space1},
    combinator::map_res,
    multi::{many1, many_m_n},
    sequence::separated_pair,
    IResult,
};
use std::fs;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc(&data, false);
    println!("Day 7 part one: {part_one}");

    let part_two = proc(&data, true);
    println!("Day 7 part two: {part_two}");
}

const CARDS: &[&str] = &[
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
];

const CARDS2: &[&str] = &[
    "A", "K", "Q", "T", "9", "8", "7", "6", "5", "4", "3", "2", "J",
];

fn digit1_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_line(input: &str) -> IResult<&str, (Vec<&str>, u32)> {
    let (input, (cards, bid)) =
        separated_pair(many_m_n(5, 5, take(1usize)), space1, digit1_u32)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (cards, bid)))
}

fn parse(input: &str) -> IResult<&str, Vec<(Vec<&str>, u32)>> {
    many1(parse_line)(input)
}

fn card_to_int(card: &str, use_jockers: bool) -> u32 {
    let table = if !use_jockers { CARDS } else { CARDS2 };
    table.iter().position(|&x| x == card).unwrap() as u32
}

fn process_hand(cards: Vec<&str>, use_jockers: bool) -> Vec<u32> {
    cards
        .into_iter()
        .map(|x| card_to_int(x, use_jockers))
        .collect()
}

fn is_jocker(card: u32) -> bool {
    card == CARDS2.len() as u32 - 1
}

fn get_hand_score(hand: Vec<u32>, use_jockers: bool) -> Vec<u32> {
    let mut card_types = vec![0; CARDS.len()];
    let mut ret = hand.clone();

    let mut jokers = 0;
    let mut max_index = 0;

    for c in hand {
        if use_jockers && is_jocker(c) {
            jokers += 1;
        } else {
            card_types[c as usize] += 1;
        }

        if card_types[c as usize] > card_types[max_index] {
            max_index = c as usize;
        }
    }

    if use_jockers {
        card_types[max_index] += jokers;
    }

    // 5 4 3 2
    let mut doubles = (0, 0, 0, 0);

    for c in card_types {
        if c == 5 {
            doubles.0 += 1;
        } else if c == 4 {
            doubles.1 += 1;
        } else if c == 3 {
            doubles.2 += 1;
        } else if c == 2 {
            doubles.3 += 1;
        }
    }

    let score = match doubles {
        (1, _, _, _) => 0, // Five of a kind
        (_, 1, _, _) => 1, // Four of a kind
        (_, _, 1, 1) => 2, // Full house
        (_, _, 1, _) => 3, // Three of a kind
        (_, _, _, 2) => 4, // Two pair
        (_, _, _, 1) => 5, // One pair
        _ => 6,            // High card
    };

    ret.insert(0, score);

    ret
}

fn proc(data: &str, use_jockers: bool) -> u32 {
    let (_, data) = parse(data).unwrap();

    let mut hands = vec![];

    for (cards, bid) in data {
        let hand = process_hand(cards, use_jockers);
        let hand_score = get_hand_score(hand, use_jockers);
        hands.push((hand_score, bid));
    }

    hands.sort();

    hands
        .iter()
        .rev()
        .enumerate()
        .map(|(i, b)| b.1 * (i as u32 + 1))
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = "32T3K 765\n";

        let (input, (cards, bid)) = parse_line(data).unwrap();

        assert!(input.is_empty());
        assert_eq!(bid, 765);

        let hand = process_hand(cards, false);
        let hand_score = get_hand_score(hand, false);
        assert_eq!(hand_score, vec![5, 11, 12, 4, 11, 1]);
    }

    #[test]
    fn test_parse2() {
        let data = "T55J5 684\n";

        let (_, (cards, _bid)) = parse_line(data).unwrap();

        let hand = process_hand(cards, true);
        let hand_score = get_hand_score(hand, true);
        assert_eq!(hand_score, vec![1, 3, 8, 8, 12, 8]);
    }

    #[test]
    fn test_parse_file() {
        let data = fs::read_to_string("data/test.txt").unwrap();

        let (input, _) = parse(&data).unwrap();
        assert!(input.is_empty());
    }

    #[test]
    fn test_proc1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc(&data, false);
        assert_eq!(res, 6440);
    }

    #[test]
    fn test_proc2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc(&data, true);
        assert_eq!(res, 5905);
    }
}
