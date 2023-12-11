use nom::{
    character::complete::{newline, one_of},
    multi::many1,
    sequence::terminated,
    IResult,
};

use std::{fs, vec};

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 11 part one: {part_one}");
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
                galaxies.push((x, y ));
            }
        }
    }

    (empty_rows, empty_cols, galaxies)
}

fn proc_1(data: &str) -> u32 {
    let (_, data) = parse(&data).unwrap();
    let (empty_rows, empty_cols, galaxies) = process_data(data);

    let mut pair_list = vec![];

    for i in 0..galaxies.len() {
        for q in i + 1..galaxies.len() {
            pair_list.push((galaxies[i], galaxies[q]));
        }
    }

    let mut res = 0;
    for g in pair_list {
        let (g1, g2) = g;
        let (min_x, max_x) = (g1.0.min(g2.0), g1.0.max(g2.0));
        let (min_y, max_y) = (g1.1.min(g2.1), g1.1.max(g2.1));

        let inc_x = &empty_cols[min_x..max_x]
            .iter()
            .sum::<u32>();
        let inc_y = &empty_rows[min_y..max_y]
            .iter()
            .sum::<u32>();

        let len = (max_x - min_x + max_y - min_y) as u32 + inc_y + inc_x;

        res += len;
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
        let res = proc_1(&data);
        assert_eq!(res, 374);
    }
}
