use nom::{
    character::complete::{newline, one_of},
    multi::many1,
    sequence::terminated,
    IResult,
};

use std::fs;

type NodePos = (u32, u32);

#[derive(Debug)]
struct Node {
    adjacent: Vec<NodePos>,
    pos: NodePos,
    value: char,
}

impl Node {
    fn build(pos: NodePos, value: char) -> Self {
        Node {
            adjacent: vec![],
            pos,
            value,
        }
    }
}

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 10 part one: {part_one}");
}

fn parse_line(input: &str) -> IResult<&str, Vec<char>> {
    terminated(many1(one_of("|-LJ7F.S")), newline)(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<char>>> {
    many1(parse_line)(input)
}

fn analyze(data: Vec<Vec<char>>) -> (NodePos, Vec<Vec<Node>>) {
    let mut nodes = vec![];
    let mut start_pos = (0, 0);

    for (y, line) in data.iter().enumerate() {
        let mut tmp = vec![];
        for (x, c) in line.iter().enumerate() {
            let pos = (x as u32, y as u32);
            tmp.push(Node::build(pos, *c));
            if *c == 'S' {
                start_pos = pos;
            }
        }
        nodes.push(tmp);
    }

    (start_pos, nodes)
}

fn adjacent_push(nodes: &mut Vec<Vec<Node>>, pos: NodePos, dir: u32) {
    // 0 - top
    // 1 - right
    // 2 - down
    // 3 - left

    let width = nodes[0].len() as u32;
    let height = nodes.len() as u32;
    let (x, y) = pos;
    match dir {
        0 => {
            if y > 0 {
                nodes[y as usize - 1][x as usize].adjacent.push(pos);
            }
        }
        1 => {
            if x < width - 1 {
                nodes[y as usize][x as usize + 1].adjacent.push(pos);
            }
        }
        2 => {
            if y < height - 1 {
                nodes[y as usize + 1][x as usize].adjacent.push(pos);
            }
        }
        3 => {
            if x > 0 {
                nodes[y as usize][x as usize - 1].adjacent.push(pos);
            }
        }
        _ => unreachable!(),
    }
}

fn build_adjacency_list(nodes: &mut Vec<Vec<Node>>) {
    let width = nodes[0].len();
    let height = nodes.len();

    for y in 0..height {
        for x in 0..width {
            let pos = (x as u32, y as u32);
            let node = &nodes[y][x];
            match node.value {
                '.' => {}
                '|' => {
                    adjacent_push(nodes, pos, 0);
                    adjacent_push(nodes, pos, 2);
                }
                '-' => {
                    adjacent_push(nodes, pos, 3);
                    adjacent_push(nodes, pos, 1);
                }
                'L' => {
                    adjacent_push(nodes, pos, 0);
                    adjacent_push(nodes, pos, 1);
                }
                'J' => {
                    adjacent_push(nodes, pos, 0);
                    adjacent_push(nodes, pos, 3);
                }
                '7' => {
                    adjacent_push(nodes, pos, 3);
                    adjacent_push(nodes, pos, 2);
                }
                'F' => {
                    adjacent_push(nodes, pos, 1);
                    adjacent_push(nodes, pos, 2);
                }
                'S' => {}
                _ => unreachable!(),
            }
        }
    }
}

fn travel_map(nodes: &[Vec<Node>], start: NodePos) -> u32 {
    let mut prev_pos = start;

    let mut pos = start;

    let mut len = 0;
    loop {
        let n = &nodes[pos.1 as usize][pos.0 as usize];
        let mut found = false;
        for p in &n.adjacent {
            let adjacent_node = &nodes[p.1 as usize][p.0 as usize];
            if *p != prev_pos && (pos == start || adjacent_node.adjacent.contains(&pos)) {
                prev_pos = pos;
                pos = *p;
                found = true;
                break;
            }
        }
        if !found {
            break;
        }
        len += 1;
    }
    (len + 1) / 2
}

fn proc_1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let (start, mut nodes) = analyze(data);
    build_adjacency_list(&mut nodes);

    travel_map(&nodes, start)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = ".S-7.\n";
        let (input, data) = parse_line(&data).unwrap();
        assert!(input.is_empty());
        assert_eq!(data, vec!['.', 'S', '-', '7', '.',]);
    }

    #[test]
    fn test_parse_file() {
        let data = fs::read_to_string("data/test_2.txt").unwrap();
        let (input, _data) = parse(&data).unwrap();
        assert!(input.is_empty());
    }

    #[test]
    fn test_analyze() {
        let data = fs::read_to_string("data/test_1.txt").unwrap();
        let (_, data) = parse(&data).unwrap();
        let (start, mut nodes) = analyze(data);
        build_adjacency_list(&mut nodes);
        let len = travel_map(&nodes, start);
        assert_eq!(len, 4);
    }
}
