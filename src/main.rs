use anyhow::Result;
use clap::Parser;
use std::fs::File;
use std::io::stdin;
use std::io::BufRead;
use std::io::BufReader;
use std::path::PathBuf;
use strip_tags::strip_tags;

#[cfg(feature = "cli")]
#[derive(Debug, Parser)]
#[command(version, about)]
struct Cli {
    /// Strip the tags from a given file. Reads from stdin if omitted
    file: Option<PathBuf>,
}

#[cfg(feature = "cli")]
fn buffered_strip(input: impl BufRead) -> Result<()> {
    for line in input.lines() {
        let line = line?;
        let stripped = strip_tags(&line);
        println!("{}", stripped);
    }
    Ok(())
}

#[cfg(feature = "cli")]
fn main() -> Result<()> {
    let args = Cli::parse();
    match args.file {
        Some(path) => {
            let f = File::open(path)?;
            let reader = BufReader::new(f);
            buffered_strip(reader)?;
        }
        None => {
            let stdin = stdin().lock();
            buffered_strip(stdin)?;
        }
    }
    Ok(())
}

