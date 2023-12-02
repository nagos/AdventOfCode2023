use anyhow::Result;
use nom::bytes::complete::tag;
use nom::character::complete::{alpha1, digit1, space1};
use nom::combinator::map_res;
use nom::multi::separated_list0;
use nom::IResult;
use std::str::FromStr;

use crate::game::{CubeColor, Game, GameSet};

fn cube_convert(cubes: Vec<(u32, CubeColor)>) -> Result<GameSet> {
    let mut ret = GameSet::default();

    for cube in cubes {
        let color = cube.1;
        let c = match color {
            CubeColor::Red => &mut ret.red,
            CubeColor::Green => &mut ret.green,
            CubeColor::Blue => &mut ret.blue,
        };
        *c += cube.0;
    }
    Ok(ret)
}

fn cube_parser(input: &str) -> IResult<&str, (u32, CubeColor)> {
    let (input, count) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = space1(input)?;
    let (input, color) = map_res(alpha1, CubeColor::from_str)(input)?;

    Ok((input, (count, color)))
}

fn game_id_parser(input: &str) -> IResult<&str, u32> {
    let (input, _) = tag("Game ")(input)?;
    let (input, game_id) = map_res(digit1, |s: &str| s.parse::<u32>())(input)?;
    let (input, _) = tag(": ")(input)?;

    Ok((input, game_id))
}

fn game_set_parser(input: &str) -> IResult<&str, GameSet> {
    map_res(separated_list0(tag(", "), cube_parser), cube_convert)(input)
}

fn game_line(input: &str) -> IResult<&str, (u32, Vec<GameSet>)> {
    let (input, game_id) = game_id_parser(input)?;
    let (input, data) = separated_list0(tag("; "), game_set_parser)(input)?;

    Ok((input, (game_id, data)))
}

pub fn parse_game(input: &str) -> Result<Game> {
    let (_, (id, sets)) = game_line(input).map_err(|e| e.to_owned())?;

    Ok(Game { id, sets })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_test() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = parse_game(data).unwrap();
        assert_eq!(game.id, 1);
        let max_set = game.max_set();
        assert_eq!(max_set.red, 4);
        assert_eq!(max_set.green, 2);
        assert_eq!(max_set.blue, 6);
        assert!(game.check(&GameSet {
            red: 4,
            green: 2,
            blue: 6
        }));
        assert_eq!(game.power(), 48);
    }
}
