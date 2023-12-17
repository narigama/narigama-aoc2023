pub mod day01;
pub mod day02;
pub mod day03;
pub mod day04;
pub mod day05;

pub fn main() -> eyre::Result<()> {
    day01::main()?;
    day02::main()?;
    day03::main()?;
    day04::main()?;
    day05::main()?;

    Ok(())
}
