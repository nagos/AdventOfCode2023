use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}

#[derive(Debug, Default)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn parse(s: &str) -> Self {
        let (game, game_data) = s.split_once(": ").unwrap();
        let id = game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
        let sets = game_data
            .split("; ")
            .map(GameSet::parse)
            .collect::<Vec<GameSet>>();
        Game { id, sets }
    }

    fn max_set(&self) -> GameSet {
        let mut ret = GameSet::default();
        for set in &self.sets {
            ret.red = ret.red.max(set.red);
            ret.green = ret.green.max(set.green);
            ret.blue = ret.blue.max(set.blue);
        }

        ret
    }

    fn check(&self, set_limit: &GameSet) -> bool {
        let max_set = self.max_set();
        max_set.red <= set_limit.red
            && max_set.green <= set_limit.green
            && max_set.blue <= set_limit.blue
    }

    fn power(&self) -> u32 {
        let max_set = self.max_set();
        max_set.red * max_set.green * max_set.blue
    }
}

impl GameSet {
    fn parse(s: &str) -> Self {
        let mut ret = GameSet::default();

        let cubes = s.split(", ");
        for cube in cubes {
            let set = cube.split(' ').collect::<Vec<&str>>();
            let count = set[0].parse::<u32>().unwrap();
            let color = set[1];
            let c = match color {
                "red" => &mut ret.red,
                "green" => &mut ret.green,
                "blue" => &mut ret.blue,
                &_ => unreachable!(),
            };
            *c += count;
        }
        ret
    }
}

fn main() {
    let data = fs::read_to_string("data/day-2-input.txt").unwrap();

    let answer_one = proc(
        &data,
        &GameSet {
            red: 12,
            green: 13,
            blue: 14,
        },
    );
    println!("Day 2 part one: {answer_one}");

    let answer_two = proc_2(&data);
    println!("Day 2 part two: {answer_two}");
}

fn proc(data: &str, set_limit: &GameSet) -> u32 {
    data.lines().map(|l| proc_line(l, set_limit)).sum()
}

fn proc_line(line: &str, set_limit: &GameSet) -> u32 {
    let game = Game::parse(line);
    if game.check(set_limit) {
        game.id
    } else {
        0
    }
}

fn proc_2(data: &str) -> u32 {
    data.lines().map(|line| Game::parse(line).power()).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn line_test() {
        let data = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let game = Game::parse(data);
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

    #[test]
    fn test_file() {
        let data = fs::read_to_string("data/day-2-test.txt").unwrap();
        let res = proc(
            &data,
            &GameSet {
                red: 12,
                green: 13,
                blue: 14,
            },
        );
        assert_eq!(res, 8);

        let res_2 = proc_2(&data);
        assert_eq!(res_2, 2286);
    }
}
