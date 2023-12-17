use std::{ops::RangeInclusive, str::FromStr};

#[derive(Debug, Clone)]
pub struct Number {
    pub value: u64,
    pub x: RangeInclusive<isize>,
    pub y: isize,
}

impl Number {
    pub fn surroundings(&self) -> Vec<(isize, isize)> {
        let mut points = Vec::new();

        // top/bottom/left/right
        points.extend((self.x.start() - 1..=self.x.end() + 1).map(|x| (x, self.y - 1)));
        points.extend((self.x.start() - 1..=self.x.end() + 1).map(|x| (x, self.y + 1)));
        points.push((self.x.start() - 1, self.y));
        points.push((self.x.end() + 1, self.y));

        points
    }
}

#[derive(Debug)]
pub struct Symbol {
    pub value: char,
    pub x: isize,
    pub y: isize,
}

impl Symbol {
    pub fn neighbours(&self, numbers: &[Number]) -> Vec<Number> {
        numbers
            .iter()
            .filter(|number| number.surroundings().iter().any(|(x, y)| x == &self.x && y == &self.y))
            .cloned()
            .collect()
    }
}

#[derive(Debug)]
pub struct Schematic {
    pub numbers: Vec<Number>,
    pub symbols: Vec<Symbol>,
}

fn build_number(digits: &mut Vec<(isize, char)>, numbers: &mut Vec<Number>, y: isize) -> Result<(), eyre::Error> {
    if !digits.is_empty() {
        let value = digits.iter().map(|(_, c)| c).collect::<String>().parse()?;
        let x_min = digits.iter().map(|(x, _)| *x).min().unwrap();
        let x_max = digits.iter().map(|(x, _)| *x).max().unwrap();

        numbers.push(Number {
            value,
            x: x_min..=x_max,
            y,
        });
        digits.clear();
    };
    Ok(())
}

impl FromStr for Schematic {
    type Err = eyre::Error;

    fn from_str(input: &str) -> eyre::Result<Self> {
        let mut digits = Vec::new();
        let mut numbers = Vec::new();
        let mut symbols = Vec::new();

        for (y, line) in input.lines().enumerate() {
            let y = y as isize;

            for (x, c) in line.char_indices() {
                let x = x as isize;

                // parse a number until you stop seeing digits
                if c.is_numeric() {
                    digits.push((x, c));
                } else {
                    build_number(&mut digits, &mut numbers, y)?;
                }

                // symbol
                if c != '.' && !c.is_numeric() {
                    symbols.push(Symbol { value: c, x, y })
                }
            }

            // make sure the buffer doesn't contain numbers at the end of a line too!
            build_number(&mut digits, &mut numbers, y)?;
        }

        Ok(Self { numbers, symbols })
    }
}

pub fn part_one(schematic: &Schematic) -> eyre::Result<u64> {
    let mut acc = 0;

    for symbol in schematic.symbols.iter() {
        for number in symbol.neighbours(&schematic.numbers) {
            acc += number.value;
        }
    }

    Ok(acc)
}

pub fn part_two(schematic: &Schematic) -> eyre::Result<u64> {
    let mut acc = 0;

    for symbol in schematic.symbols.iter().filter(|symbol| symbol.value == '*') {
        let numbers = symbol.neighbours(&schematic.numbers);

        if numbers.len() == 2 {
            acc += numbers.iter().map(|number| number.value).product::<u64>();
        }
    }

    Ok(acc)
}

pub fn main() -> eyre::Result<()> {
    let input = crate::util::get_input(2023, 3)?;
    let schematic = input.parse()?;

    tracing::info!("Part One: {}", part_one(&schematic)?);
    tracing::info!("Part Two: {}", part_two(&schematic)?);

    Ok(())
}
