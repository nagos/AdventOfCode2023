use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    multi::many1,
    sequence::terminated,
    IResult,
};
use std::{collections::HashMap, fs};

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 8 part one: {part_one}");
}

type MapNode<'a> = (&'a str, &'a str, &'a str);

fn parse_direction(input: &str) -> IResult<&str, Vec<char>> {
    let (input, data) = terminated(many1(one_of("LR")), newline)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, data))
}

fn parse_node(input: &str) -> IResult<&str, MapNode> {
    let (input, node) = alpha1(input)?;
    let (input, _) = tag(" = (")(input)?;
    let (input, edge_l) = alpha1(input)?;
    let (input, _) = tag(", ")(input)?;
    let (input, edge_r) = alpha1(input)?;
    let (input, _) = tag(")\n")(input)?;

    Ok((input, (node, edge_l, edge_r)))
}

fn parse(input: &str) -> IResult<&str, (Vec<char>, Vec<MapNode>)> {
    let (input, directions) = parse_direction(input)?;
    let (input, nodes) = many1(parse_node)(input)?;

    Ok((input, (directions, nodes)))
}

fn calc(directions: Vec<char>, map_data: Vec<MapNode>) -> u32 {
    let mut map = HashMap::new();

    for m in map_data {
        map.insert(m.0, (m.1, m.2));
    }

    let mut curren_key = "AAA";
    let mut current_node = map.get(curren_key).unwrap();

    for i in 0.. {
        let idx = i % directions.len();
        let lr = directions[idx];

        if lr == 'L' {
            curren_key = current_node.0;
        } else {
            curren_key = current_node.1;
        }
        current_node = map.get(curren_key).unwrap();

        if curren_key == "ZZZ" {
            return i as u32 + 1;
        }
    }
    unreachable!()
}

fn proc_1(data: &str) -> u32 {
    let (_, (directions, nodes)) = parse(data).unwrap();
    calc(directions, nodes)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test_1.txt").unwrap();
        let (input, (directions, nodes)) = parse(&data).unwrap();
        assert!(input.is_empty());

        let res = calc(directions, nodes);
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
}
