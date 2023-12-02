use std::str::FromStr;

#[derive(Debug)]
pub enum CubeColor {
    Red,
    Green,
    Blue,
}

impl FromStr for CubeColor {
    type Err = anyhow::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "red" => Ok(CubeColor::Red),
            "green" => Ok(CubeColor::Green),
            "blue" => Ok(CubeColor::Blue),
            &_ => Err(anyhow::anyhow!("Bad color")),
        }
    }
}

#[derive(Debug)]
pub struct Game {
    pub id: u32,
    pub sets: Vec<GameSet>,
}

#[derive(Debug, Default)]
pub struct GameSet {
    pub red: u32,
    pub green: u32,
    pub blue: u32,
}

impl Game {
    pub fn max_set(&self) -> GameSet {
        let mut ret = GameSet::default();
        for set in &self.sets {
            ret.red = ret.red.max(set.red);
            ret.green = ret.green.max(set.green);
            ret.blue = ret.blue.max(set.blue);
        }

        ret
    }

    pub fn check(&self, set_limit: &GameSet) -> bool {
        let max_set = self.max_set();
        max_set.red <= set_limit.red
            && max_set.green <= set_limit.green
            && max_set.blue <= set_limit.blue
    }

    pub fn power(&self) -> u32 {
        let max_set = self.max_set();
        max_set.red * max_set.green * max_set.blue
    }
}

impl GameSet {
    pub fn build(red: u32, green: u32, blue: u32) -> Self {
        GameSet { red, green, blue }
    }
}
