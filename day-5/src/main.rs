use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, not_line_ending, space1},
    combinator::map_res,
    multi::{many0, many1, separated_list1},
    IResult,
};

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc_1(&data);
    println!("Day 5 part one: {part_one}");

    let part_two = proc_2(&data);
    println!("Day 5 part two: {part_two}");
}

type TableItem = (u32, u32, u32);
type MapBlock = Vec<TableItem>;
type SeedRange = (u32, u32);

fn digit1_u32(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}

fn seeds_parser(input: &str) -> IResult<&str, Vec<u32>> {
    let (input, _) = tag("seeds: ")(input)?;
    let (input, seeds) = separated_list1(space1, digit1_u32)(input)?;
    let (input, _) = many1(newline)(input)?;

    Ok((input, seeds))
}

fn map_item_parser(input: &str) -> IResult<&str, TableItem> {
    let (input, data) = separated_list1(space1, digit1_u32)(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (data[0], data[1], data[2])))
}

fn block_parser(input: &str) -> IResult<&str, MapBlock> {
    let (input, _) = not_line_ending(input)?;
    let (input, _) = newline(input)?;
    let (input, data) = many1(map_item_parser)(input)?;
    let (input, _) = many0(newline)(input)?;

    Ok((input, data))
}

fn map_parser(input: &str) -> IResult<&str, (Vec<u32>, Vec<MapBlock>)> {
    let (input, seeds) = seeds_parser(input)?;
    let (input, blocks) = many1(block_parser)(input)?;

    Ok((input, (seeds, blocks)))
}

// part one

fn convert_item(item: u32, rule: TableItem) -> Option<u32> {
    let (dst_start, src_start, len) = rule;

    // Trick to get around integer overflow
    if item >= src_start && item - src_start < len {
        Some(item - src_start + dst_start)
    } else {
        None
    }
}

fn apply_transform(mut seed: u32, blocks: &Vec<Vec<TableItem>>) -> u32 {
    for block in blocks {
        for rule in block {
            if let Some(res) = convert_item(seed, *rule) {
                seed = res;
                break;
            }
        }
    }
    seed
}

fn proc_1(data: &str) -> u32 {
    let (_, (seeds, blocks)) = map_parser(data).unwrap();

    seeds
        .into_iter()
        .map(|s| apply_transform(s, &blocks))
        .min()
        .unwrap()
}

// part two

fn split_range(seed_range: SeedRange, table: &TableItem) -> (Option<SeedRange>, Vec<SeedRange>) {
    let src_range = (table.1, table.1 + (table.2 - 1));
    let is_intersect = seed_range.0 <= src_range.1 && seed_range.1 >= src_range.0;

    let mut splits = vec![];
    let intersect = if is_intersect {
        if seed_range.0 < src_range.0 {
            splits.push((seed_range.0, src_range.0 - 1));
        }

        if seed_range.1 > src_range.1 {
            splits.push((src_range.1 + 1, seed_range.1));
        }

        Some((seed_range.0.max(src_range.0), seed_range.1.min(src_range.1)))
    } else {
        None
    };

    (intersect, splits)
}

fn convert_seed_range(seed_range: SeedRange, blocks: &Vec<Vec<TableItem>>) -> Vec<SeedRange> {
    let mut ret = vec![seed_range];
    for block in blocks {
        let mut tmp = ret.clone();
        ret = vec![];
        while let Some(i) = tmp.pop() {
            let mut split = false;
            for rule in block {
                if let (Some(intersect), splits) = split_range(i, rule) {
                    ret.push((intersect.0 - rule.1 + rule.0, intersect.1 - rule.1 + rule.0));
                    tmp.extend(splits);
                    split = true;
                    break;
                }
            }
            if !split {
                ret.push(i);
            }
        }
    }
    ret
}

fn proc_2(data: &str) -> u32 {
    let (_, (seeds, blocks)) = map_parser(data).unwrap();

    seeds
        .chunks(2)
        .map(|c| (c[0], c[0] + c[1] - 1))
        .flat_map(|s| convert_seed_range(s, &blocks))
        .map(|r| r.0)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::vec;

    #[test]
    fn test_seeds_parser() {
        let data = "seeds: 79 14 55 13\n\n";

        let (input, seeds) = seeds_parser(data).unwrap();
        assert!(input.is_empty());
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_block_parser() {
        let data = "seed-to-soil map:\n50 98 2\n52 50 48\n\n";
        let (input, _) = block_parser(data).unwrap();
        assert!(input.is_empty());
    }

    #[test]
    fn test_map_item_parser() {
        let data = "49 53 8\n";
        let (input, values) = map_item_parser(data).unwrap();
        assert_eq!(values, (49, 53, 8));
        assert!(input.is_empty());
    }

    #[test]
    fn test_map_parser() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (intput, _) = map_parser(&data).unwrap();
        assert!(intput.is_empty());
    }

    #[test]
    fn test_u32_parser() {
        let data = "123";
        let (intput, value) = digit1_u32(data).unwrap();
        assert!(intput.is_empty());
        assert_eq!(value, 123);
    }

    #[test]
    fn test_convert_item() {
        let value = 79;
        let table_1 = (50, 98, 2);
        let table_2 = (52, 50, 48);
        let res = convert_item(value, table_1);
        assert_eq!(res, None);
        let res = convert_item(value, table_2);
        assert_eq!(res, Some(81));

        let res = convert_item(69, (0, 69, 2));
        assert_eq!(res, Some(0));
        let res = convert_item(70, (0, 69, 2));
        assert_eq!(res, Some(1));

        let res = convert_item(71, (0, 69, 2));
        assert_eq!(res, None);

        let res = convert_item(68, (0, 69, 2));
        assert_eq!(res, None);
    }

    #[test]
    fn test_proc_1() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_1(&data);
        assert_eq!(res, 35);
    }

    #[test]
    fn test_proc_2() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let res = proc_2(&data);
        assert_eq!(res, 46);
    }

    #[test]
    fn test_range_split() {
        let r = (79, 79 + 14 - 1);
        let table = (50, 98, 2);
        let res = split_range(r, &table);
        assert_eq!(res, (None, vec![]));

        let r = (79, 79 + 14 - 1);
        let table = (52, 50, 48);
        let res = split_range(r, &table);
        assert_eq!(res, (Some((79, 92)), vec![]));

        let r = (55, 55 + 13 - 1);
        let table = (50, 98, 2);
        let _res = split_range(r, &table);

        let table = (52, 50, 48);
        let _res = split_range(r, &table);

        // 10    20
        //    15    24
        let res = split_range((10, 20), &(0, 15, 10));
        assert_eq!(res, (Some((15, 20)), vec![(10, 14)]));

        // 10       25
        //    15 24
        let res = split_range((10, 25), &(0, 15, 10));
        assert_eq!(res, (Some((15, 24)), vec![(10, 14), (25, 25)]));

        // 10    24
        //    15 24
        let res = split_range((10, 24), &(0, 15, 10));
        assert_eq!(res, (Some((15, 24)), vec![(10, 14)]));

        // 15 24
        // 15 24
        let res = split_range((15, 24), &(0, 15, 10));
        assert_eq!(res, (Some((15, 24)), vec![]));

        // 15 24
        // 15     34
        let res = split_range((15, 24), &(0, 15, 20));
        assert_eq!(res, (Some((15, 24)), vec![]));

        // 15    34
        // 15 24
        let res = split_range((15, 34), &(0, 15, 10));
        assert_eq!(res, (Some((15, 24)), vec![(25, 34)]));

        // 15
        // 15 24
        let res = split_range((15, 15), &(0, 15, 10));
        assert_eq!(res, (Some((15, 15)), vec![]));

        //    24
        // 15 24
        let res = split_range((24, 24), &(0, 15, 10));
        assert_eq!(res, (Some((24, 24)), vec![]));

        // 0       30
        //   15 24
        let res = split_range((0, 30), &(0, 15, 10));
        assert_eq!(res, (Some((15, 24)), vec![(0, 14), (25, 30)]));
    }

    #[test]
    fn test_sample() {
        let r = (82, 82);
        let res = convert_seed_range(r, &vec![vec![(50, 98, 2), (52, 50, 48)]]);
        assert_eq!(res, vec![(84, 84)]);
        let res = convert_seed_range(res[0], &vec![vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]]);
        assert_eq!(res, vec![(84, 84)]);
        let res = convert_seed_range(
            res[0],
            &vec![vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]],
        );
        assert_eq!(res, vec![(84, 84)]);
        let res = convert_seed_range(res[0], &vec![vec![(88, 18, 7), (18, 25, 70)]]);
        assert_eq!(res, vec![(77, 77)]);
        let res = convert_seed_range(
            res[0],
            &vec![vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)]],
        );
        assert_eq!(res, vec![(45, 45)]);
        let res = convert_seed_range(res[0], &vec![vec![(0, 69, 1), (1, 0, 69)]]);
        assert_eq!(res, vec![(46, 46)]);
        let res = convert_seed_range((0, 9), &vec![vec![(20, 0, 5), (30, 5, 5)]]);
        assert_eq!(res, vec![(20, 24), (30, 34)]);
    }
}
