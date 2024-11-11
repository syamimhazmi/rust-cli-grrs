use std::{fs::File, io::{BufReader, Read}, process};
use anyhow::{Context, Result};
use cli_grrs::find_matches;
use log::{error, info};
use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(author="Syamim Hazmi", about="Search for patterns in file")]
#[allow(dead_code)]
struct Cli {
    #[structopt(help="The pattern to search for in the file")]
    pattern: String,
    #[structopt(help="The file path to search into", parse(from_os_str))]
    path: std::path::PathBuf,
    #[structopt(short)]
    verbose: bool
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Cli::from_args();

    if args.pattern.is_empty() {
        eprintln!("unexpected argument '' found");
        process::exit(1);
    }

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

    find_matches(&content, &args.pattern, &mut std::io::stdout())
        .context("failed to write output")?;

    Ok(())
}
