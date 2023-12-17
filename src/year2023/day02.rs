use std::str::FromStr;

use eyre::ContextCompat;

#[derive(Debug, Default)]
pub struct Set {
    pub red: u64,
    pub green: u64,
    pub blue: u64,
}

impl Set {
    pub fn product(&self) -> u64 {
        self.red * self.green * self.blue
    }
}

impl FromStr for Set {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        let mut set = Self::default();
        for colour_raw in s.split(", ") {
            let (value, colour) = colour_raw
                .split_once(' ')
                .context(format!("colour `{colour_raw}` was malformed"))?;

            let value: u64 = value.parse()?;

            match colour {
                "red" => set.red += value,
                "green" => set.green += value,
                "blue" => set.blue += value,
                unknown => eyre::bail!("{unknown} is not a recognised colour"),
            }
        }
        Ok(set)
    }
}

#[derive(Debug, Default)]
pub struct Game {
    pub id: u64,
    pub sets: Vec<Set>,
}

impl FromStr for Game {
    type Err = eyre::Error;

    fn from_str(s: &str) -> eyre::Result<Self> {
        let (game_raw, sets_raw) = s.split_once(": ").context(format!("Malformed Game {s}"))?;

        let (_, id_raw) = game_raw.split_once(' ').context(format!("Malformed Game {s}"))?;

        let id = id_raw.parse()?;
        let sets = sets_raw
            .split("; ")
            .map(Set::from_str)
            .collect::<eyre::Result<Vec<_>>>()?;

        Ok(Self { id, sets })
    }
}

impl Game {
    pub fn max(&self) -> Set {
        self.sets.iter().fold(Set::default(), |mut total, set| {
            total.red = total.red.max(set.red);
            total.green = total.green.max(set.green);
            total.blue = total.blue.max(set.blue);

            total
        })
    }
}

pub fn part_one(games: &[Game]) -> eyre::Result<u64> {
    Ok(games
        .iter()
        .filter_map(|game| {
            game.sets
                .iter()
                .all(|set| set.red <= 12 && set.green <= 13 && set.blue <= 14)
                .then_some(game.id)
        })
        .sum())
}

pub fn part_two(games: &[Game]) -> eyre::Result<u64> {
    Ok(games.iter().map(|game| game.max().product()).sum())
}

pub fn main() -> eyre::Result<()> {
    let input = crate::util::get_input(2023, 2)?;
    let games = input.lines().map(Game::from_str).collect::<eyre::Result<Vec<_>>>()?;

    tracing::info!("Part One: {}", part_one(&games)?);
    tracing::info!("Part Two: {}", part_two(&games)?);

    Ok(())
}
