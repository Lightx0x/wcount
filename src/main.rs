use anyhow::Result;
use clap::Parser;
use weez::{Cli, run};

fn main() -> Result<()> {
    let cli = Cli::parse();

    run(cli)
}
