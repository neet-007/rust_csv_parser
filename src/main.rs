use clap::Parser;
use rust_csv_parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let parser = match rust_csv_parser::CsvParser::new(&args.path) {
        Ok(p) => p,
        Err(err) => panic!("{err:?}"),
    };

    println!("{parser:?}");
}
