
# Csv parser in rust

## Overview

this is my first attempt to write a parser in rust and my   
first project outside the rust book and advent of code it parses   
the csv according to this [rfc](https://www.ietf.org/rfc/rfc4180.txt).

## Flags
- path: path to the csv   
- trimmer: to make the parser trim all whitespace   
- whitespace_empty: to make the parser trim empty field that have only whitespace

## Examples

the program is a cli that follows this
```bash
    'cli name' --path 'path to the csv'
```

to use the program as a lib you can make a csv from a file or a string   
from file use:   
```rust
    let mut parser = rust_csv_parser::CsvParser::from_path(&args.path)?;
    let tokens = parser.parse()?;
```

from string use:
```rust
    let str = "exmaple str";
    let mut parser = rust_csv_parser::CsvParser::from_string(str.to_string());
    let tokens = parser.parse()?;
```
