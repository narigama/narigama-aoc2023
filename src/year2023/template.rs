pub fn part_one(input: &str) -> eyre::Result<u64> {
    todo!()
}

pub fn part_two(input: &str) -> eyre::Result<u64> {
    todo!()
}

pub fn main() -> eyre::Result<()> {
    let input = crate::util::get_input(2023, 1)?;

    tracing::info!("Y2023D01P01: {}", part_one(&input)?);
    tracing::info!("Y2023D01P02: {}", part_two(&input)?);

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_part_one() {}

    #[test]
    fn test_part_two() {}
}
