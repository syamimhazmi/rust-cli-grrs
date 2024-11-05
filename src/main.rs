use std::{fs::File, io::{BufReader, Read}};
use clap::Parser;
use anyhow::{Context, Result};

#[derive(Parser)]
#[command(author="Syamim Hazmi", version, about="Search for patterns in file")]
struct Cli {
    #[arg(help="The pattern to search for in the file")]
    pattern: String,
    #[arg(help="The file path to search into")]
    path: std::path::PathBuf,
}

fn main() -> Result<()> {
    let args = Cli::parse();

    // open a file and create a buffer reader
    let file = File::open(&args.path).expect("no such file");
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut content = String::new();
    // read the buffer reader and write in into String
    reader.read_to_string(&mut content)
        .with_context(|| format!("could not read file `{}`", args.path.display()))?;

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("line: {}", line);
        }
    }

    Ok(())
}
