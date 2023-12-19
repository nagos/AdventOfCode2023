use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline, one_of},
    combinator::map_res,
    multi::{many1, separated_list1},
    IResult,
};

use std::collections::HashMap;
use std::fs;

type PartRule<'a> = (&'a str, char, u32, &'a str);
type PartWorkfloq<'a> = (&'a str, Vec<PartRule<'a>>, &'a str);
type PartData = (u32, u32, u32, u32);

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 19 part one: {part_one}");
}

fn digit1_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn parse_rule(input: &str) -> IResult<&str, PartRule> {
    let (input, param) = alpha1(input)?;
    let (input, cmp) = one_of("<>")(input)?;
    let (input, value) = digit1_u32(input)?;
    let (input, _) = tag(":")(input)?;
    let (input, next_rule) = alpha1(input)?;
    Ok((input, (param, cmp, value, next_rule)))
}

fn parse_rule_line(input: &str) -> IResult<&str, PartWorkfloq> {
    let (input, rule) = alpha1(input)?;
    let (input, _) = tag("{")(input)?;
    let (input, data) = separated_list1(tag(","), parse_rule)(input)?;
    let (input, _) = tag(",")(input)?;
    let (input, last_rule) = alpha1(input)?;
    let (input, _) = tag("}\n")(input)?;

    Ok((input, (rule, data, last_rule)))
}

fn parse_part_line(input: &str) -> IResult<&str, PartData> {
    let (input, _) = tag("{x=")(input)?;
    let (input, data_x) = digit1_u32(input)?;
    let (input, _) = tag(",m=")(input)?;
    let (input, data_m) = digit1_u32(input)?;
    let (input, _) = tag(",a=")(input)?;
    let (input, data_a) = digit1_u32(input)?;
    let (input, _) = tag(",s=")(input)?;
    let (input, data_s) = digit1_u32(input)?;
    let (input, _) = tag("}\n")(input)?;

    Ok((input, (data_x, data_m, data_a, data_s)))
}

fn parse(input: &str) -> IResult<&str, (Vec<PartWorkfloq>, Vec<PartData>)> {
    let (input, rules) = many1(parse_rule_line)(input)?;
    let (input, _) = newline(input)?;
    let (input, parts) = many1(parse_part_line)(input)?;

    Ok((input, (rules, parts)))
}

fn build_map(rules: Vec<PartWorkfloq>) -> HashMap<String, (Vec<PartRule>, &str)> {
    let mut ret = HashMap::new();
    for (workflow, part_rule, last_rule) in rules {
        ret.insert(workflow.to_string(), (part_rule, last_rule));
    }

    ret
}

fn process_part(part: PartData, rule: &(Vec<PartRule>, &str)) -> String {
    let (rules, last_rule) = rule;
    for &(param, op, value, workflow) in rules {
        let part_value = match param {
            "x" => part.0,
            "m" => part.1,
            "a" => part.2,
            "s" => part.3,
            _ => unreachable!(),
        };
        let res = match op {
            '>' => part_value > value,
            '<' => part_value < value,
            _ => unreachable!(),
        };
        if res {
            return workflow.to_string();
        }
    }
    last_rule.to_string()
}

fn proc_1(data: &str) -> u32 {
    let (_, (rules, parts)) = parse(data).unwrap();

    let map = build_map(rules);
    let mut ret = 0;

    for part in parts {
        let mut workflow = String::from("in");

        loop {
            let workflow_rule = map.get(&workflow).unwrap();

            let next_workflow = process_part(part, workflow_rule);

            workflow = next_workflow;

            if workflow == "R" {
                break;
            }
            if workflow == "A" {
                ret += part.0 + part.1 + part.2 + part.3;
                break;
            }
        }
    }

    ret
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_parse() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, _data) = parse(&data).unwrap();
        dbg!(input, _data);
        assert!(input.is_empty());
    }

    #[test]
    fn test_calc1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 19114);
    }
}
