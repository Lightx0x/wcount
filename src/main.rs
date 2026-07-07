// use std::{env, process};
// use wcount::{Config, run};
use clap::Parser;

#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    file_path: String,
    /// lines count only
    #[arg(short, long)]
    lines: bool,
    /// words count only
    #[arg(short, long)]
    words: bool,
    /// characters count only
    #[arg(short, long)]
    chars: bool
}

fn main() {
let cli = Cli::parse();
println!("{cli:?}");
/*
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1)
    });

    if let Err(e) = run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
*/
}
