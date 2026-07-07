use clap::Parser;
use std::error::Error;
use std::fs;

#[derive(Debug, Parser)]
#[command(version, about)]
pub struct Cli {
    file_path: String,
    /// lines count only
    #[arg(short, long)]
    lines: bool,
    /// words count only
    #[arg(short, long)]
    words: bool,
    /// characters count only
    #[arg(short, long)]
    chars: bool,
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

pub fn run(cli: Cli) -> Result<(), Box<dyn Error>> {
    let text_content =
        fs::read_to_string(&cli.file_path).map_err(|e| format!("{e} => {}", cli.file_path))?;
    let content_count = counts(&text_content);

    let show_all = !cli.lines && !cli.words && !cli.chars;
    if cli.lines || show_all {
        println!("lines: {}", content_count.lines);
    }

    if cli.words || show_all {
        println!("words: {}", content_count.words);
    }

    if cli.chars || show_all {
        println!("chars: {}", content_count.chars);
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_text_count() {
        let text = "";
        assert_eq!(
            counts(text),
            Counter {
                lines: 0,
                words: 0,
                chars: 0
            }
        );
    }

    #[test]
    fn single_line_count() {
        let text = "hello world";
        assert_eq!(
            counts(text),
            Counter {
                lines: 1,
                words: 2,
                chars: 11
            }
        );
    }

    #[test]
    fn multi_line_count() {
        let text = "a\nb\nc";
        assert_eq!(
            counts(text),
            Counter {
                lines: 3,
                words: 3,
                chars: 5
            }
        );
    }
}
