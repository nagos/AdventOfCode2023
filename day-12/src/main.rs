use nom::{
    character::complete::{char, digit1, newline, one_of, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

use std::{fs, vec};
use std::time::Instant;

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

fn solve(data: Vec<char>, unknown_list: Vec<u32>, list: &Vec<u32>) -> u32 {
    let mut ret = 0;

    if !unknown_list.is_empty() {
        let mut new_data = data.clone();
        let mut new_unknown_list = unknown_list.clone();
        let idx = new_unknown_list.pop().unwrap();
        new_data[idx as usize] = '#';
        ret += solve(new_data.clone(), new_unknown_list.clone(), list);
        new_data[idx as usize] = '.';
        ret += solve(new_data, new_unknown_list, list);
    } else {
        let res_list = calc_list(&data);
        if res_list == *list {
            ret = 1;
        }
    }

    ret
}

fn find_unknown(data: &[char]) -> Vec<u32> {
    let mut ret = vec![];

    for (i, c) in data.iter().enumerate() {
        if *c == '?' {
            ret.push(i as u32);
        }
    }

    ret
}

fn proc_1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let mut ret = 0;
    for d in data {
        let (data, list) = d;
        let unknown_list = find_unknown(&data);

        ret += solve(data, unknown_list, &list);
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
        let data = vec!['?', '?', '?', '.', '#', '#', '#'];
        let list = vec![1, 1, 3];
        let unknown_list = vec![0, 1, 2];
        let res = solve(data, unknown_list, &list);
        assert_eq!(res, 1);

        // let data = vec!['?','#','#','#','?','?','?','?','?','?','?', '?'];
        // let list = vec![3,2,1];
        let data = vec![
            '.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#', '.',
        ];
        let list = vec![1, 1, 3];
        let unknown_list = find_unknown(&data);
        let res = solve(data, unknown_list, &list);
        assert_eq!(res, 4);
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 21);
    }
}
