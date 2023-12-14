use nom::{
    character::complete::{newline, one_of},
    multi::{many0, many1},
    IResult,
};

use std::fs;

type Block = Vec<Vec<char>>;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 13 part one: {part_one}");
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

fn find_hor(data: &Block) -> Option<usize> {
    let width = data[0].len();
    let height = data.len();
    for i in 0..height - 1 {
        let mut top = i;
        let mut bot = i + 1;
        loop {
            let mut res = true;
            for q in 0..width {
                if data[top][q] != data[bot][q] {
                    res = false;
                    break;
                }
            }
            if !res {
                break;
            }

            if top == 0 || bot == height - 1 {
                return Some(i);
            }

            top -= 1;
            bot += 1;
        }
    }

    None
}

#[allow(clippy::needless_range_loop)]
fn find_ver(data: &Block) -> Option<usize> {
    let width = data[0].len();
    let height = data.len();
    for i in 0..width - 1 {
        let mut left = i;
        let mut right = i + 1;
        loop {
            let mut res = true;
            for q in 0..height {
                if data[q][left] != data[q][right] {
                    res = false;
                    break;
                }
            }
            if !res {
                break;
            }

            if left == 0 || right == width - 1 {
                return Some(i);
            }

            left -= 1;
            right += 1;
        }
    }

    None
}

fn process_block(data: &Block) -> usize {
    if let Some(x) = find_hor(data) {
        (x + 1) * 100
    } else if let Some(x) = find_ver(data) {
        x + 1
    } else {
        unreachable!();
    }
}

fn proc_1(data: &str) -> usize {
    let (_, data) = parse(data).unwrap();

    data.iter().map(process_block).sum()
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
        let res = process_block(&data[0]);
        assert_eq!(res, 5);
        let res = process_block(&data[1]);
        assert_eq!(res, 400);
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 405);
    }

    #[test]
    fn test_proc_2() {
        let data = fs::read_to_string("data/test2.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 100);
    }
}
