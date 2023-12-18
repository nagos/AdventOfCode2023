use nom::{
    bytes::complete::{tag, take_while_m_n},
    character::complete::{digit1, one_of, space1},
    combinator::map_res,
    multi::many1,
    IResult,
};

use itertools::Itertools;

use std::{fs, vec};

type DataLine = (char, u32, u32, u32);
fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 18 part one: {part_one}");

    let part_two = proc_2(&data);
    println!("Day 18 part two: {part_two}");
}

fn from_hex(input: &str) -> Result<u32, std::num::ParseIntError> {
    u32::from_str_radix(input, 16)
}

fn hex_primary(input: &str) -> IResult<&str, u32> {
    map_res(
        take_while_m_n(5, 5, |c: char| c.is_ascii_hexdigit()),
        from_hex,
    )(input)
}

fn digit1_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_line(input: &str) -> IResult<&str, DataLine> {
    let (input, dir) = one_of("RLUD")(input)?;
    let (input, _) = space1(input)?;
    let (input, count) = digit1_u32(input)?;
    let (input, _) = tag(" (#")(input)?;
    let (input, hex_distance) = hex_primary(input)?;
    let (input, hex_dir) = map_res(
        take_while_m_n(1, 1, |c: char| c.is_ascii_hexdigit()),
        from_hex,
    )(input)?;
    let (input, _) = tag(")\n")(input)?;

    Ok((input, (dir, count, hex_distance, hex_dir)))
}

fn parse(input: &str) -> IResult<&str, Vec<DataLine>> {
    let (input, data) = many1(parse_line)(input)?;

    Ok((input, data))
}

fn data_to_coordinates(data: Vec<DataLine>) -> (Vec<(i32, i32)>, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut res = vec![];

    let mut p = 1;
    for line in data {
        let inc = line.1 as i32;
        match line.0 {
            'R' => x += inc,
            'L' => x -= inc,
            'U' => y += inc,
            'D' => y -= inc,
            _ => unreachable!(),
        }
        res.push((x, y));
        p += inc;
    }

    (res, p / 2 + 1)
}

fn data_to_coordinates_2(data: Vec<DataLine>) -> (Vec<(i32, i32)>, i32) {
    let mut x = 0;
    let mut y = 0;
    let mut res = vec![];

    let mut p = 1;
    for line in data {
        let inc = line.2 as i32;
        match line.3 {
            0 => x += inc,
            2 => x -= inc,
            3 => y += inc,
            1 => y -= inc,
            _ => unreachable!(),
        }
        res.push((x, y));
        p += inc;
    }

    (res, p / 2 + 1)
}

fn calc_area(data: Vec<(i32, i32)>) -> i64 {
    data.iter()
        .tuple_windows::<(&(i32, i32), &(i32, i32))>()
        .map(|(d1, d2)| (d1.1 + d2.1) as i64 * (d1.0 - d2.0) as i64)
        .sum::<i64>()
        .abs()
        / 2
}

fn proc_1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let (res, p) = data_to_coordinates(data);
    let res = calc_area(res);
    (res as i32 + p) as u32
}

fn proc_2(data: &str) -> u64 {
    let (_, data) = parse(data).unwrap();
    let (res, p) = data_to_coordinates_2(data);
    let res = calc_area(res);
    (res as u64 + p as u64) as u64
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
    fn test_coordinates() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (_input, data) = parse(&data).unwrap();
        let (res, p) = data_to_coordinates(data);
        let res = calc_area(res);
        assert_eq!(res, 42);
        assert_eq!(p, 20);
    }

    #[test]
    fn test_coordinates_2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (_input, data) = parse(&data).unwrap();
        let (res, p) = data_to_coordinates_2(data);
        let res = calc_area(res);
        assert_eq!(res, 952404941483);
        assert_eq!(p, 3202632);
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 62);
    }

    #[test]
    fn test_proc_2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_2(&data);
        assert_eq!(res, 952408144115);
    }
}
