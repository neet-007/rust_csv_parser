/*
    grammer as per "RFC 4180", "https://www.ietf.org/rfc/rfc4180.txt"
    file = [header CRLF] record *(CRLF record) [CRLF]

    header = name *(COMMA name)

    record = field *(COMMA field)

    name = field

    field = (escaped / non-escaped)

    escaped = DQUOTE *(TEXTDATA / COMMA / CR / LF / 2DQUOTE) DQUOTE

    non-escaped = *TEXTDATA

    COMMA = %x2C

    CR = %x0D ;as per section 6.1 of RFC 2234 [2]
* */

use std::fs::File;
use std::io;
use std::path::PathBuf;

#[derive(Debug)]
pub struct CsvParser {
    file_path: PathBuf,
    reader: io::BufReader<File>,
}

impl CsvParser {
    pub fn new(file_path: &PathBuf) -> io::Result<CsvParser> {
        match file_path.extension() {
            Some(extension) if extension != "csv" => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "The file must have a .csv extension",
                ));
            }
            None => {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "The file must have an extension",
                ));
            }
            _ => {}
        }

        let f = File::open(file_path)?;
        let reader = io::BufReader::new(f);
        Ok(CsvParser {
            file_path: file_path.clone(),
            reader,
        })
    }
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
