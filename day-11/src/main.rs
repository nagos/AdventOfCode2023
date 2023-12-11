use nom::{
    character::complete::{newline, one_of},
    multi::many1,
    sequence::terminated,
    IResult,
};

use itertools::Itertools;
use std::{fs, vec};

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data, 2);
    println!("Day 11 part one: {part_one}");

    let part_one = proc_1(&data, 1000000);
    println!("Day 11 part two: {part_one}");
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many1(one_of(".#")), newline)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(parse_line)(input)
}

fn process_data(data: Vec<Vec<char>>) -> (Vec<u32>, Vec<u32>, Vec<(usize, usize)>) {
    let height = data.len();
    let width = data[0].len();
    let mut empty_rows = vec![0; height];
    let mut empty_cols = vec![1; width];
    let mut galaxies = vec![];

    for (y, row) in data.iter().enumerate() {
        if row.iter().all(|&c| c == '.') {
            empty_rows[y] = 1;
        }
        for (x, &v) in row.iter().enumerate() {
            if v == '#' {
                empty_cols[x] = 0;
                galaxies.push((x, y));
            }
        }
    }

    (empty_rows, empty_cols, galaxies)
}

fn proc_1(data: &str, expand: u64) -> u64 {
    let (_, data) = parse(data).unwrap();
    let (empty_rows, empty_cols, galaxies) = process_data(data);

    let mut res = 0;
    for (g1, g2) in galaxies.iter().tuple_combinations() {
        let (min_x, max_x) = (g1.0.min(g2.0), g1.0.max(g2.0));
        let (min_y, max_y) = (g1.1.min(g2.1), g1.1.max(g2.1));

        let inc_x = &empty_cols[min_x..max_x].iter().sum::<u32>();
        let inc_y = &empty_rows[min_y..max_y].iter().sum::<u32>();

        let len = max_x - min_x + max_y - min_y;
        let correction = (inc_y + inc_x) as u64 * (expand - 1);

        res += len as u64 + correction;
    }

    res
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, data) = parse(&data).unwrap();
        assert!(input.is_empty());
        let (_empty_rows, _empty_cols, galaxies) = process_data(data);
        assert_eq!(galaxies.len(), 9);
    }

    #[test]
    fn test_calc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data, 2);
        assert_eq!(res, 374);
        let res = proc_1(&data, 10);
        assert_eq!(res, 1030);
    }
}
