use std::{fs::File, io::{self, BufRead, BufReader}};
use clap::Parser;

#[derive(Parser)]
#[command(author="Syamim Hazmi", version, about="Search for patterns in file")]
struct Cli {
    #[arg(help="The pattern to search for in the file")]
    pattern: String,
    #[arg(help="The file path to search into")]
    path: std::path::PathBuf,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let file = File::open(&args.path)?;
    let reader = BufReader::new(file);

    for (_, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    Ok(())
}
