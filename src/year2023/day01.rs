use eyre::ContextCompat;

const NUMBERS: [(&str, u64); 9] = [
    ("one", 1),
    ("two", 2),
    ("three", 3),
    ("four", 4),
    ("five", 5),
    ("six", 6),
    ("seven", 7),
    ("eight", 8),
    ("nine", 9),
];

fn parse_numbers(line: &str) -> eyre::Result<Vec<u64>> {
    line.trim()
        .chars()
        .filter(|&c| c.is_numeric())
        .map(|c| Ok(c.to_string().parse()?))
        .collect::<eyre::Result<Vec<_>>>()
}

fn parse_numbers_and_words(line: &str) -> eyre::Result<Vec<u64>> {
    let line = line.trim();
    let mut results = Vec::new();

    for i in 0..line.chars().count() {
        let c = line
            .chars()
            .nth(i)
            .context(format!("{i} was out of bounds for {line}"))?;

        if c.is_numeric() {
            results.push(c.to_string().parse()?)
        } else {
            // if the substring starts with a word, insert it and move on
            for (word, value) in NUMBERS {
                if line[i..].starts_with(word) {
                    results.push(value);
                }
            }
        }
    }

    Ok(results)
}

fn solve(input: &str, line_to_numbers: impl Sync + Fn(&str) -> eyre::Result<Vec<u64>>) -> eyre::Result<u64> {
    Ok(input
        .lines()
        .map(|line| {
            let digits = line_to_numbers(line)?;

            let first = digits.first().context("line didn't contain digits")?;
            let last = digits.last().context("line didn't contain digits")?;
            Ok(first * 10 + last)
        })
        .collect::<eyre::Result<Vec<_>>>()?
        .into_iter()
        .sum())
}

pub fn part_one(input: &str) -> eyre::Result<u64> {
    solve(input, parse_numbers)
}

pub fn part_two(input: &str) -> eyre::Result<u64> {
    solve(input, parse_numbers_and_words)
}

pub fn main() -> eyre::Result<()> {
    let input = crate::util::get_input(2023, 1)?;

    tracing::info!("Part One: {}", part_one(&input)?);
    tracing::info!("Part Two: {}", part_two(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {
        let input = "
            1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet
        "
        .trim();

        let result = super::part_one(input).unwrap();
        assert_eq!(result, 142);
    }

    #[test]
    fn test_part_two() {
        let input = "
            two1nine
            eightwothree
            abcone2threexyz
            xtwone3four
            4nineeightseven2
            zoneight234
            7pqrstsixteen
        "
        .trim();

        let result = super::part_two(input).unwrap();
        assert_eq!(result, 281);
    }
}
