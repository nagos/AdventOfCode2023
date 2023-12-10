use nom::{
    character::complete::{newline, one_of},
    multi::many1,
    sequence::terminated,
    IResult,
};

use std::{fs, vec};

type NodePos = (usize, usize);

#[derive(Debug)]
struct Node {
    adjacent: Vec<NodePos>,
    value: char,
    part_of_loop: bool,
    step: u32,
}

impl Node {
    fn build(value: char) -> Self {
        Node {
            adjacent: vec![],
            value,
            part_of_loop: false,
            step: 0,
        }
    }
}

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 10 part one: {part_one}");

    let part_two = proc_2(&data);
    println!("Day 10 part two: {part_two}");
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
            let pos = (x, y);
            tmp.push(Node::build(*c));
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

    let width = nodes[0].len();
    let height = nodes.len();
    let (x, y) = (pos.0, pos.1);
    match dir {
        0 if y > 0 => nodes[y - 1][x].adjacent.push(pos),
        1 if x < width - 1 => nodes[y][x + 1].adjacent.push(pos),
        2 if y < height - 1 => nodes[y + 1][x].adjacent.push(pos),
        3 if x > 0 => nodes[y][x - 1].adjacent.push(pos),
        _ => {}
    };
}

fn build_adjacency_list(nodes: &mut Vec<Vec<Node>>) {
    let width = nodes[0].len();
    let height = nodes.len();

    for y in 0..height {
        for x in 0..width {
            let pos = (x, y);
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

fn travel_map(nodes: &mut [Vec<Node>], start: NodePos) -> u32 {
    let mut pos = start;

    let mut len = 0;
    let mut found = true;
    while found {
        let n = &mut nodes[pos.1][pos.0];
        found = false;
        n.part_of_loop = true;
        n.step = len;
        for p in n.adjacent.clone() {
            let adjacent_node = &nodes[p.1][p.0];
            if !adjacent_node.part_of_loop && (pos == start || adjacent_node.adjacent.contains(&pos)) {
                pos = p;
                found = true;
                break;
            }
        }
        len += 1;
    }
    len
}

fn find_inside(nodes: &mut Vec<Vec<Node>>, loop_size: u32) -> u32 {
    let mut ret = 0;
    let height = nodes.len();
    for (y, line) in nodes.iter().enumerate() {
        let mut cnt = 0;
        if y == height - 1 {
            continue;
        }
        for (x, node) in line.iter().enumerate() {
            let down_node = &nodes[y + 1][x];
            if node.part_of_loop {
                if down_node.part_of_loop {
                    let diff = (node.step + loop_size - down_node.step) % (loop_size);
                    if diff == 1 {
                        cnt += 1;
                    } else if diff == loop_size-1 {
                        cnt -= 1;
                    };
                }
            } else  if cnt != 0 {
                    ret += 1;
            }
        }
    }
    ret
}

fn proc_1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let (start, mut nodes) = analyze(data);
    build_adjacency_list(&mut nodes);

    travel_map(&mut nodes, start) / 2
}

fn proc_2(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();
    let (start, mut nodes) = analyze(data);
    build_adjacency_list(&mut nodes);

    let len = travel_map(&mut nodes, start);

    find_inside(&mut nodes, len)
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
        let len = (travel_map(&mut nodes, start)) / 2;
        assert_eq!(len, 4);
    }

    #[test]
    fn test_part_two() {
        let data = fs::read_to_string("data/test_5.txt").unwrap();
        let (_, data) = parse(&data).unwrap();
        let (start, mut nodes) = analyze(data);
        build_adjacency_list(&mut nodes);
        let len = travel_map(&mut nodes, start);
        let res = find_inside(&mut nodes, len);
        assert_eq!(res, 8);
    }
}
