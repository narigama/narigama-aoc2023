use std::{ops::Range, str::FromStr};

use eyre::ContextCompat;
use itertools::Itertools;

pub type Point = (isize, isize);

#[derive(Debug)]
pub struct Map {
    pub source: i64,
    pub destination: i64,
    pub range: i64,
}

impl Map {
    pub fn source_range(&self) -> Range<i64> {
        self.source..self.source + self.range
    }

    pub fn destination_range(&self) -> Range<i64> {
        self.destination..self.destination + self.range
    }

    pub fn delta(&self) -> i64 {
        self.destination - self.source
    }

    /// if the value exists within the source range, shift it, else return the original value
    pub fn process_forwards(&self, value: i64) -> Option<i64> {
        self.source_range().contains(&value).then(|| value + self.delta())
    }

    pub fn process_backwards(&self, value: i64) -> Option<i64> {
        self.destination_range().contains(&value).then(|| value - self.delta())
    }
}

impl FromStr for Map {
    type Err = eyre::Error;

    fn from_str(line: &str) -> eyre::Result<Self> {
        let mut numbers = line.split_whitespace();

        // the ordering of these args is important as we're consuming an iterator _in order_.
        Ok(Self {
            destination: numbers.next().context("line was missing destination")?.parse()?,
            source: numbers.next().context("line was missing source")?.parse()?,
            range: numbers.next().context("line was missing range")?.parse()?,
        })
    }
}

#[derive(Debug)]
pub struct Almanac {
    pub seeds: Vec<i64>,

    pub seed_to_soil: Vec<Map>,
    pub soil_to_fertilizer: Vec<Map>,
    pub fertilizer_to_water: Vec<Map>,
    pub water_to_light: Vec<Map>,
    pub light_to_temp: Vec<Map>,
    pub temp_to_humidity: Vec<Map>,
    pub humidity_to_location: Vec<Map>,
}

impl Almanac {
    /// for each stage, try all the ranges, if one succeeds, use it to shift
    /// the value, otherwise leave the value alone and move on.
    pub fn process_forwards(&self, mut index: i64) -> i64 {
        let process_order = [
            &self.seed_to_soil,
            &self.soil_to_fertilizer,
            &self.fertilizer_to_water,
            &self.water_to_light,
            &self.light_to_temp,
            &self.temp_to_humidity,
            &self.humidity_to_location,
        ];

        // walk through each stage, appling it's maps in order
        // if one of them "hits", set the index to the new value and immediately break off
        for map in process_order {
            for m in map {
                if let Some(i) = m.process_forwards(index) {
                    index = i;
                    break;
                }
            }
        }

        index
    }

    pub fn process_backwards(&self, mut index: i64) -> i64 {
        let process_order = [
            &self.humidity_to_location,
            &self.temp_to_humidity,
            &self.light_to_temp,
            &self.water_to_light,
            &self.fertilizer_to_water,
            &self.soil_to_fertilizer,
            &self.seed_to_soil,
        ];

        // walk through each stage, appling it's maps in order
        // if one of them "hits", set the index to the new value and immediately break off
        for map in process_order {
            for m in map {
                if let Some(i) = m.process_backwards(index) {
                    index = i;
                    break;
                }
            }
        }

        index
    }
}

fn parse_body_to_map(body: &str) -> eyre::Result<Vec<Map>> {
    body.lines().map(|line| line.parse()).collect::<eyre::Result<Vec<_>>>()
}

impl FromStr for Almanac {
    type Err = eyre::Error;

    fn from_str(input: &str) -> eyre::Result<Self> {
        // parse the seeds
        let seeds = input
            .lines()
            .take(2)
            .collect::<String>()
            .trim()
            .chars()
            .skip(7)
            .collect::<String>()
            .split(' ')
            .map(|chunk| Ok(chunk.parse::<_>()?))
            .collect::<eyre::Result<Vec<_>>>()?;

        let mut seed_to_soil = Default::default();
        let mut soil_to_fertilizer = Default::default();
        let mut fertilizer_to_water = Default::default();
        let mut water_to_light = Default::default();
        let mut light_to_temp = Default::default();
        let mut temp_to_humidity = Default::default();
        let mut humidity_to_location = Default::default();

        // now parse each block, skip the seeds
        for block in input.split("\n\n").skip(1) {
            let (header, body) = block.split_once('\n').context("malformed map")?;

            match header.strip_suffix(" map:").context("malformed header")? {
                "seed-to-soil" => seed_to_soil = parse_body_to_map(body)?,
                "soil-to-fertilizer" => soil_to_fertilizer = parse_body_to_map(body)?,
                "fertilizer-to-water" => fertilizer_to_water = parse_body_to_map(body)?,
                "water-to-light" => water_to_light = parse_body_to_map(body)?,
                "light-to-temperature" => light_to_temp = parse_body_to_map(body)?,
                "temperature-to-humidity" => temp_to_humidity = parse_body_to_map(body)?,
                "humidity-to-location" => humidity_to_location = parse_body_to_map(body)?,
                kind => eyre::bail!("unknown block type: {kind}"),
            };
        }

        Ok(Self {
            seeds,
            seed_to_soil,
            soil_to_fertilizer,
            fertilizer_to_water,
            water_to_light,
            light_to_temp,
            temp_to_humidity,
            humidity_to_location,
        })
    }
}

pub fn part_one(almanac: &Almanac) -> eyre::Result<i64> {
    almanac
        .seeds
        .iter()
        .cloned()
        .map(|seed| almanac.process_forwards(seed))
        .min()
        .context("almanac contained no seeds")
}

pub fn part_two(almanac: &Almanac) -> eyre::Result<i64> {
    // build list of ranges
    let ranges = almanac
        .seeds
        .iter()
        .tuples()
        .map(|(start, range)| *start..*start + *range)
        .sorted_unstable_by_key(|range| range.start)
        .collect::<Vec<_>>();

    let mut index = 0;
    let jump = 1000;

    // first, keep incrementing in 10_000s until you find the first index that appears in the seed ranges
    loop {
        let seed = almanac.process_backwards(index);
        if ranges.iter().any(|range| range.contains(&seed)) {
            // ok subtract if we're above 0, and break out
            if index > 0 {
                index -= jump;
            }
            break;
        }

        // didn't find it? Jump forwards a large chunk of elements
        index += jump;
    }

    // now go forward and search in fine detail
    loop {
        let seed = almanac.process_backwards(index);
        if ranges.iter().any(|range| range.contains(&seed)) {
            break;
        }
        index += 1
    }

    Ok(index)
}

pub fn main() -> eyre::Result<()> {
    let input = crate::util::get_input(2023, 5)?;
    let almanac = input.parse::<Almanac>()?;

    tracing::info!("Part One: {}", part_one(&almanac)?);
    tracing::info!("Part Two: {}", part_two(&almanac)?);

    Ok(())
}
