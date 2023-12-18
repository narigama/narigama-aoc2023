use eyre::ContextCompat;

#[derive(Debug)]
pub struct Race {
    pub time: i64,
    pub distance: i64,
}

impl Race {
    /// Provide a time that a boats button is held for. Returns a distance the
    /// boat will travel in the allotted race time.
    pub fn distance_travelled(&self, wait: i64) -> i64 {
        if !(1..self.distance).contains(&wait) {
            return 0;
        }

        let time_remaining = self.time - wait;
        time_remaining * wait
    }

    /// return a Vec of all the winning wait durations
    pub fn find_all_winning_times(&self) -> Vec<i64> {
        // don't include 0 or self.time
        (1..self.time)
            .filter(|wait| self.distance_travelled(*wait) > self.distance)
            .collect::<Vec<_>>()
    }
}

pub fn part_one(races: &[Race]) -> eyre::Result<i64> {
    Ok(races
        .iter()
        .map(|race| race.find_all_winning_times().len() as i64)
        .product())
}

pub fn part_two(race: &Race) -> eyre::Result<i64> {
    Ok(race.find_all_winning_times().len() as _)
}

fn parse_part_one(input: &str) -> eyre::Result<Vec<Race>> {
    let mut times = Vec::new();
    let mut distances = Vec::new();

    for line in input.lines() {
        match line.split_once(':').context("line didn't contain `:`")? {
            ("Time", line) => times.extend(
                line.split_whitespace()
                    .map(|n| Ok(n.parse::<i64>()?))
                    .collect::<eyre::Result<Vec<_>>>()?,
            ),
            ("Distance", line) => distances.extend(
                line.split_whitespace()
                    .map(|n| Ok(n.parse::<i64>()?))
                    .collect::<eyre::Result<Vec<_>>>()?,
            ),
            _ => eyre::bail!("malformed line"),
        }
    }

    Ok(times
        .into_iter()
        .zip(distances)
        .map(|(time, distance)| Race { time, distance })
        .collect::<Vec<_>>())
}

fn parse_part_two(input: &str) -> eyre::Result<Race> {
    let mut time = 0;
    let mut distance = 0;

    for line in input.lines() {
        match line.split_once(':').context("line didn't contain `:`")? {
            ("Time", line) => time = line.chars().filter(|c| c.is_numeric()).collect::<String>().parse()?,
            ("Distance", line) => distance = line.chars().filter(|c| c.is_numeric()).collect::<String>().parse()?,
            _ => eyre::bail!("malformed line"),
        }
    }

    Ok(Race { time, distance })
}
pub fn main() -> eyre::Result<()> {
    let input = crate::util::get_input(2023, 6)?;
    let races = parse_part_one(&input)?;
    let race = parse_part_two(&input)?;

    tracing::info!("Part One: {}", part_one(&races)?);
    tracing::info!("Part Two: {}", part_two(&race)?);

    Ok(())
}
