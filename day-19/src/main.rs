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
type PartRange = ((u32, u32), (u32, u32), (u32, u32), (u32, u32));

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 19 part one: {part_one}");
    let part_two = proc_2(&data);
    println!("Day 19 part two: {part_two}");
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
            workflow = process_part(part, workflow_rule);
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

fn extract_range<'a>(range: &'a mut PartRange, param: &str) -> &'a mut (u32, u32) {
    match param {
        "x" => &mut range.0,
        "m" => &mut range.1,
        "a" => &mut range.2,
        "s" => &mut range.3,
        _ => unreachable!(),
    }
}

fn apply_rule<'a>(mut range: PartRange, rule: &'a (Vec<PartRule>, &str)) -> Vec<(&'a str, PartRange)> {
    let (rules, last_rule) = rule;
    let mut ret = vec![];

    for &(param, op, value, workflow) in rules {
        let part_range = extract_range(&mut range, param);
        let value = if op == '>' { value + 1 } else { value };
        let mut range_left = if value > part_range.0 {
            Some((part_range.0, part_range.1.min(value - 1)))
        } else {
            None
        };
        let mut range_right = if value < part_range.1 {
            Some((part_range.0.max(value), part_range.1))
        } else {
            None
        };
        if op == '>' {
            std::mem::swap(&mut range_left, &mut range_right);
        }

        if let Some(x) = range_left {
            *extract_range(&mut range, param) = x;
            ret.push((workflow, range));
        }

        if let Some(x) = range_right {
            *extract_range(&mut range, param) = x;
        }
    }

    ret.push((last_rule, range));

    ret
}

fn mult_range(range: PartRange) -> u64 {
    (range.0 .1 + 1 - range.0 .0) as u64
        * (range.1 .1 + 1 - range.1 .0) as u64
        * (range.2 .1 + 1 - range.2 .0) as u64
        * (range.3 .1 + 1 - range.3 .0) as u64
}

fn proc_2(data: &str) -> u64 {
    let (_, (rules, _)) = parse(data).unwrap();

    let map = build_map(rules);
    let mut ret = 0;

    let start = ((1, 4000), (1, 4000), (1, 4000), (1, 4000));

    let mut ranges = vec![("in", start)];

    while let Some((workflow, range)) = ranges.pop() {
        let rule = map.get(&workflow.to_string()).unwrap();
        let new_ranges = apply_rule(range, rule);

        for (new_workflow, new_range) in new_ranges {
            if new_workflow == "A" {
                ret += mult_range(new_range);
            } else if new_workflow == "R" {
            } else {
                ranges.push((new_workflow, new_range));
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
        assert!(input.is_empty());
    }

    #[test]
    fn test_calc1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 19114);
    }

    #[test]
    fn test_proc2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_2(&data);
        assert_eq!(res, 167409079868000);
    }
}
