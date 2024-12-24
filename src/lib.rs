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
#![allow(warnings)] // At the top of the file
use std::fs::File;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Debug, Clone)]
enum TokenType {
    Record,
    Eof,
}

#[derive(Debug, Clone)]
pub struct Token {
    token_type: TokenType,
    value: Vec<String>,
    line: u64,
}

#[derive(Debug)]
pub struct CsvParser<R: Read> {
    tokens: Vec<Token>,
    reader: io::BufReader<R>,
    record_field_count: u64,
    line: u64,
    has_header: bool,
    trim_space: bool,
    all_whitespace_empty: bool,
}

impl<R: Read> CsvParser<R> {
    pub fn new(reader: R) -> Self {
        CsvParser {
            tokens: Vec::<Token>::new(),
            reader: io::BufReader::new(reader),
            record_field_count: 0,
            line: 1,
            has_header: true,
            trim_space: false,
            all_whitespace_empty: false,
        }
    }

    pub fn with_header(mut self, flag: bool) -> Self {
        self.has_header = flag;
        self
    }

    pub fn trim_space(mut self, flag: bool) -> Self {
        self.trim_space = flag;
        self
    }

    pub fn all_whitespace_empty(mut self, flag: bool) -> Self {
        self.all_whitespace_empty = flag;
        self
    }

    fn next_char(&mut self) -> io::Result<Option<char>> {
        let mut buffer = [0u8; 1];

        match self.reader.read_exact(&mut buffer) {
            Ok(()) => {
                let byte = buffer[0];
                if byte <= 0x7F {
                    return Ok(Some(byte as char));
                }

                let mut utf8_buffer = vec![byte];
                while utf8_buffer.len() < 4 {
                    match self.reader.read_exact(&mut buffer) {
                        Ok(()) => {
                            utf8_buffer.push(buffer[0]);
                            if let Ok(decoded) = String::from_utf8(utf8_buffer.clone()) {
                                return Ok(Some(decoded.chars().next().unwrap()));
                            }
                        }
                        Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => {
                            return Ok(None);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Ok(None)
            }
            Err(ref e) if e.kind() == io::ErrorKind::UnexpectedEof => Ok(None),
            Err(e) => Err(e),
        }
    }

    fn parse_escaped(&mut self) {}

    fn parse_record(&mut self, first: char) -> io::Result<()> {
        let mut curr = String::new();
        curr.push(first);
        let mut record = Vec::<String>::new();
        loop {
            match self.next_char() {
                Ok(Some(c)) => match c {
                    ',' => {
                        if self.all_whitespace_empty && curr.trim().is_empty() {
                            record.push(String::from(""));
                        } else {
                            if self.trim_space {
                                record.push(curr.trim().to_owned().clone());
                            } else {
                                record.push(curr.clone());
                            }
                        }
                        curr.clear();
                    }
                    '\r' | '\n' => {
                        if self.all_whitespace_empty && curr.trim().is_empty() {
                            record.push(String::from(""));
                        } else {
                            if self.trim_space {
                                record.push(curr.trim().to_owned().clone());
                            } else {
                                record.push(curr.clone());
                            }
                        }
                        if self.line == 1 {
                            self.record_field_count = record.len() as u64;
                        } else if record.len() as u64 != self.record_field_count {
                            return Err(io::Error::new(
                                io::ErrorKind::InvalidData,
                                format!(
                                    "at line {:?} expect file to have {:?} fields found {:?}",
                                    self.line,
                                    self.record_field_count,
                                    record.len()
                                ),
                            ));
                        }
                        self.tokens.push(Token {
                            token_type: TokenType::Record,
                            value: record.clone(),
                            line: self.line,
                        });
                        self.line += 1;
                        return Ok(());
                    }
                    _ => curr.push(c),
                },
                Ok(None) => return Ok(()),
                Err(err) => return Err(err),
            };
        }
    }

    pub fn scan(&mut self) -> io::Result<Vec<Token>> {
        loop {
            match self.next_char() {
                Ok(Some(c)) => match c {
                    '"' => self.parse_escaped(),
                    _ => self.parse_record(c)?,
                },
                Ok(None) => return Ok(self.tokens.clone()),
                Err(err) => return Err(err),
            };
        }
    }

    pub fn parse(&mut self) -> io::Result<Vec<Token>> {
        return self.scan();
    }
}

impl CsvParser<File> {
    pub fn from_path(file_path: &PathBuf) -> io::Result<Self> {
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

        let file = File::open(file_path)?;
        Ok(Self::new(file))
    }
}

impl CsvParser<io::Cursor<String>> {
    pub fn from_string(input: String) -> Self {
        Self::new(io::Cursor::new(input))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parser_base() {
        let expect: Vec<Vec<String>> = vec![
            vec![String::from("1"), String::from("2"), String::from("3")],
            vec![String::from("a"), String::from("b"), String::from("c")],
        ];

        let str = expect.iter().fold(String::new(), |mut str, row| {
            str.push_str(
                row.iter()
                    .enumerate()
                    .fold(String::new(), |mut inner_str, (i, field)| {
                        inner_str.push_str(field.as_str());
                        if i < row.len() - 1 {
                            inner_str.push(',');
                        }
                        inner_str
                    })
                    .as_str(),
            );
            str.push('\r');
            str
        });
        let mut parser = CsvParser::from_string(str);

        let tokens = match parser.parse() {
            Ok(tokens) => tokens,
            Err(err) => panic!("{err:?}"),
        };

        for (token, exp) in tokens.iter().zip(expect) {
            assert_eq!(token.value, exp);
        }
    }
}
