use nom::{
    character::complete::{newline, one_of},
    multi::{many0, many1},
    IResult,
};

use std::fs;

type Block = Vec<Vec<char>>;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc(&data, true);
    println!("Day 13 part one: {part_one}");
    let part_two = proc(&data, false);
    println!("Day 13 part two: {part_two}");
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, data) = many1(one_of(".#?"))(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn parse_block(input: &str) -> IResult<&str, Block> {
    let (input, data) = many1(parse_line)(input)?;
    let (input, _) = many0(newline)(input)?;

    Ok((input, data))
}

fn parse(input: &str) -> IResult<&str, Vec<Block>> {
    many1(parse_block)(input)
}

fn find_hor(data: &Block, target: usize) -> Option<usize> {
    let width = data[0].len();
    let height = data.len();
    let last_index = height - 1;
    for i in 0..last_index {
        let mut diff = 0;
        for j in 0.. {
            let top = i - j;
            let bot = i + 1 + j;
            diff += (0..width).filter(|&q| data[top][q] != data[bot][q]).count();

            if top == 0 || bot == last_index {
                break;
            }
        }
        if diff == target {
            return Some(i);
        }
    }

    None
}

#[allow(clippy::needless_range_loop)]
fn find_ver(data: &Block, target: usize) -> Option<usize> {
    let width = data[0].len();
    let height = data.len();
    let last_index = width - 1;
    for i in 0..last_index {
        let mut diff = 0;
        for j in 0.. {
            let left = i - j;
            let right = i + 1 + j;
            diff += (0..height)
                .filter(|&q| data[q][left] != data[q][right])
                .count();

            if left == 0 || right == last_index {
                break;
            }
        }
        if diff == target {
            return Some(i);
        }
    }

    None
}

fn process_block(data: &Block, part_one: bool) -> usize {
    let target = if part_one { 0 } else { 1 };
    if let Some(x) = find_hor(data, target) {
        (x + 1) * 100
    } else if let Some(x) = find_ver(data, target) {
        x + 1
    } else {
        unreachable!();
    }
}

fn proc(data: &str, part_one: bool) -> usize {
    let (_, data) = parse(data).unwrap();

    data.iter().map(|b| process_block(b, part_one)).sum()
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
    fn test_process_block() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, data) = parse(&data).unwrap();
        assert!(input.is_empty());
        let res = process_block(&data[0], true);
        assert_eq!(res, 5);
        let res = process_block(&data[1], true);
        assert_eq!(res, 400);
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc(&data, true);
        assert_eq!(res, 405);
        let res = proc(&data, false);
        assert_eq!(res, 400);
    }

    #[test]
    fn test_proc_2() {
        let data = fs::read_to_string("data/test2.txt").unwrap();
        let res = proc(&data, true);
        assert_eq!(res, 100);
    }

    #[test]
    fn test_find_hor() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, data) = parse(&data).unwrap();
        assert!(input.is_empty());
        let res = find_hor(&data[0], 0);
        assert_eq!(res, None);
        let res = find_hor(&data[1], 0);
        assert_eq!(res, Some(3));

        let res = find_hor(&data[0], 1);
        assert_eq!(res, Some(2));
        let res = find_hor(&data[1], 1);
        assert_eq!(res, Some(0));
    }

    #[test]
    fn test_find_ver() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, data) = parse(&data).unwrap();
        assert!(input.is_empty());
        let res = find_ver(&data[0], 0);
        assert_eq!(res, Some(4));
        let res = find_ver(&data[1], 0);
        assert_eq!(res, None);
        let res = find_ver(&data[0], 1);
        assert_eq!(res, None);
        let res = find_ver(&data[1], 1);
        assert_eq!(res, None);
    }
}
