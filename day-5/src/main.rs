use std::fs;

use nom::{
    bytes::complete::tag,
    character::complete::{digit1, newline, not_line_ending, space1},
    combinator::map_res,
    multi::{count, separated_list1},
    sequence::{pair, terminated},
    IResult,
};

fn main() {
    let data = fs::read_to_string("data/input.txt").unwrap();
    let part_one = proc(&data);
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
    let (input, _) = count(newline, 2)(input)?;

    Ok((input, seeds))
}

fn map_item_parser(input: &str) -> IResult<&str, TableItem> {
    let (input, data) = separated_list1(space1, digit1_u32)(input)?;

    Ok((input, (data[0], data[1], data[2])))
}

fn block_parser(input: &str) -> IResult<&str, MapBlock> {
    let (input, _) = terminated(not_line_ending, newline)(input)?;
    let (input, data) = separated_list1(newline, map_item_parser)(input)?;

    Ok((input, data))
}

fn block_list_parser(input: &str) -> IResult<&str, Vec<MapBlock>> {
    let (input, data) = separated_list1(pair(newline, newline), block_parser)(input)?;
    Ok((input, data))
}

fn map_parser(input: &str) -> IResult<&str, (Vec<u32>, Vec<MapBlock>)> {
    let (input, seeds) = seeds_parser(input)?;
    let (input, blocks) = block_list_parser(input)?;
    let (input, _) = newline(input)?;

    Ok((input, (seeds, blocks)))
}

fn convert_item(item: u32, table: TableItem) -> Option<u32> {
    let (dst_start, src_start, len) = table;

    // Trick to get around integer overflow
    if item >= src_start {
        let offset = item - src_start;
        if offset < len {
            Some(dst_start + offset)
        } else {
            None
        }
    } else {
        None
    }
}

fn match_items(items: Vec<u32>, table: Vec<TableItem>) -> Vec<u32> {
    let mut ret = items.clone();
    for i in &mut ret {
        for t in &table {
            if let Some(res) = convert_item(*i, *t) {
                *i = res;
                break;
            }
        }
    }

    ret
}

fn convert_seeds(seeds: Vec<u32>, blocks: Vec<MapBlock>) -> Vec<u32> {
    let mut items = seeds;
    for block in blocks {
        items = match_items(items, block);
    }

    items
}

fn proc(data: &str) -> u32 {
    let (_, (seeds, blocks)) = map_parser(data).unwrap();
    let res = convert_seeds(seeds, blocks);
    *res.iter().min().unwrap()
}

fn split_range(
    seed_range: SeedRange,
    table: &TableItem,
) -> (Option<SeedRange>, Option<SeedRange>, Option<SeedRange>) {
    let src_range = (table.1, table.1 + (table.2 - 1));
    let is_intersect = seed_range.0 <= src_range.1 && seed_range.1 >= src_range.0;

    let left;
    let center;
    let right;
    if is_intersect {
        if seed_range.0 < src_range.0 {
            left = Some((seed_range.0, src_range.0 - 1));
        } else {
            left = None
        }

        if seed_range.1 > src_range.1 {
            right = Some((src_range.1 + 1, seed_range.1));
        } else {
            right = None;
        }

        center = Some((seed_range.0.max(src_range.0), seed_range.1.min(src_range.1)));
    } else {
        left = None;
        center = None;
        right = None;
    }

    (left, center, right)
}

fn convert_seed_range(seed_range: Vec<SeedRange>, table: &Vec<TableItem>) -> Vec<SeedRange> {
    let ret = seed_range.clone();
    let mut tmp = vec![];
    for i in ret {
        let mut split = false;
        for t in table {
            let (left, center, right) = split_range(i, t);
            if let Some(x) = left {
                tmp.push(x);
            }
            if let Some(x) = right {
                tmp.push(x);
            }
            if let Some(x) = center {
                let a = x.0 - t.1 + t.0;
                let b = x.1 - t.1 + t.0;
                tmp.push((a, b));
                split = true;
                break;
            }
        }
        if !split {
            tmp.push(i);
        }
    }
    tmp
}

fn proc_2(data: &str) -> u32 {
    let (_, (seeds, blocks)) = map_parser(data).unwrap();

    let mut seed_ranges = vec![];

    for i in 0..seeds.len() / 2 {
        seed_ranges.push((seeds[i * 2], seeds[i * 2] + seeds[i * 2 + 1] - 1));
    }

    let mut tmp = seed_ranges.clone();

    for block in &blocks {
        tmp = convert_seed_range(tmp, block);
    }
    tmp.iter().map(|e| e.0).min().unwrap()
}

#[cfg(test)]
mod tests {
    use std::vec;
    use super::*;

    #[test]
    fn test_seeds_parser() {
        let data = "seeds: 79 14 55 13\n\n";

        let (input, seeds) = seeds_parser(data).unwrap();
        assert!(input.is_empty());
        assert_eq!(seeds, vec![79, 14, 55, 13]);
    }

    #[test]
    fn test_parser_file() {
        let data = fs::read_to_string("data/test.txt").unwrap();
        let (input, (seeds, blocks)) = map_parser(&data).unwrap();
        assert!(input.is_empty());
        assert_eq!(seeds, vec![79, 14, 55, 13]);
        assert_eq!(blocks.len(), 7);
        let res = convert_seeds(seeds, blocks);
        assert_eq!(res, vec![82, 43, 86, 35]);
    }

    #[test]
    fn test_block_parser() {
        let data = "seed-to-soil map:\n50 98 2\n52 50 48";
        let (input, _) = block_parser(data).unwrap();
        assert!(input.is_empty());
    }

    #[test]
    fn test_map_item_parser() {
        let data = "49 53 8";
        let (input, values) = map_item_parser(data).unwrap();
        assert_eq!(values, (49, 53, 8));
        assert!(input.is_empty());
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
        assert_eq!(res, (None, None, None));

        let r = (79, 79 + 14 - 1);
        let table = (52, 50, 48);
        let res = split_range(r, &table);
        assert_eq!(res, (None, Some((79, 92)), None));

        let r = (55, 55 + 13 - 1);
        let table = (50, 98, 2);
        let _res = split_range(r, &table);

        let table = (52, 50, 48);
        let _res = split_range(r, &table);

        // 10    20
        //    15    24
        let res = split_range((10, 20), &(0, 15, 10));
        assert_eq!(res, (Some((10, 14)), Some((15, 20)), None));

        // 10       25
        //    15 24
        let res = split_range((10, 25), &(0, 15, 10));
        assert_eq!(res, (Some((10, 14)), Some((15, 24)), Some((25, 25))));

        // 10    24
        //    15 24
        let res = split_range((10, 24), &(0, 15, 10));
        assert_eq!(res, (Some((10, 14)), Some((15, 24)), None));

        // 15 24
        // 15 24
        let res = split_range((15, 24), &(0, 15, 10));
        assert_eq!(res, (None, Some((15, 24)), None));

        // 15 24
        // 15     34
        let res = split_range((15, 24), &(0, 15, 20));
        assert_eq!(res, (None, Some((15, 24)), None));

        // 15    34
        // 15 24
        let res = split_range((15, 34), &(0, 15, 10));
        assert_eq!(res, (None, Some((15, 24)), Some((25, 34))));

        // 15 
        // 15 24
        let res = split_range((15, 15), &(0, 15, 10));
        assert_eq!(res, (None, Some((15, 15)), None));

        //    24 
        // 15 24
        let res = split_range((24, 24), &(0, 15, 10));
        assert_eq!(res, (None, Some((24, 24)), None));

        // 0       30
        //   15 24
        let res = split_range((0, 30), &(0, 15, 10));
        assert_eq!(res, (Some((0, 14)), Some((15, 24)), Some((25, 30))));

    }

    #[test]
    fn test_sample() {
        let r = (82, 82);
        let res = convert_seed_range(vec![r], &vec![(50, 98, 2), (52, 50, 48)]);
        assert_eq!(res, vec![(84, 84)]);
        let res = convert_seed_range(res, &vec![(0, 15, 37), (37, 52, 2), (39, 0, 15)]);
        assert_eq!(res, vec![(84, 84)]);
        let res = convert_seed_range(res, &vec![(49, 53, 8), (0, 11, 42), (42, 0, 7), (57, 7, 4)]);
        assert_eq!(res, vec![(84, 84)]);
        let res = convert_seed_range(res, &vec![(88, 18, 7), (18, 25, 70)]);
        assert_eq!(res, vec![(77, 77)]);
        let res = convert_seed_range(res, &vec![(45, 77, 23), (81, 45, 19), (68, 64, 13)]);
        assert_eq!(res, vec![(45, 45)]);
        let res = convert_seed_range(res, &vec![(0, 69, 1), (1, 0, 69)]);
        assert_eq!(res, vec![(46, 46)]);
    }
}
