use clap::Parser;
use rust_csv_parser;

#[derive(Parser)]
struct Cli {
    path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();

    let mut parser = match rust_csv_parser::CsvParser::from_path(&args.path) {
        Ok(p) => p,
        Err(err) => panic!("{err:?}"),
    };

    let mut parser_trim = match rust_csv_parser::CsvParser::from_path(&args.path) {
        Ok(p) => p.trim_space(true),
        Err(err) => panic!("{err:?}"),
    };

    let mut parser_whitespace_empty = match rust_csv_parser::CsvParser::from_path(&args.path) {
        Ok(p) => p.all_whitespace_empty(true),
        Err(err) => panic!("{err:?}"),
    };

    match parser.parse() {
        Ok(tokens) => println!("{tokens:?}"),
        Err(err) => panic!("{err:?}"),
    };
    println!();
    println!();
    match parser_trim.parse() {
        Ok(tokens) => println!("trimmed {tokens:?}"),
        Err(err) => panic!("{err:?}"),
    };
    println!();
    println!();
    match parser_whitespace_empty.parse() {
        Ok(tokens) => println!("whitespace {tokens:?}"),
        Err(err) => panic!("{err:?}"),
    };
}
