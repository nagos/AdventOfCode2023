use nom::{
    character::complete::{char, digit1, newline, one_of, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

use std::time::Instant;
use std::{fs, vec};

type LineData = (Vec<char>, Vec<u32>);

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let now = Instant::now();
    let part_one = proc_1(&data);
    let part_one_duration = now.elapsed();
    println!("Day 12 part one: {part_one} ({part_one_duration:.2?})");
}

fn digit1_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_list(input: &str) -> IResult<&str, Vec<u32>> {
    separated_list1(char(','), digit1_u32)(input)
}

fn parse_line(input: &str) -> IResult<&str, LineData> {
    let (input, data) = many1(one_of(".#?"))(input)?;
    let (input, _) = space1(input)?;
    let (input, list) = parse_list(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (data, list)))
}

fn parse(input: &str) -> IResult<&str, Vec<LineData>> {
    many1(parse_line)(input)
}

#[allow(dead_code)]
fn calc_list(data: &Vec<char>) -> Vec<u32> {
    let mut tmp = 0;
    let mut res = vec![];
    for c in data {
        if *c == '#' {
            tmp += 1;
        } else if tmp != 0 {
            res.push(tmp);
            tmp = 0;
        }
    }

    if tmp != 0 {
        res.push(tmp);
    }

    res
}

fn verify_list(data: &Vec<char>, list: &Vec<u32>) -> bool {
    let mut idx = 0;
    let mut tmp = 0;

    for c in data {
        if *c == '#' {
            tmp += 1;
        } else if tmp != 0 {
            if idx >= list.len() || tmp != list[idx] {
                return false;
            }
            tmp = 0;
            idx += 1;
        }
    }

    if tmp != 0 {
        if idx >= list.len() || tmp != list[idx] {
            return false;
        }
        idx += 1;
    }

    if idx != list.len() {
        return false;
    }

    true
}

fn solve(data: &mut Vec<char>, unknown_list: &Vec<usize>, idx: usize, list: &Vec<u32>) -> u32 {
    let mut ret = 0;

    if idx < unknown_list.len() {
        let char_idx = unknown_list[idx];
        data[char_idx] = '#';
        ret += solve(data, unknown_list, idx + 1, list);
        data[char_idx] = '.';
        ret += solve(data, unknown_list, idx + 1, list);
        data[char_idx] = '?';
    } else if verify_list(data, list) {
        ret = 1;
    }

    ret
}

fn find_unknown(data: &[char]) -> Vec<usize> {
    let mut ret = vec![];

    for (i, c) in data.iter().enumerate() {
        if *c == '?' {
            ret.push(i);
        }
    }

    ret
}

fn proc_1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let mut ret = 0;
    for d in data {
        let (mut data, list) = d;
        let unknown_list = find_unknown(&data);

        ret += solve(&mut data, &unknown_list, 0, &list);
    }

    ret
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
    fn test_list() {
        let data = vec!['#', '.', '#', '.', '#', '#', '#'];
        let list = vec![1, 1, 3];

        let res = calc_list(&data);
        assert_eq!(res, list);
    }

    #[test]
    fn test_find_unknown() {
        let data = vec!['?', '?', '?', '.', '#', '#', '#'];
        let res = find_unknown(&data);
        assert_eq!(res, vec![0, 1, 2]);
    }

    #[test]
    fn test_solve() {
        let mut data = vec!['?', '?', '?', '.', '#', '#', '#'];
        let list = vec![1, 1, 3];
        let unknown_list = vec![0, 1, 2];
        let res = solve(&mut data, &unknown_list, 0, &list);
        assert_eq!(res, 1);

        let mut data = vec![
            '.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#', '.',
        ];
        let list = vec![1, 1, 3];
        let unknown_list = find_unknown(&data);
        let res = solve(&mut data, &unknown_list, 0, &list);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_verify_list() {
        assert!(verify_list(
            &vec!['#', '.', '#', '.', '#', '#', '#'],
            &vec![1, 1, 3]
        ));
        assert!(verify_list(
            &vec!['.', '#', '#', '#', '.', '#', '#', '.', '#', '.', '.', '.'],
            &vec![3, 2, 1]
        ));
        assert!(verify_list(
            &vec!['.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '#', '.'],
            &vec![3, 2, 1]
        ));
        assert_eq!(
            verify_list(
                &vec!['.', '#', '#', '#', '.', '.', '.', '#', '#', '.', '.', '.'],
                &vec![3, 2, 1]
            ),
            false
        );
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 21);
    }
}
