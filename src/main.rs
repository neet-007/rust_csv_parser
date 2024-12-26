use std::io;

use clap::Parser;
use rust_csv_parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,

    #[clap(short, long, default_value_t = false)]
    trimmer: bool,

    #[clap(short, long, default_value_t = false)]
    whitespace_empty: bool,
}

fn main() -> io::Result<()> {
    let args = Cli::parse();

    let mut parser = rust_csv_parser::CsvParser::from_path(&args.path)?
        .trim_space(args.trimmer)
        .all_whitespace_empty(args.whitespace_empty);

    let tokens = parser.parse()?;
    for token in tokens {
        println!("{token:?}");
    }
    Ok(())
}
