use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, newline, one_of},
    combinator::opt,
    multi::{many1, separated_list1},
    IResult,
};

use std::collections::HashMap;
use std::{fs, vec};

type InputLine<'a> = (Option<char>, &'a str, Vec<&'a str>);
type MemoryItem<'a> = (NodeType, Vec<&'a str>, Vec<SignalType>, Vec<&'a str>);

#[derive(Debug)]
enum NodeType {
    FlipFlop,
    Conjunction,
}

#[derive(Debug, PartialEq, Clone, Copy)]
enum SignalType {
    Low,
    High,
}

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 20 part one: {part_one}");
}

fn parse_line(input: &str) -> IResult<&str, InputLine> {
    let (input, block_type) = opt(one_of("%&"))(input)?;
    let (input, name) = alpha1(input)?;
    let (input, _) = tag(" -> ")(input)?;
    let (input, conn) = separated_list1(tag(", "), alpha1)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (block_type, name, conn)))
}

fn parse(input: &str) -> IResult<&str, Vec<InputLine>> {
    many1(parse_line)(input)
}

fn find_broadcaster<'a>(data: &'a Vec<InputLine>) -> &'a InputLine<'a> {
    for l in data {
        if l.1 == "broadcaster" {
            return l;
        }
    }
    unreachable!()
}

fn build_conn_map<'a>(data: &'a Vec<InputLine>) -> HashMap<&'a str, MemoryItem<'a>> {
    let mut m = HashMap::new();

    for (symbol, name, list) in data {
        if *name == "broadcaster" {
            continue;
        }
        let node_type = match symbol {
            Some('%') => NodeType::FlipFlop,
            Some('&') => NodeType::Conjunction,
            _ => unreachable!(),
        };
        m.insert(*name, (node_type, list.clone(), vec![], vec![]));
    }
    for (_, name, list) in data {
        for l in list {
            if !m.contains_key(l) {
                m.insert(l, (NodeType::FlipFlop, vec![], vec![], vec![]));
            }
            let el = m.get_mut(l).unwrap();
            el.3.push(*name);
            el.2.push(SignalType::Low);
        }
    }
    m
}

fn calc_1(m: &mut HashMap<&str, MemoryItem>, broadcaster: &InputLine) -> (u32, u32) {
    let mut count_low = 1;
    let mut count_high = 0;
    let mut signal_queue = vec![];
    for &s in &broadcaster.2 {
        signal_queue.insert(0, (s, SignalType::Low, broadcaster.1));
    }
    while let Some((next_node, signal, prev_node)) = signal_queue.pop() {
        if signal == SignalType::High {
            count_high += 1;
        } else {
            count_low += 1;
        }
        let node = m.get_mut(next_node).unwrap();
        match node.0 {
            NodeType::FlipFlop => {
                if signal == SignalType::Low {
                    if node.2[0] == SignalType::Low {
                        node.2[0] = SignalType::High;
                    } else {
                        node.2[0] = SignalType::Low;
                    }

                    for n in &node.1 {
                        signal_queue.insert(0, (n, node.2[0], next_node));
                    }
                }
            }
            NodeType::Conjunction => {
                let idx = node.3.iter().position(|&n| n == prev_node).unwrap();
                node.2[idx] = signal;

                let output_signal = if node.2.iter().all(|&s| s == SignalType::High) {
                    SignalType::Low
                } else {
                    SignalType::High
                };
                for n in &node.1 {
                    signal_queue.insert(0, (n, output_signal, next_node));
                }
            }
        }
    }
    (count_low, count_high)
}

fn proc_1(data: &str) -> u32 {
    let (_, data) = parse(data).unwrap();

    let mut count_low = 0;
    let mut count_high = 0;

    let broadcaster = find_broadcaster(&data);
    let mut m = build_conn_map(&data);

    for _ in 0..1000 {
        let (res_low, res_high) = calc_1(&mut m, broadcaster);
        count_low += res_low;
        count_high += res_high;
    }

    count_low * count_high
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test1.txt").unwrap();
        let (input, _data) = parse(&data).unwrap();
        assert!(input.is_empty());
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test1.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 32000000);
    }

    #[test]
    fn test_proc_2() {
        let data = fs::read_to_string("data/test2.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 11687500);
    }
}
