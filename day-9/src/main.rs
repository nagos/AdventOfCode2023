use nom::{
    character::complete::{digit1, newline, one_of, space1},
    combinator::{map_res, opt},
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

use std::fs;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 9 part one: {part_one}");
}

fn digit1_i32(input: &str) -> IResult<&str, i32> {
    let (input, sign) = opt(one_of("-"))(input)?;
    let (input, value) = map_res(digit1, |s: &str| s.parse::<i32>())(input)?;
    let ret = if sign.is_some() { -value } else { value };
    Ok((input, ret))
}

fn pare_line(input: &str) -> IResult<&str, Vec<i32>> {
    terminated(separated_list1(space1, digit1_i32), newline)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<i32>>> {
    many1(pare_line)(input)
}

fn calc_line(data: Vec<i32>) -> i32 {
    let mut data = data;
    let mut res = 0;
    loop {
        res += *data.last().unwrap();

        if data.is_empty() || data.iter().all(|&x| x == 0) {
            break;
        }

        data = data
            .iter()
            .zip(data.iter().skip(1))
            .map(|(a1, a2)| a2 - a1)
            .collect::<Vec<i32>>();
    }

    res
}

fn calc_1(data: Vec<Vec<i32>>) -> i32 {
    data.into_iter().map(calc_line).sum()
}

fn proc_1(data: &str) -> i32 {
    let (_, data) = parse(data).unwrap();
    calc_1(data)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, _) = parse(&data).unwrap();
        assert!(input.is_empty());
    }

    #[test]
    fn test_parse_i32() {
        let data = "-123";
        let (input, data) = digit1_i32(&data).unwrap();
        assert!(input.is_empty());
        assert_eq!(data, -123);
    }

    #[test]
    fn test_calc_line() {
        let data = vec![0, 3, 6, 9, 12, 15];
        let res = calc_line(data);
        assert_eq!(res, 18);

        let data = vec![1, 3, 6, 10, 15, 21];
        let res = calc_line(data);
        assert_eq!(res, 28);

        let data = vec![10, 13, 16, 21, 30, 45];
        let res = calc_line(data);
        assert_eq!(res, 68);
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 114);
    }
}
