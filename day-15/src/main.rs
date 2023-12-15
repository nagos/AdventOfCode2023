use nom::{
    bytes::complete::tag,
    character::complete::{newline, none_of},
    multi::{many1, separated_list1},
    IResult,
};

use std::fs;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc(&data);
    println!("Day 15 part one: {part_one}");
}

fn parse_block(input: &str) -> IResult<&str, Vec<char>> {
    many1(none_of(",\n"))(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, data) = separated_list1(tag(","), parse_block)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn hash(data: &Vec<char>) -> u32 {
    let mut ret = 0;

    for &d in data {
        ret += d as u32;
        ret *= 17;
        ret %= 256;
    }

    ret
}

fn proc(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();

    data.iter().map(hash).sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, _data) = parse(&data).unwrap();
        assert!(input.is_empty());
    }

    #[test]
    fn test_hash() {
        let data = vec!['H', 'A', 'S', 'H'];

        let res = hash(&data);
        assert_eq!(res, 52);
    }

    #[test]
    fn test_proc() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc(&data);
        assert_eq!(res, 1320);
    }
}
