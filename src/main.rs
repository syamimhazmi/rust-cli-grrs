use std::{fs::File, io::{self, BufRead, BufReader, Read}};
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

    // open a file and create a buffer reader
    let file = File::open(&args.path).expect("no such file");
    let mut reader: BufReader<File> = BufReader::new(file);
    let mut str = String::new();
    let result = reader.read_to_string(&mut str);
    match result {
        Ok(content) => { println!("File content: {}", content) },
        Err(error) => { println!("Oh noes: {}", error) }
    }

    for (_line_num, line) in reader.lines().enumerate() {
        let line = line?;
        if line.contains(&args.pattern) {
            println!("{}", line);
        }
    }

    println!("pattern: {:?}, path: {:?}", args.pattern, args.path);

    Ok(())
}
