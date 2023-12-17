use std::{collections::HashSet, str::FromStr};

use eyre::{Context, ContextCompat};

#[derive(Debug, Clone)]
pub struct Card {
    pub id: u64,
    pub winning_numbers: HashSet<u64>,
    pub player_numbers: HashSet<u64>,
}

impl Card {
    /// Calculate how many of the player numbers are also in the winning numbers.
    /// This is the "point" value of this card.
    pub fn points(&self) -> u64 {
        self.player_numbers.intersection(&self.winning_numbers).count() as _
    }
}

impl FromStr for Card {
    type Err = eyre::Error;

    fn from_str(input: &str) -> eyre::Result<Self> {
        let (id_raw, body) = input.split_once(": ").context("malformed input")?;

        // get the card ID
        let id = id_raw
            .chars()
            .skip(4)
            .collect::<String>()
            .trim_start()
            .parse()
            .context(id_raw.to_string())?;

        // split the winning/player numbers
        let (winning_raw, player_raw) = body.split_once(" | ").context("malformed input")?;

        // parse winning numbers
        let winning_numbers = winning_raw
            .split_whitespace()
            .map(|n| Ok(n.parse()?))
            .collect::<eyre::Result<HashSet<_>>>()?;

        // parse player numbers
        let player_numbers = player_raw
            .split_whitespace()
            .map(|n| Ok(n.parse()?))
            .collect::<eyre::Result<HashSet<_>>>()?;

        Ok(Self {
            id,
            winning_numbers,
            player_numbers,
        })
    }
}

pub fn part_one(cards: &[Card]) -> eyre::Result<u64> {
    let mut acc = 0;

    for card in cards {
        acc += match card.points() {
            0 => 0,
            m => 2u64.pow(m as u32 - 1),
        };
    }

    Ok(acc)
}

pub fn part_two(cards: &[Card]) -> eyre::Result<u64> {
    let mut counts = cards.iter().map(|_| 1).collect::<Vec<u64>>();

    for (index, card) in cards.iter().enumerate() {
        for i in 0..card.points() as _ {
            counts[index + i + 1] += counts[index]
        }
    }

    Ok(counts.iter().sum())
}

pub fn main() -> eyre::Result<()> {
    let input = crate::util::get_input(2023, 4)?;
    let cards = input
        .lines()
        .map(|line| line.parse())
        .collect::<eyre::Result<Vec<_>>>()?;

    tracing::info!("Part One: {}", part_one(&cards)?);
    tracing::info!("Part Two: {}", part_two(&cards)?);

    Ok(())
}
