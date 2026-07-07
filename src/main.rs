use std::error::Error;
use clap::Parser;
use wcount::{Cli, run};

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    run(cli)
}
