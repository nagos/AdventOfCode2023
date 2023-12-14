use nom::{
    character::complete::{char, digit1, newline, one_of, space1},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

use std::fs;
use std::time::Instant;
use std::{collections::HashMap, vec};

type LineData = (Vec<char>, Vec<u32>);
type Cache = HashMap<(usize, usize), u64>;

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let now = Instant::now();
    let part_one = proc_1(&data);
    let part_one_duration = now.elapsed();
    println!("Day 12 part one (cache): {part_one} ({part_one_duration:.2?})");

    let now = Instant::now();
    let part_two = proc_2(&data);
    let part_two_duration = now.elapsed();
    println!("Day 12 part two (cache): {part_two} ({part_two_duration:.2?})");

    let now = Instant::now();
    let part_one = proc_1_tabular(&data);
    let part_one_duration = now.elapsed();
    println!("Day 12 part one (tabular): {part_one} ({part_one_duration:.2?})");

    let now = Instant::now();
    let part_two = proc_2_tabular(&data);
    let part_two_duration = now.elapsed();
    println!("Day 12 part two (tabular): {part_two} ({part_two_duration:.2?})");
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

fn read_bang<'a>(data: &'a [char], list: &'a [u32], cache: &mut Cache) -> u64 {
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
            solve(&data[next_idx..], &list[1..], cache)
        } else {
            0
        }
    } else if list.len() == 1 {
        1
    } else {
        0
    }
}

fn solve<'a>(data: &'a [char], list: &'a [u32], cache: &mut Cache) -> u64 {
    if let Some(v) = cache.get(&(data.len(), list.len())) {
        return *v;
    }
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

    let current_group = list[0] as usize;
    let current_char = data[0];
    let ret = match current_char {
        _ if data.len() < current_group => 0,
        '.' => solve(&data[1..], list, cache),
        '#' => read_bang(data, list, cache),
        '?' => read_bang(data, list, cache) + solve(&data[1..], list, cache),
        _ => unreachable!(),
    };

    cache.insert((data.len(), list.len()), ret);

    ret
}

fn proc_1(data: &str) -> u64 {
    let (_, data) = parse(data).unwrap();

    data.iter()
        .map(|(data, list)| solve(data, list, &mut HashMap::new()))
        .sum()
}

fn proc_2(data: &str) -> u64 {
    let (_, data) = parse(data).unwrap();

    data.iter()
        .map(|(data, list)| part_two_process_data(data, list))
        .map(|(data, list)| solve(&data, &list, &mut HashMap::new()))
        .sum()
}

fn proc_1_tabular(data: &str) -> u64 {
    let (_, data) = parse(data).unwrap();

    data.iter()
        .map(|(data, list)| solve_table(data, list))
        .sum()
}

fn proc_2_tabular(data: &str) -> u64 {
    let (_, data) = parse(data).unwrap();

    data.iter()
        .map(|(data, list)| part_two_process_data(data, list))
        .map(|(data, list)| solve_table(&data, &list))
        .sum()
}

fn part_two_process_data(data: &Vec<char>, list: &Vec<u32>) -> (Vec<char>, Vec<u32>) {
    let mut new_data = data.clone();
    let mut new_list = list.clone();

    for _ in 1..5 {
        new_data.push('?');
        new_data.extend(data);
        new_list.extend(list);
    }

    (new_data, new_list)
}

fn solve_table(data: &[char], list: &[u32]) -> u64 {
    let width = data.len();
    let height = list.len();
    let mut tab = vec![vec![0; width + 1]; height + 1];

    // x - remaining chars
    // y - remaining groups

    for y in 0..=height {
        for x in 0..=width {
            let chars = &data[(data.len() - x)..];
            let v = match (x, y) {
                _ if y == 0 => {
                    if chars.iter().all(|&c| c == '.' || c == '?') {
                        1
                    } else {
                        0
                    }
                }
                _ if x == 0 => 0,
                (x, y) => {
                    let group = list[list.len() - y] as usize;

                    // skip
                    let skip_value = if chars[0] != '#' { tab[y][x - 1] } else { 0 };

                    // take
                    let take_value;
                    if x < group {
                        take_value = 0;
                    } else {
                        let group_chars = &chars[..group];

                        if group_chars.iter().all(|&c| c == '#' || c == '?') {
                            if x == group {
                                take_value = tab[y - 1][x - group];
                            } else if x > group && chars[group] != '#' {
                                take_value = tab[y - 1][x - group - 1];
                            } else {
                                take_value = 0;
                            };
                        } else {
                            take_value = 0;
                        }
                    }
                    take_value + skip_value
                }
            };

            tab[y][x] = v;
        }
    }
    tab[height][width]
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
        let res = solve(&data, &list, &mut HashMap::new());
        assert_eq!(res, 1);

        let data = vec![
            '.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#', '.',
        ];
        let list = vec![1, 1, 3];
        let res = solve(&data, &list, &mut HashMap::new());
        assert_eq!(res, 4);

        let data = vec!['?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'];
        let list = vec![3, 2, 1];
        let res = solve(&data, &list, &mut HashMap::new());
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

        let (new_data, new_list) = part_two_process_data(&data, &list);

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

    #[test]
    fn test_table() {
        let data = vec!['?', '?', '?', '.', '#', '#', '#'];
        let list = vec![1, 1, 3];
        let res = solve_table(&data, &list);
        assert_eq!(res, 1);

        let data = vec![
            '.', '?', '?', '.', '.', '?', '?', '.', '.', '.', '?', '#', '#', '.',
        ];
        let list = vec![1, 1, 3];
        let res = solve_table(&data, &list);
        assert_eq!(res, 4);

        let data = vec![
            '?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?', '#', '?',
        ];
        let list = vec![1, 3, 1, 6];
        let res = solve_table(&data, &list);
        assert_eq!(res, 1);

        let data = vec![
            '?', '?', '?', '?', '.', '#', '.', '.', '.', '#', '.', '.', '.',
        ];
        let list = vec![4, 1, 1];
        let res = solve_table(&data, &list);
        assert_eq!(res, 1);

        let data = vec![
            '?', '?', '?', '?', '.', '#', '#', '#', '#', '#', '#', '.', '.', '#', '#', '#', '#',
            '#', '.',
        ];
        let list = vec![1, 6, 5];
        let res = solve_table(&data, &list);
        assert_eq!(res, 4);

        let data = vec!['?', '#', '#', '#', '?', '?', '?', '?', '?', '?', '?', '?'];
        let list = vec![3, 2, 1];
        let res = solve_table(&data, &list);
        assert_eq!(res, 10);
    }

    #[test]
    fn test_proc_1_tabular() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1_tabular(&data);
        assert_eq!(res, 21);
    }

    #[test]
    fn test_proc_2_tabular() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_2_tabular(&data);
        assert_eq!(res, 525152);
    }
}
