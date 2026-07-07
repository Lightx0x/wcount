use std::error::Error;
use std::fs;

/*
use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    file_path: Option<String>,
    #[arg(short, long)]
    /// -l --lines gets you the lines count only
    lines: bool,
    #[arg(short, long)]
    /// -w --words gets you the words count only
    words: bool,
    #[arg(short, long)]
    /// -c --char gets you the characters count only
    chars: bool
}

*/


#[derive(Debug)]
pub struct Config {
    mode: CountFlags,
    file_path: String,
}

#[derive(Debug)]
enum CountFlags {
    LinesOnly,
    WordOnly,
    CharOnly,
    All,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Config, &'static str> {
        args.next();

        let Some(first) = args.next() else {
            return Err("Expected file path");
        };

        let mode = if first.starts_with("-") {
            match first.as_str() {
                "-l" => CountFlags::LinesOnly,
                "-w" => CountFlags::WordOnly,
                "-c" => CountFlags::CharOnly,
                _ => return Err("Unknown flag"),
            }
        } else {
            CountFlags::All
        };

        let file_path = if first.starts_with("-") {
            let Some(path) = args.next() else {
                return Err("Flag found but no file");
            };
            path
        } else {
            first
        };

        Ok(Config { mode, file_path })
    }
}

#[derive(Debug, PartialEq)]
struct Counter {
    lines: usize,
    words: usize,
    chars: usize,
}

fn counts(text: &str) -> Counter {
    let lines = text.lines().count();
    let words = text.split_whitespace().count();
    let chars = text.chars().count();

    Counter {
        lines,
        words,
        chars,
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(&config.file_path).map_err(|err| {
        format!("{err} => {}", config.file_path)
    })?;

    let content_count = counts(&content);

    match config.mode {
        CountFlags::LinesOnly => println!("{} => lines: {}", config.file_path, content_count.lines),

        CountFlags::WordOnly => println!("{} => words: {}", config.file_path, content_count.words),

        CountFlags::CharOnly => println!("{} => chars: {}", config.file_path, content_count.chars),

        CountFlags::All => println!(
            "{} => lines: {}, words: {}, chars: {}",
            config.file_path, content_count.lines, content_count.words, content_count.chars
        ),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn empty_text_count() {
        let text = "";
        assert_eq!(counts(text), Counter{lines: 0, words: 0, chars: 0});
    }
    
    #[test]
    fn single_line_count() {
        let text = "hello world";
        assert_eq!(counts(text), Counter{lines: 1, words: 2, chars: 11});
    }

    #[test]
    fn multi_line_count() {
        let text = "a\nb\nc";
        assert_eq!(counts(text), Counter{lines: 3, words: 3, chars: 5});
    }
}

