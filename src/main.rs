use std::{fs::File, io::{BufReader, Read}};
use clap::Parser;
use anyhow::{Context, Result};
use log::{error, info};

#[derive(Parser)]
#[command(author="Syamim Hazmi", version, about="Search for patterns in file")]
struct Cli {
    #[arg(help="The pattern to search for in the file")]
    pattern: String,
    #[arg(help="The file path to search into")]
    path: std::path::PathBuf,
    #[command(flatten)]
    verbose: clap_verbosity_flag::Verbosity
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::parse();

    info!("starting up");

    // open a file and create a buffer reader
    let file = match File::open(&args.path) {
        Ok(f) => f,
        Err(e) => {
            error!("failed to open a file '{}': {}", args.path.display(), e);
            return  Err(e).with_context(
                || format!("failed to open file `{}`", args.path.display())            
            );
        }
    };
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut content: String = String::new();
    // read the buffer reader and write in into String
    match reader.read_to_string(&mut content) {
        Ok(_) => (),
        Err(e) => {
            error!("failed to read file '{}': {}", args.path.display(), e);
            return Err(e).with_context(
                || format!("could not read file `{}`", args.path.display())
            )
        }
    }

    for line in content.lines() {
        if line.contains(&args.pattern) {
            println!("line: {}", line);
        }
    }

    Ok(())
}
