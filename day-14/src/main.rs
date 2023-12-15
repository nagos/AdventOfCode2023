use nom::{
    character::complete::{newline, one_of},
    multi::{many0, many1},
    IResult,
};

use std::fs;
struct InputData {
    data: Vec<Vec<char>>,
}

impl InputData {
    fn col_iter(&self, col: usize) -> impl Iterator<Item = &char> {
        let width = self.data[0].len();
        self.data.iter().flatten().skip(col).step_by(width)
    }
    fn width(&self) -> usize {
        self.data[0].len()
    }

    fn height(&self) -> usize {
        self.data.len()
    }
}

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc(&data);
    println!("Day 43 part one: {part_one}");
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

fn calc_1(data: &InputData) -> usize {
    let mut ret = 0;
    for i in 0..data.width() {
        let mut last_square = 0;

        for (idx, &s) in data.col_iter(i).enumerate() {
            if s == 'O' {
                ret += data.height() - last_square;
                last_square += 1;
            } else if s == '#' {
                last_square = idx + 1;
            }
        }
    }

    ret
}

fn proc(data: &str) -> usize {
    let (_, data) = parse(data).unwrap();

    let data = InputData { data };
    calc_1(&data)
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
        let res = calc_1(&data);
        assert_eq!(res, 136);
    }
}
