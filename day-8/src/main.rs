use nom::{
    bytes::complete::tag,
    character::complete::{alphanumeric1, newline, one_of},
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 8 part one: {part_one}");

    let part_two = proc_2(&data);
    println!("Day 8 part two: {part_two}");
}

type MapNode<'a> = (&'a str, &'a str, &'a str);

fn parse_direction(input: &str) -> IResult<&str, Vec<char>> {
    let (input, data) = terminated(many1(one_of("LR")), newline)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn parse_node(input: &str) -> IResult<&str, MapNode> {
    let (input, node) = alphanumeric1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, edge_l) = alphanumeric1(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, edge_r) = alphanumeric1(input)?;
    let (input, _) = tag(")\n")(input)?;

    Ok((input, (node, edge_l, edge_r)))
}

fn parse(input: &str) -> IResult<&str, (Vec<char>, Vec<MapNode>)> {
    let (input, directions) = parse_direction(input)?;
    let (input, nodes) = many1(parse_node)(input)?;

    Ok((input, (directions, nodes)))
}

fn build_map(map_data: Vec<MapNode>) -> HashMap<&str, (&str, &str)> {
    let mut map = HashMap::new();

    for m in map_data {
        map.insert(m.0, (m.1, m.2));
    }

    map
}

fn calc_1(directions: Vec<char>, map: HashMap<&str, (&str, &str)>) -> u32 {
    let mut curren_key = "AAA";

    for i in 0.. {
        let idx = i % directions.len();
        let lr = directions[idx];

        curren_key = node_traverse(lr, curren_key, &map);

        if curren_key == "ZZZ" {
            return i as u32 + 1;
        }
    }
    unreachable!()
}

fn proc_1(data: &str) -> u32 {
    let (_, (directions, nodes)) = parse(data).unwrap();
    let map = build_map(nodes);
    calc_1(directions, map)
}

fn find_starting_nodes<'a>(map: &HashMap<&'a str, (&'a str, &'a str)>) -> Vec<&'a str> {
    let mut nodes = vec![];
    for n in map.keys() {
        if n.ends_with('A') {
            nodes.push(*n);
        }
    }

    nodes
}

fn node_traverse<'a>(
    lr: char,
    node: &'a str,
    map: &HashMap<&'a str, (&'a str, &'a str)>,
) -> &'a str {
    let current_node = map.get(node).unwrap();

    if lr == 'L' {
        current_node.0
    } else {
        current_node.1
    }
}

fn calc_2(directions: Vec<char>, map: HashMap<&str, (&str, &str)>) -> u32 {
    let mut path_nodes = find_starting_nodes(&map);

    for i in 0.. {
        let idx = i % directions.len();
        let lr = directions[idx];

        path_nodes = path_nodes
            .iter()
            .map(|node| node_traverse(lr, node, &map))
            .collect();

        let end = path_nodes.iter().map(|n| n.ends_with('Z')).all(|end| end);
        if end {
            return i as u32 + 1;
        }
    }
    unreachable!()
}

fn proc_2(data: &str) -> u32 {
    let (_, (directions, nodes)) = parse(data).unwrap();
    let map = build_map(nodes);
    calc_2(directions, map)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test_1.txt").unwrap();
        let (input, (directions, nodes)) = parse(&data).unwrap();
        assert!(input.is_empty());

        let map = build_map(nodes);
        let res = calc_1(directions, map);
        dbg!(res);
    }

    #[test]
    fn test_parse_direction() {
        let data = "RL\n\n";
        let (input, data) = parse_direction(&data).unwrap();
        assert!(input.is_empty());
        assert_eq!(data, ['R', 'L']);
    }

    #[test]
    fn test_parse_node() {
        let data = "AAA = (BBB, CCC)\n";
        let (input, data) = parse_node(&data).unwrap();
        assert!(input.is_empty());
        assert_eq!(data.0, "AAA");
        assert_eq!(data.1, "BBB");
        assert_eq!(data.2, "CCC");
    }

    #[test]
    fn test_calc_1() {
        let data_1 = fs::read_to_string("data/test_1.txt").unwrap();
        let res = proc_1(&data_1);
        assert_eq!(res, 2);

        let data_2 = fs::read_to_string("data/test_2.txt").unwrap();
        let res = proc_1(&data_2);
        assert_eq!(res, 6);
    }

    #[test]
    fn test_calc_2() {
        let data = fs::read_to_string("data/test_3.txt").unwrap();
        let res = proc_2(&data);
        assert_eq!(res, 6);
    }
}
