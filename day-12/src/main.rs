use nom::{
    character::complete::{char, digit1, newline, one_of, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

use std::time::Instant;
use std::fs;

type LineData = (Vec<char>, Vec<u32>);

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let now = Instant::now();
    let part_one = proc_1(&data);
    let part_one_duration = now.elapsed();
    println!("Day 12 part one: {part_one} ({part_one_duration:.2?})");

    let now = Instant::now();
    let part_two = proc_2(&data);
    let part_two_duration = now.elapsed();
    println!("Day 12 part two: {part_two} ({part_two_duration:.2?})");
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

fn read_bang(data: &[char], list: &[u32]) -> u32 {
    if data.is_empty() {
        return 0;
    }

    let group = list[0] as usize;

    let group_data = &data[0..group];

    if !group_data.iter().all(|&c| c == '#' || c == '?') {
        return 0;
    }

    if group != data.len() {
        let next_char = data[group];
        if next_char == '?' || next_char == '.' {
            let next_idx = group + 1;
            let next_data = &data[next_idx..];
            let next_list = &list[1..];
            return solve(next_data, next_list);
        } else {
            return 0;
        }
    }

    if list.len() == 1 {
        1
    } else {
        0
    }
}

fn solve(data: &[char], list: &[u32]) -> u32 {
    if list.is_empty() {
        if data.iter().all(|&c| c == '.' || c == '?') {
            return 1;
        } else {
            return 0;
        }
    }

    if data.is_empty() && !list.is_empty() {
        return 0;
    }

    for idx in 0..data.len() {
        let current_group = list[0] as usize;

        if data.len() - idx < current_group {
            return 0;
        }

        let current_char = data[idx];
        if current_char == '.' {
            continue;
        }

        if current_char == '#' {
            let next_data = &data[idx..];
            return read_bang(next_data, list);
        }

        if current_char == '?' {
            let next_data = &data[idx..];
            let res_bang = read_bang(next_data, list);
            let next_data = &data[(idx + 1)..];
            let res_dot = solve(next_data, list);

            return res_bang + res_dot;
        }
    }

    0
}

fn proc_1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let mut ret = 0;
    for d in data {
        let (data, list) = d;
        ret += solve(&data, &list);
    }

    ret
}

fn proc_2(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let mut ret = 0;

    for d in data {
        let (data, list) = d;
        let (new_data, new_list) = part_two_process_data(data, list);

        ret += solve(&new_data, &new_list);
    }

    ret
}

fn part_two_process_data(data: Vec<char>, list: Vec<u32>) -> (Vec<char>, Vec<u32>) {
    let mut new_data = data.clone();
    let mut new_list = list.clone();

    for _ in 1..5 {
        new_data.push('?');
        new_data.extend(&data);
        new_list.extend(&list);
    }

    (new_data, new_list)
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
    fn test_solve() {
        let data = vec!['?', '?', '?', '.', '#', '#', '#'];
        let list = vec![1, 1, 3];
        let res = solve(&data, &list);
        assert_eq!(res, 1);

        let data = vec![
            '.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#', '.',
        ];
        let list = vec![1, 1, 3];
        let res = solve(&data, &list);
        assert_eq!(res, 4);

        let data = vec!['?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'];
        let list = vec![3, 2, 1];
        let res = solve(&data, &list);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_part_two_expand() {
        let data = vec![
            '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#',
        ];
        let list = vec![1, 1, 1, 1, 1];

        let (new_data, new_list) = part_two_process_data(data, list);

        let res_data = vec![
            '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#',
            '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.',
            '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?',
            '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#', '?', '.', '#',
            '?', '.', '#', '?', '.', '#',
        ];

        let res_list = vec![
            1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1,
        ];

        assert_eq!(new_data, res_data);
        assert_eq!(new_list, res_list);
    }

    #[test]
    fn test_proc_2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_2(&data);
        assert_eq!(res, 525152);
    }
}
