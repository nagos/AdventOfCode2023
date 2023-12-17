use nom::{
    character::complete::{newline, one_of},
    combinator::map_res,
    multi::{many0, many1},
    IResult,
};

use std::collections::{BinaryHeap, HashMap, HashSet};
use std::{fs, vec};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Eq)]
struct PathNode {
    score: u32,
    x: u32,
    y: u32,
    dir: Dir,
    cons: u32,
}

impl PartialOrd for PathNode {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(other.score.cmp(&self.score))
    }
}

impl Ord for PathNode {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.score.cmp(&self.score)
    }
}

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 17 part one: {part_one}");
}

fn parse_line(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, data) = many1(map_res(one_of("0123456789"), |c| {
        c.to_string().parse::<u32>()
    }))(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<u32>>> {
    let (input, data) = many1(parse_line)(input)?;
    let (input, _) = many0(newline)(input)?;

    Ok((input, data))
}

fn get_next_dir(node: &PathNode, width: usize, height: usize) -> Vec<Dir> {
    let mut ret = vec![];
    match node.dir {
        Dir::Down => {
            if node.x != 0 {
                ret.push(Dir::Left)
            }
            if node.x != width as u32 - 1 {
                ret.push(Dir::Right)
            }
            if node.y != height as u32 - 1 && node.cons != 3 {
                ret.push(Dir::Down)
            }
        }
        Dir::Up => {
            if node.x != 0 {
                ret.push(Dir::Left)
            }
            if node.x != width as u32 - 1 {
                ret.push(Dir::Right)
            }
            if node.y != 0 && node.cons != 3 {
                ret.push(Dir::Up)
            }
        }
        Dir::Left => {
            if node.x != 0 && node.cons != 3 {
                ret.push(Dir::Left)
            }
            if node.y != 0 as u32 {
                ret.push(Dir::Up)
            }
            if node.y != height as u32 - 1 {
                ret.push(Dir::Down)
            }
        }
        Dir::Right => {
            if node.x != width as u32 - 1 && node.cons != 3 {
                ret.push(Dir::Right)
            }
            if node.y != 0 as u32 {
                ret.push(Dir::Up)
            }
            if node.y != height as u32 - 1 {
                ret.push(Dir::Down)
            }
        }
    }
    ret
}

fn get_next_node(node: &PathNode, dir: Dir, data: &Vec<Vec<u32>>) -> PathNode {
    let (new_x, new_y) = match dir {
        Dir::Down => (node.x, node.y + 1),
        Dir::Left => (node.x - 1, node.y),
        Dir::Right => (node.x + 1, node.y),
        Dir::Up => (node.x, node.y - 1),
    };

    let new_score = node.score + data[new_y as usize][new_x as usize];
    let new_cons = if dir == node.dir { node.cons + 1 } else { 1 };

    PathNode {
        score: new_score,
        x: new_x,
        y: new_y,
        dir: dir,
        cons: new_cons,
    }
}

fn reconstruct_path(
    x: u32,
    y: u32,
    d: Dir,
    cons: u32,
    came_from: &HashMap<(u32, u32, Dir, u32), (u32, u32, Dir, u32)>,
    data: &Vec<Vec<u32>>,
) {
    let mut x = x;
    let mut y = y;
    let mut d = d;
    let mut cons = cons;

    println!("{x} {y} {}", data[y as usize][x as usize]);
    let mut m = HashSet::new();
    let mut score = data[y as usize][x as usize];
    m.insert((x, y));
    while let Some(&(next_x, next_y, next_d, next_cons)) = came_from.get(&(x, y, d, cons)) {
        score += data[y as usize][x as usize];
        x = next_x;
        y = next_y;
        d = next_d;
        cons = next_cons;
        m.insert((x, y));
        println!("{x} {y} {}", data[y as usize][x as usize]);
    }

    dbg!(score - data[0][0]);
    for y in 0..data.len() {
        for x in 0..data[0].len() {
            if m.contains(&(x as u32, y as u32)) {
                print!("x");
            } else {
                print!("{}", data[y][x]);
            }
        }
        println!();
    }
}

fn resort_heap<T: Ord>(b: &mut BinaryHeap<T>) {
    use std::mem::swap;
    let mut temp = BinaryHeap::new();
    swap(b, &mut temp);
    temp = BinaryHeap::from(temp);
    swap(b, &mut temp);
}

fn pathfind(data: Vec<Vec<u32>>) -> u32 {
    let width = data[0].len();
    let height = data.len();
    let (target_x, target_y) = (width as u32 - 1, height as u32 - 1);
    let mut open_set = BinaryHeap::new();
    let mut visited: HashMap<(u32, u32, Dir, u32), u32> = HashMap::new();
    let mut came_from: HashMap<(u32, u32, Dir, u32), (u32, u32, Dir, u32)> = HashMap::new();

    let start = PathNode {
        cons: 0,
        score: 0,
        x: 0,
        y: 0,
        dir: Dir::Right,
    };

    visited.insert((0, 0, Dir::Right, 0), 0);
    open_set.push(start);

    let mut found = None;

    while let Some(node) = open_set.pop() {
        if node.x == target_x && node.y == target_y {
            found = Some((node.x, node.y, node.dir, node.cons, node.score));
            break;
        }

        for dir in get_next_dir(&node, width, height) {
            let next_node = get_next_node(&node, dir, &data);
            if let Some(v) =
                visited.get_mut(&(next_node.x, next_node.y, next_node.dir, next_node.cons))
            {
                if *v > next_node.score {
                    *v = next_node.score;
                    came_from.insert(
                        (next_node.x, next_node.y, next_node.dir, next_node.cons),
                        (node.x, node.y, node.dir, node.cons),
                    );
                    open_set.push(next_node);
                    resort_heap(&mut open_set);
                }
            } else {
                visited.insert(
                    (next_node.x, next_node.y, next_node.dir, next_node.cons),
                    next_node.score,
                );
                came_from.insert(
                    (next_node.x, next_node.y, next_node.dir, next_node.cons),
                    (node.x, node.y, node.dir, node.cons),
                );
                open_set.push(next_node);
                resort_heap(&mut open_set);
            }
        }
    }

    found.unwrap().4
}

fn proc_1(data: &str) -> u32 {
    let (input, data) = parse(data).unwrap();
    assert!(input.is_empty());
    pathfind(data)
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
    fn test_pathfind() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, data) = parse(&data).unwrap();
        assert!(input.is_empty());
        let res = pathfind(data);
        assert_eq!(res, 102);
    }

    #[test]
    fn test_proc1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 102);
    }
}
