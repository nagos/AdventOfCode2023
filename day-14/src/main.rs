use nom::{
    character::complete::{newline, one_of},
    multi::{many0, many1},
    IResult,
};

use std::{fmt::Display, fs};
#[derive(Clone, PartialEq)]
struct InputData {
    data: Vec<Vec<char>>,
}

impl InputData {
    #[allow(dead_code)]
    fn col_iter(&self, col: usize) -> impl Iterator<Item = &char> {
        let width = self.data[0].len();
        self.data.iter().flatten().skip(col).step_by(width)
    }
    #[allow(dead_code)]
    fn col_iter_mut(&mut self, col: usize) -> impl Iterator<Item = &mut char> {
        let width = self.data[0].len();
        self.data.iter_mut().flatten().skip(col).step_by(width)
    }
    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }
}

impl Display for InputData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for l in self.data.iter() {
            for e in l {
                write!(f, "{e}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc(&data);
    println!("Day 14 part one: {part_one}");
    let part_two = proc_2(&data);
    println!("Day 14 part two: {part_two}");
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, data) = many1(one_of(".#O"))(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, data) = many1(parse_line)(input)?;
    let (input, _) = many0(newline)(input)?;

    Ok((input, data))
}

fn calc_1(data: InputData) -> usize {
    let mut data = data;
    tilt_north(&mut data);

    score(&data)
}

fn calc_2(data: InputData) -> usize {
    let (start, len) = find_loop(data.clone());
    let mut data = data;

    let cycles = start + (1000000000 - start) % len;
    for _ in 0..cycles {
        tilt_cycle(&mut data);
    }

    score(&data)
}

fn tilt_north(data: &mut InputData) {
    for x in 0..data.width() {
        let mut last_square = 0;

        for y in 0..data.height() {
            let s = &mut data.data[y][x];
            if *s == 'O' {
                *s = '.';
                data.data[last_square][x] = 'O';
                last_square += 1;
            } else if *s == '#' {
                last_square = y + 1;
            }
        }
    }
}

fn tilt_west(data: &mut InputData) {
    for y in 0..data.height() {
        let mut last_square = 0;
        for x in 0..data.width() {
            let s = &mut data.data[y][x];
            if *s == 'O' {
                *s = '.';
                data.data[y][last_square] = 'O';
                last_square += 1;
            } else if *s == '#' {
                last_square = x + 1;
            }
        }
    }
}

fn tilt_south(data: &mut InputData) {
    for x in 0..data.width() {
        let mut last_square = data.height();

        for y in (0..data.height()).rev() {
            let s = &mut data.data[y][x];
            if *s == 'O' {
                *s = '.';
                data.data[last_square - 1][x] = 'O';
                last_square -= 1;
            } else if *s == '#' {
                last_square = y;
            }
        }
    }
}

fn tilt_east(data: &mut InputData) {
    for y in 0..data.height() {
        let mut last_square = data.width();
        for x in (0..data.width()).rev() {
            let s = &mut data.data[y][x];
            if *s == 'O' {
                *s = '.';
                data.data[y][last_square - 1] = 'O';
                last_square -= 1;
            } else if *s == '#' {
                last_square = x;
            }
        }
    }
}

fn tilt_cycle(data: &mut InputData) {
    tilt_north(data);
    tilt_west(data);
    tilt_south(data);
    tilt_east(data);
}

fn score(data: &InputData) -> usize {
    let mut ret = 0;
    for x in 0..data.width() {
        for y in 0..data.height() {
            let s = data.data[y][x];
            if s == 'O' {
                ret += data.height() - y;
            }
        }
    }

    ret
}

fn find_loop(data: InputData) -> (usize, usize) {
    let mut tortoise = data.clone();
    let mut hare = data.clone();

    let mut loop_start = 0;
    let mut loop_length = 0;
    loop {
        tilt_cycle(&mut hare);
        tilt_cycle(&mut hare);
        tilt_cycle(&mut tortoise);
        if hare == tortoise {
            break;
        }
    }

    let mut tortoise = data.clone();
    for i in 0.. {
        if hare == tortoise {
            loop_start = i;
            break;
        }
        tilt_cycle(&mut hare);
        tilt_cycle(&mut tortoise);
    }
    for i in 1.. {
        tilt_cycle(&mut hare);
        if hare == tortoise {
            loop_length = i;
            break;
        }
    }

    (loop_start, loop_length)
}

fn proc(data: &str) -> usize {
    let (_, data) = parse(data).unwrap();

    let data = InputData { data };
    calc_1(data)
}

fn proc_2(data: &str) -> usize {
    let (_, data) = parse(data).unwrap();

    let data = InputData { data };
    calc_2(data)
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
    fn test_calc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (_input, data) = parse(&data).unwrap();

        let data = InputData { data };
        let res = calc_1(data);
        assert_eq!(res, 136);
    }

    #[test]
    fn test_calc_2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (_input, data) = parse(&data).unwrap();

        let data = InputData { data };
        let res = calc_2(data);
        assert_eq!(res, 64);
    }

    #[test]
    fn test_tilt() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (_input, data) = parse(&data).unwrap();

        let mut data = InputData { data: data.clone() };

        tilt_cycle(&mut data);
        tilt_cycle(&mut data);
        tilt_cycle(&mut data);
        let res = score(&data);
        assert_eq!(res, 69);
    }

    #[test]
    fn test_find_loop() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (_input, data) = parse(&data).unwrap();

        let data = InputData { data };
        let (start, len) = find_loop(data);
        assert_eq!(start, 3);
        assert_eq!(len, 7);
    }
}
