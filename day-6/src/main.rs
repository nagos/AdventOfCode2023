use nom::{
    bytes::complete::{tag, take_until1},
    character::complete::{digit1, newline, space1},
    multi::{many0, many1},
    IResult,
};
use std::{fs, time::Instant};

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();

    let part_one = proc_1(&data);
    println!("Day 6 part one: {part_one}");

    let part_two = proc_2(&data);
    println!("Day 6 part one: {part_two}");

    let now = Instant::now();
    let part_two_brute_force = proc_2_brute_force(&data);
    println!("Day 6 part one: {part_two_brute_force}");
    println!("Elapsed: {:.2?}", now.elapsed());
}

fn line_parser(input: &str) -> IResult<&str, Vec<&str>> {
    let (input, _) = take_until1(":")(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, d) = many1(digit_parser)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, d))
}

fn digit_parser(input: &str) -> IResult<&str, &str> {
    let (input, _) = many0(space1)(input)?;
    digit1(input)
}

fn parse(input: &str) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    let (input, d1) = line_parser(input)?;
    let (input, d2) = line_parser(input)?;

    Ok((input, (d1, d2)))
}

fn process_input_part_one(data: Vec<&str>) -> Vec<u64> {
    data.iter()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<u64>>()
}

fn calc(t: u64, d: u64) -> u32 {
    let r1 = (t as f64) / 2.0 + ((t as f64).powf(2.0) - 4.0 * (d as f64)).sqrt() / 2.0;
    let r2 = (t as f64) / 2.0 - ((t as f64).powf(2.0) - 4.0 * (d as f64)).sqrt() / 2.0;
    let res = r1.ceil() - r2.floor() - 1.0;
    res as u32
}

fn proc_1(data: &str) -> u32 {
    let (_, (d1, d2)) = parse(data).unwrap();
    let d1 = process_input_part_one(d1);
    let d2 = process_input_part_one(d2);

    d1.into_iter().zip(d2).map(|(t, d)| calc(t, d)).product()
}

fn process_input_part_two(data: Vec<&str>) -> u64 {
    data.join("").parse::<u64>().unwrap()
}

fn proc_2(data: &str) -> u32 {
    let (_, (d1, d2)) = parse(data).unwrap();
    let d1 = process_input_part_two(d1);
    let d2 = process_input_part_two(d2);
    calc(d1, d2)
}

fn proc_2_brute_force(data: &str) -> u32 {
    let (_, (d1, d2)) = parse(data).unwrap();
    let d1 = process_input_part_two(d1);
    let d2 = process_input_part_two(d2);
    calc_brute_force(d1, d2)
}

fn calc_brute_force(t: u64, d: u64) -> u32 {
    (0..t).map(|x| t * x - x.pow(2)).filter(|x| x > &d).count() as u32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, _) = parse(&data).unwrap();
        assert!(input.is_empty());
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();

        let r = proc_1(&data);
        assert_eq!(r, 288);
    }

    #[test]
    fn test_proc_2() {
        let data = fs::read_to_string("data/test.txt").unwrap();

        let r = proc_2(&data);
        assert_eq!(r, 71503);
    }

    #[test]
    fn test_calc() {
        let res = calc(7, 9);
        assert_eq!(res, 4);
        let res = calc(15, 40);
        assert_eq!(res, 8);
        let res = calc(30, 200);
        assert_eq!(res, 9);
    }

    #[test]
    fn test_calc_brute_force() {
        let res = calc_brute_force(7, 9);
        assert_eq!(res, 4);
        let res = calc_brute_force(15, 40);
        assert_eq!(res, 8);
        let res = calc_brute_force(30, 200);
        assert_eq!(res, 9);
    }
}
