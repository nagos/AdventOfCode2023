use nom::{
    bytes::complete::{tag, take_till1},
    character::complete::{alphanumeric1, digit1, newline},
    combinator::map_res,
    combinator::opt,
    multi::separated_list1,
    IResult,
};

use std::fs;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc(&data);
    println!("Day 15 part one: {part_one}");
    let part_two = proc_2(&data);
    println!("Day 15 part two: {part_two}");
}

fn parse_block(input: &str) -> IResult<&str, &str> {
    take_till1(|c| c == ',' || c == '\n')(input)
}

fn parse(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, data) = separated_list1(tag(","), parse_block)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn digit1_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_value(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("=")(input)?;
    let (input, value) = digit1_u32(input)?;

    Ok((input, value))
}

fn parse_part2(input: &str) -> IResult<&str, (&str, Option<u32>)> {
    let (input, label) = alphanumeric1(input)?;
    let (mut input, value) = opt(parse_value)(input)?;

    if value.is_none() {
        (input, _) = tag("-")(input)?;
    }

    Ok((input, (label, value)))
}

fn hash(data: &str) -> u32 {
    data.chars().fold(0, |acc, x| ((acc + x as u32) * 17) % 256)
}

fn proc(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();

    data.iter().map(|&s| hash(s)).sum()
}

fn box_find_lens(store: &[(&str, u32)], label: &str) -> Option<usize> {
    for (i, v) in store.iter().enumerate() {
        if v.0 == label {
            return Some(i);
        }
    }
    None
}

fn proc_2(data: &str) -> usize {
    let mut store: Vec<Vec<(&str, u32)>> = vec![vec![]; 256];
    let (_, data) = parse(data).unwrap();

    for l in data {
        let (_, (label, value)) = parse_part2(l).unwrap();
        let h = hash(label);
        let lens_box = &mut store[h as usize];
        let idx = box_find_lens(lens_box, label);

        match (value, idx) {
            (Some(val), Some(idx)) => {
                lens_box[idx].1 = val;
            }
            (Some(val), None) => {
                lens_box.push((label, val));
            }
            (None, Some(idx)) => {
                lens_box.remove(idx);
            }
            _ => {}
        };
    }

    store
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(q, lens)| (i + 1) * (q + 1) * (lens.1 as usize))
                .sum::<usize>()
        })
        .sum()
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
        let data = "HASH";
        let res = hash(&data);
        assert_eq!(res, 52);
    }

    #[test]
    fn test_proc() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc(&data);
        assert_eq!(res, 1320);
    }

    #[test]
    fn test_parse_2() {
        let data = "rn=1";
        let (input, data) = parse_part2(&data).unwrap();
        assert!(input.is_empty());
        assert_eq!(data.1, Some(1))
    }

    #[test]
    fn test_proc_2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_2(&data);
        assert_eq!(res, 145);
    }
}
