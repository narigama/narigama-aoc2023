pub mod day01;
pub mod day02;

pub fn main() -> eyre::Result<()> {
    day01::main()?;
    day02::main()?;

    Ok(())
}
