use nom::{
    character::complete::{newline, one_of},
    multi::{many0, many1},
    IResult,
};

use std::fs;
use std::time::Instant;

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

#[derive(Debug)]
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

fn beam_move(data: &Vec<Vec<char>>, start_x: usize, start_y: usize, start_dir: Dir) -> u32 {
    let mut queue: Vec<(Dir, usize, usize)> = vec![];
    let width = data[0].len();
    let height = data.len();

    fn add_left(x: usize, y: usize, queue: &mut Vec<(Dir, usize, usize)>) {
        if x != 0 {
            queue.push((Dir::Left, x - 1, y));
        }
    }

    fn add_up(x: usize, y: usize, queue: &mut Vec<(Dir, usize, usize)>) {
        if y != 0 {
            queue.push((Dir::Up, x, y - 1));
        }
    }
    fn add_down(x: usize, y: usize, queue: &mut Vec<(Dir, usize, usize)>, height: usize) {
        if y != height - 1 {
            queue.push((Dir::Down, x, y + 1));
        }
    }
    fn add_right(x: usize, y: usize, queue: &mut Vec<(Dir, usize, usize)>, width: usize) {
        if x != width - 1 {
            queue.push((Dir::Right, x + 1, y));
        }
    }

    let mut visit_list = vec![vec![(false, false, false, false); width]; height];

    queue.push((start_dir, start_x, start_y));

    while let Some((d, x, y)) = queue.pop() {
        let visited = match d {
            Dir::Up => visit_list[y][x].0,
            Dir::Down => visit_list[y][x].1,
            Dir::Left => visit_list[y][x].2,
            Dir::Right => visit_list[y][x].3,
        };
        if visited {
            continue;
        }
        match d {
            Dir::Up => visit_list[y][x].0 = true,
            Dir::Down => visit_list[y][x].1 = true,
            Dir::Left => visit_list[y][x].2 = true,
            Dir::Right => visit_list[y][x].3 = true,
        };
        let c = data[y][x];
        match (d, c) {
            (Dir::Right, '.') => add_right(x, y, &mut queue, width),
            (Dir::Right, '|') => {
                add_up(x, y, &mut queue);
                add_down(x, y, &mut queue, height);
            }
            (Dir::Right, '-') => add_right(x, y, &mut queue, width),
            (Dir::Right, '\\') => {
                add_down(x, y, &mut queue, height);
            }
            (Dir::Right, '/') => {
                add_up(x, y, &mut queue);
            }
            (Dir::Down, '.') => {
                add_down(x, y, &mut queue, height);
            }
            (Dir::Down, '|') => {
                add_down(x, y, &mut queue, height);
            }
            (Dir::Down, '\\') => {
                add_right(x, y, &mut queue, width);
            }
            (Dir::Down, '/') => {
                add_left(x, y, &mut queue);
            }
            (Dir::Down, '-') => {
                add_left(x, y, &mut queue);
                add_right(x, y, &mut queue, width);
            }
            (Dir::Up, '/') => {
                add_right(x, y, &mut queue, width);
            }
            (Dir::Up, '|') => {
                add_up(x, y, &mut queue);
            }
            (Dir::Up, '.') => {
                add_up(x, y, &mut queue);
            }
            (Dir::Up, '\\') => {
                add_left(x, y, &mut queue);
            }
            (Dir::Up, '-') => {
                add_left(x, y, &mut queue);
                add_right(x, y, &mut queue, width);
            }
            (Dir::Left, '\\') => {
                add_up(x, y, &mut queue);
            }
            (Dir::Left, '/') => {
                add_down(x, y, &mut queue, height);
            }
            (Dir::Left, '|') => {
                add_up(x, y, &mut queue);
                add_down(x, y, &mut queue, height);
            }
            (Dir::Left, '.') => {
                add_left(x, y, &mut queue);
            }
            (Dir::Left, '-') => {
                add_left(x, y, &mut queue);
            }
            _ => unreachable!(),
        }
    }

    let mut ret = 0;

    for y in 0..height {
        for x in 0..width {
            let visited = visit_list[y][x];
            if visited.0 || visited.1 || visited.2 || visited.3 {
                ret += 1;
            }
        }
    }

    ret
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
