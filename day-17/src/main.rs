use nom::{
    character::complete::{newline, one_of},
    combinator::map_res,
    multi::{many0, many1},
    IResult,
};

use std::collections::{BinaryHeap, HashMap};
use std::{fs, vec};

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash, PartialOrd, Ord)]
struct NodePos {
    x: u32,
    y: u32,
    dir: Dir,
    cons: u32,
}

#[derive(Debug, PartialEq, Eq)]
struct PathNode {
    score: u32,
    pos: NodePos,
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
    let part_two = proc_2(&data);
    println!("Day 17 part two: {part_two}");
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

fn get_next_dir(node: &NodePos, width: usize, height: usize, part2: bool) -> Vec<Dir> {
    let mut ret = vec![];

    if !part2 {
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
                if node.y != 0 {
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
                if node.y != 0 {
                    ret.push(Dir::Up)
                }
                if node.y != height as u32 - 1 {
                    ret.push(Dir::Down)
                }
            }
        }
    } else if node.cons < 4 {
        match node.dir {
            Dir::Down => {
                if node.y != height as u32 - 1 {
                    ret.push(Dir::Down)
                }
            }
            Dir::Up => {
                if node.y != 0 {
                    ret.push(Dir::Up)
                }
            }
            Dir::Left => {
                if node.x != 0 {
                    ret.push(Dir::Left)
                }
            }
            Dir::Right => {
                if node.x != width as u32 - 1 {
                    ret.push(Dir::Right)
                }
            }
        }
    } else if node.cons <= 10 {
        match node.dir {
            Dir::Down => {
                if node.x != 0 {
                    ret.push(Dir::Left)
                }
                if node.x != width as u32 - 1 {
                    ret.push(Dir::Right)
                }
                if node.y != height as u32 - 1 {
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
                if node.y != 0 {
                    ret.push(Dir::Up)
                }
            }
            Dir::Left => {
                if node.x != 0 {
                    ret.push(Dir::Left)
                }
                if node.y != 0 {
                    ret.push(Dir::Up)
                }
                if node.y != height as u32 - 1 {
                    ret.push(Dir::Down)
                }
            }
            Dir::Right => {
                if node.x != width as u32 - 1 {
                    ret.push(Dir::Right)
                }
                if node.y != 0 {
                    ret.push(Dir::Up)
                }
                if node.y != height as u32 - 1 {
                    ret.push(Dir::Down)
                }
            }
        }
    }

    ret
}

fn get_next_node(node: &PathNode, dir: Dir, data: &[Vec<u32>]) -> PathNode {
    let (new_x, new_y) = match dir {
        Dir::Down => (node.pos.x, node.pos.y + 1),
        Dir::Left => (node.pos.x - 1, node.pos.y),
        Dir::Right => (node.pos.x + 1, node.pos.y),
        Dir::Up => (node.pos.x, node.pos.y - 1),
    };

    let new_score = node.score + data[new_y as usize][new_x as usize];
    let new_cons = if dir == node.pos.dir {
        node.pos.cons + 1
    } else {
        1
    };

    PathNode {
        score: new_score,
        pos: NodePos {
            x: new_x,
            y: new_y,
            dir,
            cons: new_cons,
        },
    }
}

#[allow(dead_code)]
fn reconstruct_path(mut pos: NodePos, came_from: HashMap<NodePos, NodePos>) {
    println!("{} {}", pos.x, pos.y);

    while let Some(new_pos) = came_from.get(&pos) {
        pos = *new_pos;
        println!("{} {}", pos.x, pos.y);
    }
}

fn pathfind(data: Vec<Vec<u32>>, part2: bool) -> u32 {
    let width = data[0].len();
    let height = data.len();
    let (target_x, target_y) = (width as u32 - 1, height as u32 - 1);
    let mut open_set = BinaryHeap::new();
    let mut visited: HashMap<NodePos, u32> = HashMap::new();
    let mut came_from: HashMap<NodePos, NodePos> = HashMap::new();

    let start = PathNode {
        score: 0,
        pos: NodePos {
            cons: 0,
            x: 0,
            y: 0,
            dir: Dir::Right,
        },
    };

    let start2 = PathNode {
        score: 0,
        pos: NodePos {
            cons: 0,
            x: 0,
            y: 0,
            dir: Dir::Down,
        },
    };

    visited.insert(start.pos, start.score);
    open_set.push(start);
    visited.insert(start2.pos, start2.score);
    open_set.push(start2);

    let mut found = None;

    while let Some(node) = open_set.pop() {
        if node.pos.x == target_x
            && node.pos.y == target_y
            && (!part2 || (node.pos.cons >= 4 && node.pos.cons <= 10))
        {
            found = Some(node);
            break;
        }

        for dir in get_next_dir(&node.pos, width, height, part2) {
            let next_node = get_next_node(&node, dir, &data);
            if let Some(v) = visited.get_mut(&next_node.pos) {
                if *v > next_node.score {
                    *v = next_node.score;
                    came_from.insert(next_node.pos, node.pos);
                }
            } else {
                visited.insert(next_node.pos, next_node.score);
                came_from.insert(next_node.pos, node.pos);
                open_set.push(next_node);
            }
        }
    }

    // reconstruct_path(found.unwrap().pos, came_from);
    found.unwrap().score
}

fn proc_1(data: &str) -> u32 {
    let (input, data) = parse(data).unwrap();
    assert!(input.is_empty());
    pathfind(data, false)
}

fn proc_2(data: &str) -> u32 {
    let (input, data) = parse(data).unwrap();
    assert!(input.is_empty());
    pathfind(data, true)
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
        let res = pathfind(data, false);
        assert_eq!(res, 102);
    }

    #[test]
    fn test_proc1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 102);
    }

    #[test]
    fn test_proc2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_2(&data);
        assert_eq!(res, 94);
    }

    #[test]
    fn test_proc2_2() {
        let data = fs::read_to_string("data/test2.txt").unwrap();
        let res = proc_2(&data);
        assert_eq!(res, 71);
    }

    #[test]
    fn test_next_dirs() {
        let res = get_next_dir(
            &NodePos {
                cons: 0,
                x: 0,
                y: 0,
                dir: Dir::Right,
            },
            12,
            12,
            true,
        );
        assert_eq!(res, vec![Dir::Right]);
    }
}
