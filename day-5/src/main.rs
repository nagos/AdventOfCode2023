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
}

type TableItem = (u32, u32, u32);
type MapBlock = Vec<TableItem>;

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

#[cfg(test)]
mod tests {
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
    }
}
