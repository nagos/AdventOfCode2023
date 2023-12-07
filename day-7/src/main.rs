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
    let part_one = proc1(&data);
    println!("Day 7 part one: {part_one}");
}

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

const CARDS: &[&str] = &[
    "A", "K", "Q", "J", "T", "9", "8", "7", "6", "5", "4", "3", "2",
];

fn card_to_int(card: &str) -> u32 {
    CARDS.iter().position(|&x| x == card).unwrap() as u32
}

fn process_hand(hand: (Vec<&str>, u32)) -> (Vec<u32>, u32) {
    let cards = hand.0.into_iter().map(card_to_int).collect::<Vec<u32>>();
    (cards, hand.1)
}

fn get_hand_score(hand: Vec<u32>) -> Vec<u32> {
    let mut card_types = vec![0; CARDS.len()];
    let mut ret = hand.clone();

    for c in hand {
        card_types[c as usize] += 1;
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

fn proc1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();

    let mut bids = vec![];

    for (cards, bid) in data {
        let hand = process_hand((cards, bid));
        let hand_score = get_hand_score(hand.0);
        bids.push((hand_score, bid));
    }

    bids.sort();
    bids.reverse();

    let mut ret = 0;
    for (i, b) in bids.iter().enumerate() {
        ret += b.1 * (i as u32 + 1);
    }

    ret
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

        let hand = process_hand((cards, bid));
        let hand_score = get_hand_score(hand.0);
        assert_eq!(hand_score, vec![5, 11, 12, 4, 11, 1]);
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
        let res = proc1(&data);
        assert_eq!(res, 6440);
    }
}
