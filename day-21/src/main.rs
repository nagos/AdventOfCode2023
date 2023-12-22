use nom::{
    character::complete::{newline, one_of},
    multi::many1,
    IResult,
};

use std::{fs, vec};

type Data = Vec<Vec<char>>;
type Point = (usize, usize);

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data, 64);
    println!("Day 21 part one: {part_one}");
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, data) = many1(one_of(".#S"))(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn parse(input: &str) -> IResult<&str, Data> {
    many1(parse_line)(input)
}

fn find_start(data: &Data) -> Point {
    for (y, line) in data.iter().enumerate() {
        for (x, c) in line.iter().enumerate() {
            if *c == 'S' {
                return (x, y);
            }
        }
    }
    unreachable!()
}

fn proc_1(data: &str, steps: u32) -> usize {
    let (_, data) = parse(data).unwrap();
    let start = find_start(&data);

    let mut points = vec![start];

    for _ in 0..steps {
        let mut new_points = vec![];
        for p in points {
            let n = find_neighbours(&data, p);
            new_points.extend(n);
        }
        new_points.sort();
        new_points.dedup();
        points = new_points;
    }

    points.len()
}

fn find_neighbours(data: &Data, p: Point) -> Vec<Point> {
    let height = data.len();
    let width = data[0].len();
    let mut ret = vec![];

    // left
    if p.0 > 0 {
        let new_p = (p.0 - 1, p.1);
        if data[new_p.1][new_p.0] != '#' {
            ret.push(new_p);
        }
    }

    // right
    if p.0 < width - 1 {
        let new_p = (p.0 + 1, p.1);
        if data[new_p.1][new_p.0] != '#' {
            ret.push(new_p);
        }
    }

    // up
    if p.1 > 0 {
        let new_p = (p.0, p.1 - 1);
        if data[new_p.1][new_p.0] != '#' {
            ret.push(new_p);
        }
    }

    // down
    if p.1 < height - 1 {
        let new_p = (p.0, p.1 + 1);
        if data[new_p.1][new_p.0] != '#' {
            ret.push(new_p);
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, data) = parse(&data).unwrap();
        assert!(input.is_empty());

        let start = find_start(&data);
        assert_eq!(start, (5, 5));

        let n = find_neighbours(&data, start);
        assert!(n.contains(&(4, 5)));
        assert!(n.contains(&(5, 4)));
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data, 6);
        assert_eq!(res, 16);
    }
}
