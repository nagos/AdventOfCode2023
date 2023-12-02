pub mod game;
pub mod game_parser;

use game::GameSet;
use game_parser::parse_game;
use std::fs;

fn main() {
    let data = fs::read_to_string("data/day-2-input.txt").unwrap();

    let answer_one = proc(&data, &GameSet::build(12, 13, 14));
    println!("Day 2 part one: {answer_one}");

    let answer_two = proc_2(&data);
    println!("Day 2 part two: {answer_two}");
}

fn proc(data: &str, set_limit: &GameSet) -> u32 {
    data.lines().map(|l| proc_line(l, set_limit)).sum()
}

fn proc_line(line: &str, set_limit: &GameSet) -> u32 {
    let game = parse_game(line).unwrap();
    if game.check(set_limit) {
        game.id
    } else {
        0
    }
}

fn proc_2(data: &str) -> u32 {
    data.lines()
        .map(|line| parse_game(line).unwrap().power())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file() {
        let data = fs::read_to_string("data/day-2-test.txt").unwrap();
        let res = proc(&data, &GameSet::build(12, 13, 14));
        assert_eq!(res, 8);

        let res_2 = proc_2(&data);
        assert_eq!(res_2, 2286);
    }
}
