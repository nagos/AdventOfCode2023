use std::fs;

#[derive(Debug)]
struct Game {
    id: u32,
    red: u32,
    green: u32,
    blue: u32,
}

impl Game {
    fn parse(s: &str) -> Self {
        let (game, game_data) = s.split_once(": ").unwrap();
        let id = game.strip_prefix("Game ").unwrap().parse::<u32>().unwrap();
        let sets = game_data.split("; ").map(GameSet::parse);
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for set in sets {
            red = red.max(set.red);
            green = green.max(set.green);
            blue = blue.max(set.blue);
        }
        Game {
            id,
            red,
            green,
            blue,
        }
    }

    fn check(&self, red: u32, green: u32, blue: u32) -> bool {
        self.red <= red && self.green <= green && self.blue <= blue
    }

    fn power(&self) -> u32 {
        self.red * self.green * self.blue
    }
}

#[derive(Debug)]
struct GameSet {
    red: u32,
    green: u32,
    blue: u32,
}

impl GameSet {
    fn parse(s: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let cubes = s.split(", ");
        for cube in cubes {
            let set = cube.split(' ').collect::<Vec<&str>>();
            let count = set[0].parse::<u32>().unwrap();
            let color = set[1];
            match color {
                "red" => red += count,
                "green" => green += count,
                "blue" => blue += count,
                &_ => unreachable!(),
            }
        }
        GameSet { red, green, blue }
    }
}

fn main() {
    let data = fs::read_to_string("data/day-2-input.txt").unwrap();

    let answer_one = proc(&data, 12, 13, 14);
    println!("Day 2 part one: {answer_one}");

    let answer_two = proc_2(&data);
    println!("Day 2 part two: {answer_two}");
}

fn proc(data: &str, red: u32, green: u32, blue: u32) -> u32 {
    data.lines().map(|l| proc_line(l, red, green, blue)).sum()
}

fn proc_line(line: &str, red: u32, green: u32, blue: u32) -> u32 {
    let game = Game::parse(line);
    if game.check(red, green, blue) {
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
        assert_eq!(game.red, 4);
        assert_eq!(game.green, 2);
        assert_eq!(game.blue, 6);
        assert!(game.check(4, 2, 6));
        assert_eq!(game.power(), 48);
    }

    #[test]
    fn test_file() {
        let data = fs::read_to_string("data/day-2-test.txt").unwrap();
        let res = proc(&data, 12, 13, 14);
        assert_eq!(res, 8);

        let res_2 = proc_2(&data);
        assert_eq!(res_2, 2286);
    }
}
