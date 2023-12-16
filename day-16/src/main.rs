use nom::{
    character::complete::{newline, one_of},
    multi::{many0, many1},
    IResult,
};

use std::collections::HashSet;
use std::time::Instant;
use std::{fs, vec};

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let now = Instant::now();
    let part_one = calc_1(&data);
    let duration = now.elapsed();
    println!("Day 16 part one: {part_one} ({duration:.2?})");

    let now = Instant::now();
    let part_two = calc_2(&data);
    let duration = now.elapsed();
    println!("Day 16 part two: {part_two} ({duration:.2?})");
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    let (input, data) = many1(one_of("./\\-|"))(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    let (input, data) = many1(parse_line)(input)?;
    let (input, _) = many0(newline)(input)?;

    Ok((input, data))
}

fn next_dir(incoming: Dir, c: char) -> Vec<Dir> {
    match (incoming, c) {
        (_, '.') => vec![incoming],
        (Dir::Right, '|') => vec![Dir::Up, Dir::Down],
        (Dir::Right, '-') => vec![Dir::Right],
        (Dir::Right, '\\') => vec![Dir::Down],
        (Dir::Right, '/') => vec![Dir::Up],
        (Dir::Down, '|') => vec![Dir::Down],
        (Dir::Down, '\\') => vec![Dir::Right],
        (Dir::Down, '/') => vec![Dir::Left],
        (Dir::Down, '-') => vec![Dir::Left, Dir::Right],
        (Dir::Up, '|') => vec![Dir::Up],
        (Dir::Up, '\\') => vec![Dir::Left],
        (Dir::Up, '/') => vec![Dir::Right],
        (Dir::Up, '-') => vec![Dir::Left, Dir::Right],
        (Dir::Left, '|') => vec![Dir::Up, Dir::Down],
        (Dir::Left, '\\') => vec![Dir::Up],
        (Dir::Left, '/') => vec![Dir::Down],
        (Dir::Left, '-') => vec![Dir::Left],
        _ => unreachable!(),
    }
}

fn get_next_cell(
    dir: Dir,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
) -> Option<(Dir, usize, usize)> {
    match dir {
        Dir::Left => {
            if x != 0 {
                Some((dir, x - 1, y))
            } else {
                None
            }
        }
        Dir::Up => {
            if y != 0 {
                Some((dir, x, y - 1))
            } else {
                None
            }
        }
        Dir::Down => {
            if y != height - 1 {
                Some((dir, x, y + 1))
            } else {
                None
            }
        }
        Dir::Right => {
            if x != width - 1 {
                Some((dir, x + 1, y))
            } else {
                None
            }
        }
    }
}

fn beam_move(data: &Vec<Vec<char>>, start_x: usize, start_y: usize, start_dir: Dir) -> u32 {
    let mut queue: Vec<(Dir, usize, usize)> = vec![];
    let width = data[0].len();
    let height = data.len();
    let mut visit_list: HashSet<(Dir, u8, u8)> = HashSet::new();

    queue.push((start_dir, start_x, start_y));
    visit_list.insert((start_dir, start_x as u8, start_y as u8));
    while let Some((d, x, y)) = queue.pop() {
        let c = data[y][x];
        next_dir(d, c)
            .iter()
            .filter_map(|&dir| get_next_cell(dir, x, y, width, height))
            .for_each(|(dir, x, y)| {
                if !visit_list.contains(&(d, x as u8, y as u8)) {
                    queue.push((dir, x, y));
                    visit_list.insert((d, x as u8, y as u8));
                }
            });
    }

    visit_list
        .iter()
        .fold(
            HashSet::new(),
            |mut s: HashSet<(u8, u8)>, (_, x, y): &(Dir, u8, u8)| {
                s.insert((*x, *y));
                s
            },
        )
        .len() as u32
}

fn calc_1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    beam_move(&data, 0, 0, Dir::Right)
}

fn calc_2(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let width = data[0].len();
    let height = data.len();

    let mut max_value = 0;

    // right
    for y in 0..height {
        max_value = max_value.max(beam_move(&data, 0, y, Dir::Right));
    }

    // left
    for y in 0..height {
        max_value = max_value.max(beam_move(&data, width - 1, y, Dir::Left));
    }

    // down
    for x in 0..width {
        max_value = max_value.max(beam_move(&data, x, 0, Dir::Down));
    }

    // up
    for x in 0..width {
        max_value = max_value.max(beam_move(&data, x, height - 1, Dir::Up));
    }

    max_value
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
    fn test_calc1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = calc_1(&data);
        assert_eq!(res, 46);
    }

    #[test]
    fn test_calc2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = calc_2(&data);
        assert_eq!(res, 51);
    }
}
