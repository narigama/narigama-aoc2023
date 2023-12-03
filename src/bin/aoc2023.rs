fn main() -> eyre::Result<()> {
    // load envvars and setup logging
    dotenvy::dotenv().ok();
    tracing_subscriber::fmt::init();
    
    // now boot
    narigama_aoc2023::main()
}