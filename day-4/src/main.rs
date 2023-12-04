use nom::bytes::complete::tag;
use nom::character::complete::{digit1, space0, space1};
use nom::combinator::map_res;
use nom::multi::separated_list1;
use nom::IResult;
use std::collections::HashSet;
use std::fs;
use std::iter::FromIterator;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let total = proc(&data);
    println!("Day 4 tart one: {total}");
}

fn digit1_padded(input: &str) -> IResult<&str, u32> {
    let (input, _) = space0(input)?;
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_line(input: &str) -> IResult<&str, (u32, Vec<u32>, Vec<u32>)> {
    let (input, _) = tag("Card ")(input)?;
    let (input, card_id) = digit1_padded(input)?;
    let (input, _) = tag(": ")(input)?;
    let (input, numbers_winning) = separated_list1(space1, digit1_padded)(input)?;
    let (input, _) = tag(" | ")(input)?;
    let (input, numbers_have) = separated_list1(space1, digit1_padded)(input)?;

    Ok((input, (card_id, numbers_winning, numbers_have)))
}

fn calc_points(numbers_have: Vec<u32>, numbers_winning: Vec<u32>) -> u32 {
    let have: HashSet<u32> = HashSet::from_iter(numbers_have);
    let winning: HashSet<u32> = HashSet::from_iter(numbers_winning);
    let intersect = have.intersection(&winning);
    let count = intersect.count();

    match count as u32 {
        0 => 0,
        x => 2_u32.pow(x - 1),
    }
}

fn proc(data: &str) -> u32 {
    let mut ret = 0;
    for input in data.lines() {
        let (_, (_card_id, numbers_winning, numbers_have)) = parse_line(input).unwrap();
        let score = calc_points(numbers_have, numbers_winning);
        ret += score;
    }

    ret
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse() {
        let data = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53";
        let (input, (card_id, numbers_winning, numbers_have)) = parse_line(data).unwrap();
        assert!(input.is_empty());
        assert_eq!(card_id, 1);
        assert_eq!(numbers_winning, vec![41, 48, 83, 86, 17]);
        assert_eq!(numbers_have, vec![83, 86, 6, 31, 17, 9, 48, 53]);

        let score = calc_points(numbers_have, numbers_winning);
        assert_eq!(score, 8);
    }

    #[test]
    fn test_parse_space() {
        let data = "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1";
        let (input, (card_id, numbers_winning, numbers_have)) = parse_line(data).unwrap();
        assert!(input.is_empty());
        assert_eq!(card_id, 3);
        assert_eq!(numbers_winning, vec![1, 21, 53, 59, 44]);
        assert_eq!(numbers_have, vec![69, 82, 63, 72, 16, 21, 14, 1]);
    }

    #[test]
    fn test_file() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let total = proc(&data);
        assert_eq!(total, 13);
    }

    #[test]
    fn test_parser_padded() {
        let data = " 123";
        let (input, value) = digit1_padded(data).unwrap();
        assert!(input.is_empty());
        assert_eq!(value, 123);
    }
}
